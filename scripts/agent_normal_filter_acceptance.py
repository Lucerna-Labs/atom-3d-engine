#!/usr/bin/env python3
"""Normal minification through the JSONL editor, independently decoded HDR frames.

Requires the official OpenEXR Python reader. Source-population and stratified bilinear-field references
measure the approximation's limitations as well as its frontal-glint correction.
"""
import argparse
import copy
import hashlib
import json
import math
from pathlib import Path
import shutil
import sys
import time
import traceback
from agent_animation_acceptance import Client, write_json
from agent_material_map_acceptance import png

ROOT = Path(__file__).resolve().parents[1]

def unit(v):
    length = math.sqrt(sum(c*c for c in v))
    return [c/length for c in v]

def brdf(normal, light):
    """Independent double-precision GGX evaluation for black dielectric, V=+Z."""
    dot = lambda a,b: sum(x*y for x,y in zip(a,b))
    nol = max(dot(normal,light),0)
    if nol <= 0:
        return 0.0
    nov = max(normal[2],1e-4)
    half = unit([light[0],light[1],light[2]+1])
    noh = max(dot(normal,half),0)
    a2 = max(.04**2,1e-3)**2
    d = a2 / max(math.pi*(noh*noh*(a2-1)+1)**2,1e-12)
    visibility = .5 / max(nol*math.sqrt(nov*nov*(1-a2)+a2)+nov*math.sqrt(nol*nol*(1-a2)+a2),1e-5)
    fresnel = .08+.92*(1-half[2])**5
    return d*visibility*fresnel*nol

def run(executable, root, transcript, openexr):
    pixels = [[204,128,230,0] if (i//64+i%64)%2==0 else [51,127,230,255] for i in range(64*64)]
    source = png(pixels)
    (root/'detail.png').write_bytes(source)
    client = Client(executable,root,transcript,'normal-filter')
    binding = {'object':'panel','uv_set':'uv','normal':{'texture':'detail','variance_filter':True}}
    def bind(value):
        client.apply([{'op':'bind_texture','binding':value}])
    def sample(footprint=1.0):
        return client.call({'op':'material_state','request':{'points':[[0,0,.01]],'footprint_m':footprint}})['samples'][0]
    def render(name):
        client.call({'op':'render','path':name+'.exr'})
        with openexr.File(str(root/(name+'.exr')),separate_channels=True) as image:
            channels=image.channels()
            return {k:float(channels[k].pixels[0,0]) for k in ['R','G','B','A']}
    try:
        client.call({'op':'import_texture','request':{'id':'detail','path':'detail.png','color_space':'linear'}},True)
        client.apply([
            {'op':'create','object':{'id':'panel','shape':{'type':'surface','vertices':[[-1,-1,0],[1,-1,0],[0,1,0]],'triangles':[[0,1,2]],'thickness_m':.02},'material':{'albedo':[0,0,0],'roughness':.04,'specular':1}}},
            {'op':'project_uvs','request':{'id':'uv','object':'panel','origin':[-1,-1,0],'axis_u':[1,0,0],'axis_v':[0,1,0],'meters_per_uv':[1/32,1/32]}},
            {'op':'bind_texture','binding':binding},
            {'op':'set_camera','camera':{'eye':[0,0,3],'target':[0,0,0],'fov_degrees':40}},
            {'op':'set_settings','settings':{'width':1,'height':1,'quality':'full','spatial_aa':1,'shadows':False,'ao':False,'sky_ambient':[0,0,0],'film':{'transparent_background':True,'exr_data_channels':True}}},
            {'op':'set_lights','lights':[{'vector':[0,0,1],'color':[1,1,1],'directional':True,'radius':0}]},
        ])
        enabled=sample(); assert enabled['normal_variance']>.5 and enabled['roughness']>.8
        assert sample(0)['normal_variance']==0
        authored=client.call({'op':'get_document'})
        assert authored['texture_bindings'][0]['normal']['variance_filter'] is True
        off=copy.deepcopy(binding);off['normal']['variance_filter']=False
        client.apply([{'op':'bind_texture','binding':off}],dry_run=True)
        assert client.call({'op':'get_document'})==authored
        bind(off);disabled=sample();assert disabled['normal_variance']==0
        assert disabled['roughness']==enabled['unfiltered_roughness']
        for key in ['shading_normal','geometric_normal','field','uv']:
            assert disabled[key]==enabled[key],key
        client.call({'op':'undo'},True);assert sample()==enabled
        client.call({'op':'redo'},True);assert sample()==disabled
        client.call({'op':'save','path':'disabled.json'})
        bind(binding)
        normals=[unit([2*c/255-1 for c in p[:3]]) for p in pixels[:2]]
        # Pixel footprints cover many full periods. Integrate the authored level-zero
        # bilinear field over one period, separately from the texel-population model.
        raw=[[2*c/255-1 for c in p[:3]] for p in pixels[:2]]
        def field_samples(size):
            for y in range(size):
                for x in range(size):
                    fx,fy=(x+.5)/size,(y+.5)/size
                    weight=(1-fx)*(1-fy)+fx*fy
                    yield unit([raw[0][i]*weight+raw[1][i]*(1-weight) for i in range(3)])
        bilinear=list(field_samples(256))
        convergence=sum(brdf(n,[0,0,1]) for n in field_samples(128))/(128*128)
        peak=2*math.degrees(math.atan2(normals[0][0],normals[0][2]))
        angles=[-80,-peak,-60,-40,-20,0,20,40,60,peak,80]
        rows=[]
        for index,degrees in enumerate(angles):
            angle=math.radians(degrees);light=[math.sin(angle),0,math.cos(angle)]
            client.apply([{'op':'set_lights','lights':[{'vector':light,'color':[1,1,1],'directional':True,'radius':0}]}])
            bind(binding);filtered=render(f'{index:02}-filtered')
            bind(off);unfiltered=render(f'{index:02}-unfiltered')
            reference=sum(brdf(n,light) for n in normals)/2
            field_reference=sum(brdf(n,light) for n in bilinear)/len(bilinear)
            assert filtered['A']==unfiltered['A']==1
            assert all(math.isfinite(v) and v>=0 for v in filtered.values())
            rows.append({'degrees':degrees,'filtered':filtered['R'],'unfiltered':unfiltered['R'],'source_population_reference':reference,'bilinear_field_reference_256':field_reference,
                         'filtered_absolute_error':abs(filtered['R']-reference),'unfiltered_absolute_error':abs(unfiltered['R']-reference)})
        front=next(r for r in rows if r['degrees']==0)
        assert front['filtered_absolute_error']<.1
        assert abs(front['filtered']-front['bilinear_field_reference_256'])<abs(front['unfiltered']-front['bilinear_field_reference_256'])
        assert front['filtered_absolute_error']<front['unfiltered_absolute_error']*.001
        # Strength zero preserves rendering exactly, even with the flag enabled.
        flat=copy.deepcopy(binding);flat['normal']['strength']=0;bind(flat);a=render('flat-filtered')
        flat['normal']['variance_filter']=False;bind(flat);b=render('flat-control');assert a==b
        bind(binding);client.call({'op':'save','path':'filtered.json'});before=sample();original=render('before-reload')
        (root/'detail.png').unlink();client.close();client=Client(executable,root,transcript,'normal-filter-cold')
        client.call({'op':'load','path':'filtered.json'},True);assert sample()==before;assert render('after-reload')==original
        client.call({'op':'export_texture','request':{'id':'detail','path':'source-recovered.png'}})
        assert (root/'source-recovered.png').read_bytes()==source
        client.call({'op':'load','path':'disabled.json'},True);assert sample()==disabled
        assert 'variance_filter' not in client.call({'op':'get_document'})['texture_bindings'][0]['normal']
        legacy=copy.deepcopy(binding);legacy['normal'].pop('variance_filter');bind(legacy);assert sample()==disabled
        return {'bilinear_reference_samples_per_period':len(bilinear),'frontal_bilinear_reference_128':convergence,'light_samples':rows,'front':front,'resolved_sample':enabled,'cold_reload_exact':True,'source_recovered_exact':True,
                'dry_run_undo_redo_and_explicit_disable_preserved':True,'strength_zero_pixels_exact':True,
                'scope':'Isotropic slope-variance approximation; side peaks and bilinear-field energy remain inaccurate; the reference is fixed-view periodic-field quadrature, not a general shot reference. No exact GGX convolution or general temporal quality claim.'}
    finally:
        client.close()

def main():
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--editor',type=Path,default=ROOT/'target/release/mm3e-editor')
    parser.add_argument('--output',type=Path,required=True)
    parser.add_argument('--exr-reader-path',type=Path)
    args=parser.parse_args()
    if args.exr_reader_path:
        sys.path.insert(0,str(args.exr_reader_path.resolve(strict=True)))
    import OpenEXR
    root=args.output.resolve();root.mkdir(parents=True,exist_ok=False)
    binary=root/args.editor.name;shutil.copyfile(args.editor.resolve(strict=True),binary);binary.chmod(0o755)
    report={'status':'running','binary_sha256':hashlib.sha256(binary.read_bytes()).hexdigest(),'harness_sha256':hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),'independent_reader_version':OpenEXR.__version__}
    started=time.monotonic()
    try:
        with (root/'transcript.jsonl').open('w') as transcript:
            report['case']=run(binary,root,transcript,OpenEXR)
        report['status']='passed'
    except Exception as error:
        report.update(status='failed',error=str(error),traceback=traceback.format_exc());raise
    finally:
        report['elapsed_seconds']=time.monotonic()-started;write_json(root/'acceptance.json',report)
    print(json.dumps({'status':report['status'],'front':report['case']['front']},indent=2))

if __name__=='__main__':main()
