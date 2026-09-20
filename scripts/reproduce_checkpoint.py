#!/usr/bin/env python3
"""Rebuild and rerun the retained September 19 checkpoint without archived binaries.

Failures are preserved and produce a nonzero exit; known failures are not waived.
All output goes to a new directory. See artifacts/REPRODUCTION.md for dependencies.
"""
import argparse
import hashlib
import json
import os
from pathlib import Path
import subprocess
import sys
import time

ROOT = Path(__file__).resolve().parents[1]


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--output', type=Path, required=True)
    parser.add_argument('--editor', type=Path, help='Editor path (also the expected build output when building)')
    parser.add_argument('--skip-build', action='store_true')
    parser.add_argument('--skip-tests', action='store_true')
    parser.add_argument('--speech-backend', type=Path, help='Pinned Rhubarb executable; omission records speech as skipped')
    parser.add_argument('--reader-path', type=Path, default=ROOT / '.dependencies/usd-reader')
    parser.add_argument('--exr-reader-path', type=Path, default=ROOT / '.dependencies/exr-reader')
    args = parser.parse_args()
    manifest = json.loads((ROOT / 'artifacts/reproduction-manifest.json').read_text())
    for name, record in manifest['files'].items():
        data = (ROOT / name).read_bytes()
        if len(data) != record['bytes'] or hashlib.sha256(data).hexdigest() != record['sha256']:
            raise SystemExit('Retained fixture/report differs from its manifest: ' + name)
    output = args.output.resolve()
    output.mkdir(parents=True, exist_ok=False)
    target = Path(os.environ.get('CARGO_TARGET_DIR', ROOT / 'target'))
    if not target.is_absolute():
        target = ROOT / target
    editor = (args.editor or target / 'release' / ('mm3e-editor.exe' if os.name == 'nt' else 'mm3e-editor')).resolve()
    report = {'status': 'running', 'retained_files_verified': len(manifest['files']), 'runs': [], 'skipped': []}

    def save():
        (output / 'reproduction.json').write_text(json.dumps(report, indent=2) + '\n')

    def run(name, command):
        print('Running ' + name, flush=True)
        started = time.monotonic()
        with (output / (name + '.log')).open('x') as log:
            result = subprocess.run([str(part) for part in command], cwd=ROOT, stdout=log, stderr=subprocess.STDOUT)
        report['runs'].append({'name': name, 'command': [str(part) for part in command],
                               'exit_code': result.returncode, 'elapsed_seconds': time.monotonic() - started})
        save()
        print(name + ': exit ' + str(result.returncode), flush=True)
        return result.returncode

    try:
        if not args.skip_build:
            if run('build', ['cargo', 'build', '--locked', '--release', '-p', 'mm3e-editor']):
                report['status'] = 'build_failed'
                return 1
        else:
            report['skipped'].append('build')
        report['editor_sha256'] = hashlib.sha256(editor.read_bytes()).hexdigest()
        if not args.skip_tests:
            run('tests', ['cargo', 'test', '--locked', '-p', 'mm3e-kit', '-p', 'mm3e-orchestrator',
                          '-p', 'mm3e-editor', '--tests', '--no-fail-fast'])
        else:
            report['skipped'].append('tests')
        for name, flag in [('face', '--editor'), ('deformed_face', '--editor'),
                           ('animation_layer', '--editor'), ('sewing', '--executable')]:
            run(name, [sys.executable, ROOT / 'scripts' / ('agent_' + name + '_acceptance.py'),
                       flag, editor, '--output', output / name])
        if args.speech_backend:
            run('speech', [sys.executable, ROOT / 'scripts/agent_speech_acceptance.py', '--editor', editor,
                           '--backend', args.speech_backend.resolve(), '--output', output / 'speech'])
        else:
            report['skipped'].append('speech: provide --speech-backend to run local recognition')
        run('textured_usd', [sys.executable, ROOT / 'scripts/agent_textured_usd_acceptance.py',
                            '--binary', editor, '--output', output / 'textured_usd',
                            '--fixture-root', ROOT / 'artifacts/material-maps-normal-filter-20260907-opt-in',
                            '--challenge-root', ROOT / 'artifacts/textured-usd-acceptance-20260907-final',
                            '--reader-path', args.reader_path.resolve(), '--exr-reader-path', args.exr_reader_path.resolve()])
        failed = [entry['name'] for entry in report['runs'] if entry['exit_code'] != 0]
        report['failed'] = failed
        report['status'] = 'failed' if failed else ('incomplete' if report['skipped'] else 'passed')
        return 1 if failed else (2 if report['skipped'] else 0)
    except Exception as error:
        report['status'] = 'error'
        report['error'] = str(error)
        raise
    finally:
        save()


if __name__ == '__main__':
    raise SystemExit(main())
