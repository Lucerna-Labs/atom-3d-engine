#!/usr/bin/env python3
"""Exercise real JSON-line editor garment creation, rendering, fit and cold reopen."""
import argparse
import hashlib
import json
import subprocess
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--binary', default='target/release/mm3e-editor')
    parser.add_argument('--output', default='artifacts/agent-garments-2026-09-06')
    args = parser.parse_args()
    output = ROOT / args.output
    output.mkdir(parents=True, exist_ok=False)
    (output / 'summary.json').write_text(json.dumps({'status': 'incomplete', 'scope': 'Acceptance has not completed; inspect transcript and process output.'}, indent=2) + '\n')
    log = []
    counter = 0

    class Session:
        def __init__(self):
            self.process = subprocess.Popen([str(ROOT / args.binary), '--root', str(ROOT)], stdin=subprocess.PIPE,
                                            stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)
            self.revision = 0

        def command(self, command, okay=True):
            nonlocal counter
            counter += 1
            request = {'id': f'garment-{counter}', 'expected_revision': self.revision, 'command': command}
            self.process.stdin.write(json.dumps(request) + '\n')
            self.process.stdin.flush()
            line = self.process.stdout.readline()
            if not line:
                raise RuntimeError(self.process.stderr.read())
            response = json.loads(line)
            log.append({'request': request, 'response': response})
            (output / 'transcript.json').write_text(json.dumps(log, indent=2) + '\n')
            assert response['ok'] == okay, response
            self.revision = response['revision']
            return response.get('result')

        def close(self):
            self.process.stdin.close()
            assert self.process.wait(timeout=15) == 0, self.process.stderr.read()

    session = Session()
    operations = [
        {'op': 'create_humanoid', 'id': 'hero', 'height': 1.8, 'build': 1.0, 'head_scale': 1.0},
        {'op': 'set_settings', 'settings': {'width': 192, 'height': 256, 'quality': 'preview'}},
        {'op': 'set_camera', 'camera': {'eye': [2.6, 1.75, 4.8], 'target': [0, 0.93, 0], 'fov_degrees': 26}},
        {'op': 'create_garment', 'request': {'id': 'hero/shirt', 'character': 'hero', 'style': 'short_sleeve_top',
          'clearance_m': 0.012, 'thickness_m': 0.004, 'material': {'albedo': [0.68, 0.08, 0.045], 'roughness': 0.85}}},
        {'op': 'create_garment', 'request': {'id': 'hero/trousers', 'character': 'hero', 'style': 'trousers',
          'clearance_m': 0.005, 'thickness_m': 0.004, 'material': {'albedo': [0.035, 0.075, 0.22], 'roughness': 0.90}}},
        {'op': 'rig_humanoid', 'id': 'hero'},
        {'op': 'create_face', 'request': {'id': 'hero/face', 'character': 'hero'}},
        {'op': 'put_clip', 'clip': {'id': 'dressed_motion', 'duration': 2, 'tracks': [
            {'target': {'type': 'joint', 'id': 'hero/left_shoulder'}, 'keys': [
                {'time': 0}, {'time': 1, 'rotation_degrees': [0, 0, 90]}, {'time': 2, 'rotation_degrees': [0, 0, 45]}]},
            {'target': {'type': 'joint', 'id': 'hero/left_elbow'}, 'keys': [
                {'time': 0}, {'time': 1, 'rotation_degrees': [0, 0, 30]}, {'time': 2}]},
            {'target': {'type': 'joint', 'id': 'hero/right_hip'}, 'keys': [
                {'time': 0}, {'time': 1, 'rotation_degrees': [-20, 0, 0]}, {'time': 2}]},
        ], 'face_tracks': [
            {'face': 'hero/face', 'channel': 'jaw_open', 'keys': [{'time': 0, 'value': 0}, {'time': 1, 'value': 0.7}, {'time': 2, 'value': 0}]},
            {'face': 'hero/face', 'channel': 'blink_left', 'keys': [{'time': 0, 'value': 0}, {'time': 1, 'value': 1}, {'time': 2, 'value': 0}]},
            {'face': 'hero/face', 'channel': 'blink_right', 'keys': [{'time': 0, 'value': 0}, {'time': 1, 'value': 1}, {'time': 2, 'value': 0}]},
        ]}},
    ]
    session.command({'op': 'apply', 'operations': operations})
    initial = session.command({'op': 'get_document'})
    original_request = next(op['request'] for op in operations if op.get('op') == 'create_garment' and op['request']['id'] == 'hero/shirt')
    updated_request = {**original_request, 'thickness_m': 0.006, 'material': {'albedo': [0.08, 0.50, 0.12], 'roughness': 0.85}}
    session.command({'op': 'apply', 'operations': [{'op': 'update_garment', 'request': updated_request}]})
    updated = session.command({'op': 'get_document'})
    assert [e['id'] for e in updated['objects']] == [e['id'] for e in initial['objects']]
    actual_albedo = next(e for e in updated['objects'] if e['id'] == 'hero/shirt')['material']['albedo']
    assert max(abs(a - b) for a, b in zip(actual_albedo, updated_request['material']['albedo'])) < 1e-6
    measurement = session.command({'op': 'sample', 'id': 'hero/shirt', 'points': [[0, 1.062, 0.1266]]})
    assert abs(measurement['samples'][0]['value'] + 0.003) < 1e-5, measurement
    session.command({'op': 'undo'})
    assert session.command({'op': 'get_document'}) == initial
    session.command({'op': 'redo'})
    assert session.command({'op': 'get_document'}) == updated
    session.command({'op': 'apply', 'operations': [{'op': 'update_garment', 'request': original_request}]})
    fits = []
    for time in [None, 0.5, 1.0, 2.0]:
        for garment in ['hero/shirt', 'hero/trousers']:
            command = {'op': 'garment_fit', 'id': garment, 'samples_per_source': 64}
            if time is not None:
                command['animation'] = {'clip': 'dressed_motion', 'time': time}
            result = session.command(command)
            assert result['surface_samples'] > 0, result
            assert result['sampled_fit_pass'], result
            fits.append(result)
    print(f'Fit: {sum(fit["surface_samples"] for fit in fits)} surface probes across 8 garment/pose checks', flush=True)
    animation = {'clip': 'dressed_motion', 'time': 1.0}
    renders = []
    renders.append(session.command({'op': 'render', 'path': str(Path(args.output) / 'rest.png')}))
    print(f'Rest rendered: {renders[-1]["render_ms"]:.0f} ms', flush=True)
    renders.append(session.command({'op': 'render', 'path': str(Path(args.output) / 'motion.png'), 'animation': animation}))
    print(f'Motion rendered: {renders[-1]["render_ms"]:.0f} ms', flush=True)
    for render in renders:
        owners = render['metrics']['visible_material_owner_pixels']
        assert owners.get('hero/shirt', 0) > 0, owners
        assert owners.get('hero/trousers', 0) > 0, owners
    before = session.command({'op': 'get_document'})
    previous_revision = session.revision
    session.command({'op': 'apply', 'operations': [{'op': 'update', 'id': 'hero/shirt', 'patch': {'position': [1, 0, 0]}}]}, okay=False)
    assert session.revision == previous_revision
    assert session.command({'op': 'get_document'}) == before
    session.command({'op': 'save', 'path': str(Path(args.output) / 'dressed-character.mm3e-agent.json')})
    pose = session.command({'op': 'pose', 'animation': animation})
    face = session.command({'op': 'face_state', 'animation': animation})
    session.close()

    reopened = Session()
    reopened.command({'op': 'load', 'path': str(Path(args.output) / 'dressed-character.mm3e-agent.json')})
    assert reopened.command({'op': 'get_document'}) == before
    assert reopened.command({'op': 'pose', 'animation': animation}) == pose
    assert reopened.command({'op': 'face_state', 'animation': animation}) == face
    cold = reopened.command({'op': 'render', 'path': str(Path(args.output) / 'motion-reopened.png'), 'animation': animation})
    assert cold['rgba_fnv1a64'] == renders[1]['rgba_fnv1a64'], (cold, renders[1])
    assert (output / 'motion.png').read_bytes() == (output / 'motion-reopened.png').read_bytes()
    reopened.close()
    summary = {'status': 'passed', 'binary': str(ROOT / args.binary),
               'surface_samples': sum(fit['surface_samples'] for fit in fits),
               'garment_pose_checks': len(fits), 'fits': fits, 'renders': renders,
               'cold_reopen_rgba_hash': cold['rgba_fnv1a64'],
               'png_sha256': hashlib.sha256((output / 'motion.png').read_bytes()).hexdigest(),
               'scope': 'Continuous fitted SDF garments; finite scalar-field fit probes. No fabric simulation or inter-garment collision certification.'}
    (output / 'summary.json').write_text(json.dumps(summary, indent=2) + '\n')
    print(json.dumps({key: summary[key] for key in ['status', 'surface_samples', 'garment_pose_checks', 'cold_reopen_rgba_hash']}, indent=2))


if __name__ == '__main__':
    main()
