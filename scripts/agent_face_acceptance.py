#!/usr/bin/env python3
"""Real-process facial authoring, geometry, rendered visibility and persistence acceptance.

The facial model is an analytic control foundation, not a claim of film-ready human anatomy.
Requests, responses, partial images and failures are preserved in a new output directory.
"""
import argparse
import copy
import json
import math
from pathlib import Path
import time
import traceback
from agent_animation_acceptance import Client, by_id, frame_path, make_playback, write_json


def main():
    parser=argparse.ArgumentParser(description=__doc__)
    workspace=Path(__file__).resolve().parents[1]
    parser.add_argument("--editor",type=Path,default=workspace/"target/release/mm3e-editor")
    parser.add_argument("--output",type=Path,required=True)
    args=parser.parse_args()
    root=args.output.resolve();root.mkdir(parents=True,exist_ok=False)
    executable=args.editor.resolve(strict=True)
    started=time.monotonic()
    result={"status":"running","kind":"actual JSONL process facial acceptance","executable":str(executable),
            "backend":"MM3E CPU","quality":"preview","limitations":"procedural SDF face; no facial skinning, anatomical jaw hinge, teeth/tongue, detailed anatomy or automatic speech alignment"}
    write_json(root/"acceptance.json",result)
    client=None
    try:
        with (root/"transcript.jsonl").open("w") as log:
            client=Client(executable,root,log,"author")
            client.apply([
                {"op":"create_humanoid","id":"hero","height":1.8,"build":1.0,"head_scale":1.1},
                {"op":"rig_humanoid","id":"hero"},
                {"op":"set_settings","settings":{"width":256,"height":320,"quality":"preview","shadows":False,"ao":False,"exposure":1}},
                {"op":"set_camera","camera":{"eye":[0,1.62,0.85],"target":[0,1.62,0.0],"fov_degrees":31}},
            ])
            body=client.call({"op":"get_document"})
            client.apply([{"op":"update","id":e["id"],"patch":{"material":{"albedo":[0.63,0.39,0.27],"roughness":0.7}}}
                          for e in body["objects"] if not e["id"].endswith("_eye")])
            source=client.call({"op":"get_document"})
            original_eyes=[e for e in source["objects"] if e["id"].endswith("_eye")]
            def track(channel,keys):
                return {"face":"hero/face","channel":channel,"easing":"smooth_step","keys":[{"time":t,"value":v} for t,v in keys]}
            clip={"id":"expressions","duration":2,"face_tracks":[
                track("blink_left",[(0,0),(.3,0),(.5,1),(.7,0),(1.5,0),(1.65,1),(1.8,0),(2,0)]),
                track("blink_right",[(0,0),(.3,0),(.5,1),(.7,0),(1.5,0),(1.65,1),(1.8,0),(2,0)]),
                track("jaw_open",[(0,0),(.6,0),(1,1),(1.4,.5),(1.8,.85),(2,0)]),
                track("lip_round",[(0,0),(1,0),(1.4,1),(1.8,.3),(2,0)]),
                track("lip_wide",[(0,0),(1,.35),(1.4,-.3),(2,0)]),
                track("smile",[(0,0),(1,.1),(1.8,.65),(2,0)]),
            ]}
            head_clip={"id":"head-turn","duration":1,"tracks":[{"target":{"type":"object","id":"hero/head"},"keys":[{"time":0},{"time":1,"rotation_degrees":[0,35,0],"translation":[0.02,0.01,0]}]}],
                       "face_tracks":[track("blink_left",[(0,0),(1,1)]),track("jaw_open",[(0,0),(1,1)])]}
            client.apply([{"op":"create_face","request":{"id":"hero/face","character":"hero"}},{"op":"put_clip","clip":clip},{"op":"put_clip","clip":head_clip}])
            document=client.call({"op":"get_document"})
            assert len(document["objects"])==26
            assert document["objects"][:19]==source["objects"],"facial creation changed source body/eyes"
            client.call({"op":"validate"})
            revision=client.revision
            client.apply([{"op":"set_face_controls","id":"hero/face","controls":{"blink_left":1.1}}],expect_ok=False)
            assert client.revision==revision and client.call({"op":"get_document"})==document
            client.apply([{"op":"update","id":"hero/face/upper_lip","patch":{"position":[9,9,9]}}],expect_ok=False)
            assert client.revision==revision and client.call({"op":"get_document"})==document
            client.apply([{"op":"set_face_controls","id":"hero/face","controls":{"jaw_open":0.7}}])
            changed=client.call({"op":"get_document"})
            client.call({"op":"undo"},True);assert client.call({"op":"get_document"})==document
            client.call({"op":"redo"},True);assert client.call({"op":"get_document"})==changed
            client.call({"op":"undo"},True)
            def animation(t,clip="expressions",playback="clamp"):
                return {"clip":clip,"time":t,"playback":playback}
            renders={}
            for name,t in [("neutral",0),("blink-closed",.5),("mouth-open",1),("rounded-lips",1.4),("return",2)]:
                renders[name]=client.call({"op":"render","path":f"{name}.png","animation":animation(t)})
            open_visibility=renders["neutral"]["metrics"]["visible_material_owner_pixels"]
            closed_visibility=renders["blink-closed"]["metrics"]["visible_material_owner_pixels"]
            for eye in ("hero/left_eye","hero/right_eye"):
                assert open_visibility.get(eye,0)>0,(eye,open_visibility)
                assert closed_visibility.get(eye,0)==0,(eye,closed_visibility)
            for name in ("neutral","blink-closed","return"):
                assert renders[name]["metrics"]["visible_material_owner_pixels"].get("hero/face/mouth_interior",0)==0, "closed mouth exposes phantom recessed interior"
            assert renders["mouth-open"]["metrics"]["visible_material_owner_pixels"].get("hero/face/mouth_interior",0)>0
            assert renders["neutral"]["rgba_fnv1a64"]==renders["return"]["rgba_fnv1a64"]
            head=by_id(source["objects"])["hero/head"]
            radii=head["shape"]["radii"]
            probe=[head["position"][0],head["position"][1]-.45*radii[1],head["position"][2]+.8*radii[2]]
            def sample(t,id="hero/head"):
                return client.call({"op":"sample","id":id,"points":[probe],"animation":animation(t)})["samples"][0]["value"]
            neutral_distance=sample(0);open_distance=sample(1);world_open_distance=sample(1,None)
            assert neutral_distance<0<open_distance and world_open_distance>0,(neutral_distance,open_distance,world_open_distance)
            client.apply([{"op":"create","object":{"id":"mouth-witness","position":probe,"shape":{"type":"sphere","radius":.003},"material":{"albedo":[1,0,0]}}}])
            assert sample(1)>0 and sample(1,None)<0,"local subtraction erased an independent witness solid"
            client.call({"op":"undo"},True)
            neutral_pose=client.call({"op":"pose","animation":animation(0,"head-turn")})
            moved_pose=client.call({"op":"pose","animation":animation(1,"head-turn")})
            at_rest=by_id(neutral_pose["objects"]);moved=by_id(moved_pose["objects"])
            for eye in ("hero/left_eye","hero/right_eye"):
                assert math.dist(at_rest[eye]["position"],moved[eye]["position"])>.01
            assert moved["hero/left_eye"]["position"]==moved["hero/face/left_upper_lid"]["position"]
            assert moved["hero/head"]["position"]==moved["hero/face/upper_lip"]["position"]
            turned=client.call({"op":"render","path":"head-turn.png","animation":animation(1,"head-turn")})
            state=client.call({"op":"face_state","animation":animation(1)})
            assert client.call({"op":"face_state","animation":animation(3,"expressions","loop")})["faces"]==state["faces"]
            assert [e for e in client.call({"op":"get_document"})["objects"] if e["id"].endswith("_eye")]==original_eyes
            sequence=client.call({"op":"render_sequence","request":{"directory":"frames","clip":"expressions","start":0,"end":2,"fps":12}})
            manifest=json.loads(frame_path(root,sequence["manifest_path"]).read_text())
            frames=manifest["frames"]
            assert len(frames)==25
            paths=[frame_path(root,f["path"]) for f in frames]
            assert frames[0]["rgba_fnv1a64"]==renders["neutral"]["rgba_fnv1a64"]
            assert frames[6]["rgba_fnv1a64"]==renders["blink-closed"]["rgba_fnv1a64"]
            assert frames[12]["rgba_fnv1a64"]==renders["mouth-open"]["rgba_fnv1a64"]
            client.call({"op":"save","path":"hero-facial.mm3e-agent.json"})
            client.apply([{"op":"delete_clip","id":"expressions"},{"op":"delete_clip","id":"head-turn"},{"op":"remove_face","id":"hero/face"}])
            assert client.call({"op":"get_document"})["objects"]==source["objects"],"removing facial authoring failed to restore exact source objects"
            client.call({"op":"undo"},True)
            assert client.call({"op":"get_document"})==document
            client.close();client=None
            client=Client(executable,root,log,"reopen")
            client.call({"op":"load","path":"hero-facial.mm3e-agent.json"},True)
            assert client.call({"op":"get_document"})==document
            assert client.call({"op":"pose","animation":animation(1,"head-turn")})==moved_pose
            reopened=client.call({"op":"render","path":"reopened-mouth.png","animation":animation(1)})
            assert reopened["rgba_fnv1a64"]==renders["mouth-open"]["rgba_fnv1a64"]
            assert (root/"reopened-mouth.png").read_bytes()==(root/"mouth-open.png").read_bytes()
            client.close();client=None
            playback=make_playback(paths[:-1],root/"facial-playback.png",12)
            result.update({"status":"passed","rendered_frame_count":len(frames),"renders":renders,"head_turn":turned,
                           "open_eye_pixels":{e:open_visibility.get(e,0) for e in ("hero/left_eye","hero/right_eye")},
                           "closed_eye_pixels":{e:closed_visibility.get(e,0) for e in ("hero/left_eye","hero/right_eye")},
                           "head_cavity_probe":{"point":probe,"neutral":neutral_distance,"open":open_distance,"world_open":world_open_distance},
                           "closed_mouth_has_no_phantom_interior_hits":True,"body_and_eye_data_unchanged":True,"witness_solid_survives":True,"undo_redo":True,"source_restored_after_remove":True,
                           "fresh_process_pose_identical":True,"fresh_process_pixels_identical":True,"playback":playback})
    except Exception as error:
        result.update({"status":"failed","error":str(error),"traceback":traceback.format_exc()})
        raise
    finally:
        if client is not None:client.abort()
        result["elapsed_seconds"]=time.monotonic()-started
        write_json(root/"acceptance.json",result)
    print(json.dumps({k:result[k] for k in ("status","elapsed_seconds","rendered_frame_count","open_eye_pixels","closed_eye_pixels")},indent=2))


if __name__=="__main__":main()
