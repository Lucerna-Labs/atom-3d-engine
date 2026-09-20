#!/usr/bin/env python3
"""Linux process-death acceptance at seven real resumable-job storage boundaries."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import shutil
import subprocess
import time
import traceback
from agent_animation_acceptance import Client, write_json

ROOT=Path(__file__).resolve().parents[1]
def sha(path):return hashlib.sha256(path.read_bytes()).hexdigest()
def main():
    p=argparse.ArgumentParser(description=__doc__);p.add_argument('--editor',type=Path,default=ROOT/'target/release/mm3e-editor');p.add_argument('--output',type=Path,required=True);a=p.parse_args()
    root=a.output.resolve();root.mkdir(parents=True,exist_ok=False);binary=root/'mm3e-editor';shutil.copyfile(a.editor.resolve(strict=True),binary);binary.chmod(0o755)
    report={'status':'running','binary_sha256':sha(binary),'cases':{},'scope':'Real local Linux process death and OS-lock recovery; not physical power-loss or hostile-filesystem certification.'};started=time.monotonic()
    try:
        shim=root/'render-job-fault.so'
        built=subprocess.run(['cc','-shared','-fPIC','-O2','-Wall','-Wextra','-Werror',str(ROOT/'scripts/render_job_fault_shim.c'),'-o',str(shim)],capture_output=True,text=True)
        (root/'shim-build.log').write_text(built.stdout+built.stderr);assert built.returncode==0,built.stderr
        shutil.copyfile(ROOT/'mm3e-editor/tests/fixtures/dialogue/hello-reference.wav',root/'voice.wav')
        with (root/'transcript.jsonl').open('w') as transcript:
            client=Client(binary,root,transcript,'author')
            try:
                client.call({'op':'import_audio','request':{'id':'voice','path':'voice.wav'}},True)
                client.apply([
                    {'op':'create','object':{'id':'ball','shape':{'type':'sphere','radius':.3},'position':[0,0,0]}},
                    {'op':'set_camera','camera':{'eye':[0,0,2],'target':[0,0,0],'fov_degrees':35}},
                    {'op':'set_settings','settings':{'width':16,'height':16,'quality':'preview','shadows':False,'ao':False}},
                    {'op':'put_clip','clip':{'id':'move','duration':1,'tracks':[{'target':{'type':'object','id':'ball'},'keys':[{'time':0},{'time':1,'translation':[.4,0,0]}]}]}},
                    {'op':'put_shot','shot':{'id':'line','clip':'move','rate':{'numerator':24000,'denominator':1001},'start_frame':1001,'clip_frame_zero':1001,'frame_count':12,'audio':{'asset':'voice','start_sample':317}}},
                ])
                selection={'start_frame':1002,'frame_count':5}
                client.call({'op':'render_shot','request':{'shot':'line','directory':'reference','selection':selection}})
                for mode in ['pending_before','pending_after','frame_before','frame_after','audio_after','manifest_after','complete_after']:
                    client.call({'op':'create_render_job','request':{'type':'shot','shot':'line','directory':mode,'selection':selection}})
                (root/'voice.wav').unlink()
            finally:client.close()
            for mode in ['pending_before','pending_after','frame_before','frame_after','audio_after','manifest_after','complete_after']:
                job=root/mode
                env=dict(os.environ,LD_PRELOAD=str(shim),MM3E_JOB_FAULT_ROOT=str(job),MM3E_JOB_FAULT_MODE=mode)
                req={'id':mode,'command':{'op':'step_render_job','directory':mode,'max_frames':32}}
                killed=subprocess.run([str(binary),'--root',str(root)],input=json.dumps(req)+'\n',text=True,capture_output=True,env=env,timeout=60)
                (job/'fault-stdout.log').write_text(killed.stdout);(job/'fault-stderr.log').write_text(killed.stderr)
                assert killed.returncode==86,(mode,killed.returncode,killed.stderr)
                trigger=json.loads((job/'fault-trigger.json').read_text());before=json.loads((job/'state.json').read_text())
                write_json(job/'state-before-recovery.json',before)
                preserved={f.name:(sha(f),f.stat().st_mtime_ns) for f in job.glob('frame_*.png')}
                staged={f.name:(sha(f),f.stat().st_mtime_ns) for f in (job/'.render-staging').glob('*.png')}
                if mode=='pending_before':assert before['pending'] is None and not preserved
                if mode in ['pending_after','frame_before','frame_after']:assert before['pending']['index']==0
                if mode=='frame_after':assert 'frame_0000.png' in preserved
                if mode=='audio_after':assert (job/'audio.wav').is_file() and not (job/'manifest.json').exists()
                if mode in ['manifest_after','complete_after']:assert (job/'manifest.json').is_file()
                if mode=='complete_after':assert before['completed_manifest'] is not None
                client=Client(binary,root,transcript,mode+'-recovery')
                try:
                    result=client.call({'op':'step_render_job','directory':mode,'max_frames':32})
                    assert result['status']=='complete' and result['completed_frames']==5
                    assert client.call({'op':'render_job_state','directory':mode,'verify_outputs':True})['outputs_verified'] is True
                    for name,identity in preserved.items():assert (sha(job/name),(job/name).stat().st_mtime_ns)==identity
                    for name,identity in staged.items():assert (sha(job/'.render-staging'/name),(job/'.render-staging'/name).stat().st_mtime_ns)==identity
                    for i in range(5):assert (job/f'frame_{i:04}.png').read_bytes()==(root/f'reference/frame_{i:04}.png').read_bytes()
                    assert (job/'audio.wav').read_bytes()==(root/'reference/audio.wav').read_bytes()
                    manifest=json.loads((job/'manifest.json').read_text());assert manifest['audio']['start_sample']==984 and manifest['audio']['sample_frames']==3337
                    after=json.loads((job/'state.json').read_text())
                    assert after['next_attempt']==(6 if mode=='pending_before' else 5),(mode,after['next_attempt'])
                    # Idempotent retry of the lost completion acknowledgement.
                    assert client.call({'op':'step_render_job','directory':mode})['status']=='complete'
                finally:client.close()
                report['cases'][mode]={'trigger':trigger,'committed_before':len(before['frames']),'existing_frames_reused':len(preserved),'next_attempt':after['next_attempt'],'image_and_audio_bytes_match_synchronous_export':True}
                write_json(root/'acceptance.json',report)
        report['status']='passed'
    except Exception as error:
        report.update(status='failed',error=str(error),traceback=traceback.format_exc());raise
    finally:
        report['elapsed_seconds']=time.monotonic()-started;write_json(root/'acceptance.json',report)
    print(json.dumps({'status':report['status'],'cases':len(report['cases'])},indent=2))
if __name__=='__main__':main()
