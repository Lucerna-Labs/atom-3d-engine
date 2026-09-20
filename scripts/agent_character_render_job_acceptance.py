#!/usr/bin/env python3
"""Resume real face/cloth material scenes and compare to synchronous native renders.

Inputs are the accepted material-map fixtures, not an anatomical/artistic approval.
"""
import argparse
import hashlib
import json
from pathlib import Path
import shutil
import time
import traceback
from agent_animation_acceptance import Client, write_json
from agent_sewing_acceptance import png_pixels

ROOT=Path(__file__).resolve().parents[1]
def sha(path):return hashlib.sha256(path.read_bytes()).hexdigest()
def main():
    p=argparse.ArgumentParser(description=__doc__);p.add_argument('--editor',type=Path,default=ROOT/'target/release/mm3e-editor');p.add_argument('--face-project',type=Path,required=True);p.add_argument('--cloth-project',type=Path,required=True);p.add_argument('--output',type=Path,required=True);a=p.parse_args()
    root=a.output.resolve();root.mkdir(parents=True,exist_ok=False);binary=root/a.editor.name;shutil.copyfile(a.editor.resolve(strict=True),binary);binary.chmod(0o755)
    report={'status':'running','binary_sha256':sha(binary),'harness_sha256':sha(Path(__file__)),'cases':{},'scope':'Existing continuous-face and sewn-cloth material fixtures; exact render and frozen physical data recovery, not final-shot quality approval.'};started=time.monotonic()
    try:
        with (root/'transcript.jsonl').open('w') as transcript:
            for kind,path,clip,end,fps in [('face',a.face_project,'expression',.8,2.5),('cloth',a.cloth_project,'pose',.3,20/3)]:
                case=root/kind;case.mkdir();shutil.copyfile(path.resolve(strict=True),case/'input.json')
                input_sha=sha(case/'input.json');client=Client(binary,case,transcript,kind+'-author')
                try:
                    client.call({'op':'load','path':'input.json'},True)
                    before=client.call({'op':'get_document'})
                    client.call({'op':'save','path':'authored.json'})
                    native_bytes=(case/'authored.json').read_bytes()
                    native_document=json.loads(native_bytes)['document']
                    spec={'directory':'job','clip':clip,'start':0,'end':end,'fps':fps,'format':'png'}
                    client.call({'op':'render_sequence','request':dict(spec,directory='reference')})
                    created=client.call({'op':'create_render_job','request':dict(spec,type='sequence')})
                    assert created['total_frames']==3
                    assert (case/'job/snapshot.json').read_bytes()==native_bytes
                    step=client.call({'op':'step_render_job','directory':'job'});assert step['completed_frames']==1
                    first=case/'job/frame_0000.png';prefix=(sha(first),first.stat().st_mtime_ns)
                    client.apply([{'op':'set_camera','camera':{'eye':[4,3,-4],'target':[0,1,0],'fov_degrees':60}}])
                    assert client.call({'op':'get_document'})['camera']!=before['camera']
                finally:client.close()
                (case/'input.json').unlink()
                client=Client(binary,case,transcript,kind+'-cold')
                try:
                    result=client.call({'op':'step_render_job','directory':'job','max_frames':2})
                    assert result['status']=='complete' and result['completed_frames']==3
                    assert (sha(first),first.stat().st_mtime_ns)==prefix
                    assert client.call({'op':'render_job_state','directory':'job','verify_outputs':True})['outputs_verified']
                    frames=[]
                    for i in range(3):
                        job=case/f'job/frame_{i:04}.png';reference=case/f'reference/frame_{i:04}.png'
                        assert job.read_bytes()==reference.read_bytes()
                        pixels,_=png_pixels(job);assert pixels
                        frames.append({'sha256':sha(job),'decoded_pixel_count':len(pixels)})
                    snapshot=json.loads((case/'job/snapshot.json').read_text())['document']
                    assert snapshot.get('cloths',[])==native_document.get('cloths',[]) and snapshot.get('deformers',[])==native_document.get('deformers',[])
                    report['cases'][kind]={'input_sha256':input_sha,'snapshot_sha256':result['snapshot_sha256'],'frames':frames,'native_physical_data_unchanged':True,'cold_resume_exact':True}
                finally:client.close()
                write_json(root/'acceptance.json',report)
        report['status']='passed'
    except Exception as error:
        report.update(status='failed',error=str(error),traceback=traceback.format_exc());raise
    finally:
        report['elapsed_seconds']=time.monotonic()-started;write_json(root/'acceptance.json',report)
    print(json.dumps({'status':report['status'],'cases':list(report['cases'])},indent=2))
if __name__=='__main__':main()
