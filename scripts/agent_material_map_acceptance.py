#!/usr/bin/env python3
"""Real native material maps on the existing animated face and sewn-cloth fixtures.

Checks data channels, normal-frame attachment, relighting, geometry invariance,
physical-cache preservation and cold reload. Calibration art is not a measured
skin/fabric model or a claim of film-production readiness.
"""
import argparse
import copy
import hashlib
import json
import math
from pathlib import Path
import shutil
import struct
import time
import traceback
import zlib
from agent_animation_acceptance import Client, write_json
from agent_texture_acceptance import pattern, run_case as texture_case, barycentric
from agent_sewing_acceptance import png_pixels, pixel_difference

ROOT=Path(__file__).resolve().parents[1]
SIZE=64
def sha(path):return hashlib.sha256(path.read_bytes()).hexdigest()
def sub(a,b):return [x-y for x,y in zip(a,b)]
def dot(a,b):return sum(x*y for x,y in zip(a,b))
def scale(a,s):return [x*s for x in a]
def cross(a,b):return [a[1]*b[2]-a[2]*b[1],a[2]*b[0]-a[0]*b[2],a[0]*b[1]-a[1]*b[0]]
def unit(v):return scale(v,1/math.sqrt(dot(v,v)))
def png(pixels):
    def chunk(name,b):return struct.pack('>I',len(b))+name+b+struct.pack('>I',zlib.crc32(name+b))
    raw=b''.join(b'\0'+bytes(sum(pixels[y*SIZE:(y+1)*SIZE],[])) for y in range(SIZE))
    return b'\x89PNG\r\n\x1a\n'+chunk(b'IHDR',struct.pack('>IIBBBBB',SIZE,SIZE,8,6,0,0,0))+chunk(b'IDAT',zlib.compress(raw))+chunk(b'IEND',b'')
def maps():
    out={k:[] for k in ('packed','detail','glow')}
    for y in range(SIZE):
        for x in range(SIZE):
            u,v=(x+.5)/SIZE,1-(y+.5)/SIZE
            n=unit([.4*math.sin(2*math.pi*6*u),.25*math.cos(2*math.pi*5*v),1])
            out['detail'].append([round((c*.5+.5)*255) for c in n]+[(x*13+y*17)%256])
            out['packed'].append([round(255*(.2+.65*(.5+.5*math.sin(2*math.pi*3*u))**2)),220 if x<12 else 0,100,(x*11+y*7)%256])
            out['glow'].append([255,80,20,255 if 27<=x<37 and 27<=y<37 else 0])
    return out
def sample_image(pixels,uv,repeat,as_color=False):
    u=uv[0]%1 if repeat else min(1,max(0,uv[0]));v=min(1,max(0,uv[1]));x,y=u*SIZE-.5,(1-v)*SIZE-.5
    ix,iy,fx,fy=math.floor(x),math.floor(y),x%1,y%1
    def pixel(x,y):
        x=x%SIZE if repeat else max(0,min(SIZE-1,x));y=max(0,min(SIZE-1,y));p=[c/255 for c in pixels[y*SIZE+x]]
        return [p[i]*p[3] for i in range(3)]+[p[3]] if as_color else p
    a,b,c,d=pixel(ix,iy),pixel(ix+1,iy),pixel(ix,iy+1),pixel(ix+1,iy+1)
    return [(a[i]*(1-fx)+b[i]*fx)*(1-fy)+(c[i]*(1-fx)+d[i]*fx)*fy for i in range(4)]
def expected_normal(vertices,uv,geometric,rgb,strength):
    e1,e2=sub(vertices[1],vertices[0]),sub(vertices[2],vertices[0]);du1,dv1=sub(uv[1],uv[0]);du2,dv2=sub(uv[2],uv[0]);det=du1*dv2-du2*dv1
    u=scale(sub(scale(e1,dv2),scale(e2,dv1)),1/det);v=scale(sub(scale(e2,du1),scale(e1,du2)),1/det)
    source,n=unit(cross(e1,e2)),unit(geometric);side=-1 if dot(n,source)<0 else 1;hand=-1 if dot(cross(source,u),v)<0 else 1
    projected=sub(u,scale(n,dot(u,n)))
    if math.sqrt(dot(projected,projected))>64*2.220446049250313e-16*math.sqrt(dot(u,u)):t=unit(scale(projected,side))
    else:t=unit(scale(cross(unit(scale(sub(v,scale(n,dot(v,n))),side)),n),hand*side))
    b=unit(scale(cross(n,t),hand*side));mapped=unit([(2*rgb[0]-1)*strength,(2*rgb[1]-1)*strength,2*rgb[2]-1])
    return unit([t[i]*mapped[0]+b[i]*mapped[1]+n[i]*mapped[2] for i in range(3)])
def lighting(direction):return [{'vector':direction,'color':[3,3,3],'directional':True,'radius':.06}]
def run(executable,root,transcript,kind,data):
    client=Client(executable,root,transcript,kind+'-maps')
    try:
        client.call({'op':'load','path':'textured.json'},True)
        original=client.call({'op':'get_document'});cache=copy.deepcopy(original['cloths'])
        for name,pixels in data.items():
            (root/(name+'.png')).write_bytes(png(pixels));client.call({'op':'import_texture','request':{'id':name,'path':name+'.png','color_space':'linear'}},True)
        bindings=[];operations=[]
        for old in original['texture_bindings']:
            b={'object':old['object'],'uv_set':old['uv_set'],'sampler':old['sampler'],
               'roughness':{'texture':'packed','channel':'r'},'metallic':{'texture':'packed','channel':'g'},
               'emissive':{'texture':'glow'},'normal':{'texture':'detail','strength':1}}
            bindings.append(b);operations.append({'op':'bind_texture','binding':b})
            obj=next(o for o in original['objects'] if o['id']==b['object']);m=copy.deepcopy(obj['material']);m.update(roughness=1,metallic=1,emissive=[3,1,.25])
            operations.append({'op':'update','id':obj['id'],'patch':{'material':m}})
        operations.append({'op':'set_lights','lights':lighting([.8,.2,1])});client.apply(operations)
        authored=client.call({'op':'get_document'});assert authored['cloths']==cache
        clip='expression' if kind=='face' else 'pose';times=[0,.4,.8] if kind=='face' else [0,.15,.3]
        normal_error=scalar_error=0;count=0;frames={}
        for pose_index,t in enumerate(times):
            animation={'clip':clip,'time':t}
            for b in bindings:
                obj=next(o for o in authored['objects'] if o['id']==b['object']);uv=next(s for s in authored['uv_sets'] if s['id']==b['uv_set']);repeat=b['sampler']['u']=='repeat'
                state=client.call({'op':'deformer_state' if kind=='face' else 'cloth_state','id':b['object'],'animation':animation});verts=state['vertices'];triangles=obj['shape']['triangles'];ids=list(range(0,len(triangles),max(1,len(triangles)//25)))
                points=[[sum(verts[triangles[i][k]][axis]*w for k,w in enumerate([.23,.31,.46])) for axis in range(3)] for i in ids]
                observed=client.call({'op':'material_state','request':{'points':points,'animation':animation}})['samples']
                matched=0
                for tri,s in zip(ids,observed):
                    if s['object']!=b['object']:continue
                    assert s['triangle']==tri
                    packed=sample_image(data['packed'],s['uv'],repeat);detail=sample_image(data['detail'],s['uv'],repeat);glow=sample_image(data['glow'],s['uv'],repeat,True)
                    scalar_error=max(scalar_error,abs(s['roughness']-max(.04,packed[0])),abs(s['metallic']-packed[1]))
                    scalar_error=max(scalar_error,max(abs(s['linear_emissive'][i]-glow[i]*[3,1,.25][i]) for i in range(3)))
                    n=expected_normal([verts[i] for i in triangles[tri]],[uv['values'][i] for i in uv['corner_indices'][tri]],s['geometric_normal'],detail[:3],1)
                    normal_error=max(normal_error,math.dist(n,s['shading_normal']));assert dot(s['geometric_normal'],s['shading_normal'])>=-1e-6
                    matched+=1;count+=1
                assert matched>=8,(kind,b['object'],matched)
            frames[str(t)]=client.call({'op':'render','path':f'mapped-{pose_index}.png','animation':animation})
        assert normal_error<5e-5,normal_error;assert scalar_error<3e-6,scalar_error
        differences=[];animation={'clip':clip,'time':times[1]}
        for name,direction in [('right',[.8,.2,1]),('left',[-.8,.2,1])]:
            client.apply([{'op':'set_lights','lights':lighting(direction)}]);full=client.call({'op':'render','path':name+'-normal.png','animation':animation})
            flat=[dict(b,normal=dict(b['normal'],strength=0)) for b in bindings];client.apply([{'op':'bind_texture','binding':b} for b in flat]);control=client.call({'op':'render','path':name+'-flat.png','animation':animation})
            assert full['metrics']['visible_material_owner_pixels']==control['metrics']['visible_material_owner_pixels']
            difference=pixel_difference(png_pixels(root/(name+'-normal.png'))[0],png_pixels(root/(name+'-flat.png'))[0]);assert difference>200,(kind,name,difference);differences.append(difference)
            client.call({'op':'undo'},True)
        assert pixel_difference(png_pixels(root/'right-normal.png')[0],png_pixels(root/'left-normal.png')[0])>200
        if kind=='cloth':assert client.call({'op':'cloth_state','id':'garment'})['cache']['fresh'] is True
        assert client.call({'op':'get_document'})['cloths']==cache
        before=client.call({'op':'get_document'});bad=copy.deepcopy(bindings[0]);bad['normal']['texture']='print'
        revision=client.revision;error=client.apply([{'op':'bind_texture','binding':bad}],expect_ok=False);assert 'linear' in error['error']['message'];assert client.revision==revision and client.call({'op':'get_document'})==before
        client.call({'op':'render','path':'material-hdr.exr','animation':animation})
        client.call({'op':'save','path':'material-project.json'});client.call({'op':'render','path':'final.png','animation':animation})
        for name in data:(root/(name+'.png')).unlink()
        client.close();client=Client(executable,root,transcript,kind+'-maps-cold');client.call({'op':'load','path':'material-project.json'},True);assert client.call({'op':'get_document'})==before
        client.call({'op':'render','path':'final-cold.png','animation':animation});assert (root/'final-cold.png').read_bytes()==(root/'final.png').read_bytes()
        for name,pixels in data.items():client.call({'op':'export_texture','request':{'id':name,'path':name+'-recovered.png'}});assert (root/(name+'-recovered.png')).read_bytes()==png(pixels)
        return {'probes':count,'maximum_normal_error':normal_error,'maximum_channel_error':scalar_error,'normal_vs_flat_changed_pixels':differences,'frames':frames,'cache_and_geometry_preserved':True,'cold_reload_exact':True}
    finally:client.close()
def main():
    parser=argparse.ArgumentParser(description=__doc__);parser.add_argument('--editor',type=Path,default=ROOT/'target/release/mm3e-editor');parser.add_argument('--output',type=Path,required=True);args=parser.parse_args()
    root=args.output.resolve();root.mkdir(parents=True,exist_ok=False);binary=root/('mm3e-editor.exe' if args.editor.suffix=='.exe' else 'mm3e-editor');shutil.copyfile(args.editor.resolve(strict=True),binary);binary.chmod(0o755)
    report={'status':'running','binary_sha256':sha(binary),'harness_sha256':sha(Path(__file__)),'cases':{},'scope':'Per-triangle normal frames, scalar/emission maps and authored relighting; no calibrated textile/skin, MikkTSpace, displacement or normal-variance filtering claim.'};started=time.monotonic()
    try:
        with (root/'transcript.jsonl').open('w') as transcript:
            encoded,pixels=pattern()
            for kind in ['face','cloth']:
                texture_case(binary,root/kind,transcript,kind,encoded,pixels)
                report['cases'][kind]=run(binary,root/kind,transcript,kind,maps());write_json(root/'acceptance.json',report)
        assert sha(binary)==report['binary_sha256'];report['status']='passed'
    except Exception as error:report.update(status='failed',error=str(error),traceback=traceback.format_exc());raise
    finally:report['elapsed_seconds']=time.monotonic()-started;write_json(root/'acceptance.json',report)
    print(json.dumps({'status':report['status'],'elapsed_seconds':report['elapsed_seconds']},indent=2))
if __name__=='__main__':main()
