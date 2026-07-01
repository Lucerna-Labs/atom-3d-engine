"""Evaluate a running llama-server (raw /completion) on the campaign dev/holdout
suites with the campaign graders. Used to test the answer-mode control vector on
the REAL Ollama GGUF (bare vs --control-vector-scaled), apples-to-apples with the
prompt-side results.

Start a llama-server first, e.g.:
  llama-server -m <blob> -ngl 99 --port 8081 -c 4096
  llama-server -m <blob> -ngl 99 --port 8081 -c 4096 --control-vector-scaled cv.gguf 5

Env: LS_URL (default http://127.0.0.1:8081), LS_SUITES (dev,holdout), LS_TAG (label),
     LS_NPREDICT (96).
"""
from __future__ import annotations
import os, sys, json, csv, time, urllib.request

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import run_general_corpus_campaign as camp  # noqa: E402

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
URL = os.environ.get("LS_URL", "http://127.0.0.1:8081").rstrip("/")
SUITES = [s for s in os.environ.get("LS_SUITES", "dev,holdout").split(",") if s.strip()]
TAG = os.environ.get("LS_TAG", "run")
NPREDICT = int(os.environ.get("LS_NPREDICT", "96"))


def complete(prompt: str) -> str:
    body = {
        "prompt": prompt,
        "n_predict": NPREDICT,
        "temperature": 0,
        "seed": 41,
        "stop": ["\n###", "\nTask:", "\n### Task"],
        "cache_prompt": False,
    }
    req = urllib.request.Request(URL + "/completion", data=json.dumps(body).encode("utf-8"),
                                headers={"Content-Type": "application/json"})
    with urllib.request.urlopen(req, timeout=600) as r:
        return json.loads(r.read().decode("utf-8")).get("content", "")


def clean_cut(cont: str) -> str:
    for marker in ("\n###", "\nTask:", "\n\n###"):
        i = cont.find(marker)
        if i != -1:
            cont = cont[:i]
    return camp.as_clean(cont)


def wait_ready(timeout=180):
    t0 = time.time()
    while time.time() - t0 < timeout:
        try:
            with urllib.request.urlopen(URL + "/health", timeout=5) as r:
                if json.loads(r.read().decode("utf-8")).get("status") == "ok":
                    return True
        except Exception:
            time.sleep(2)
    return False


def main():
    if not wait_ready():
        print("server not ready"); return 1
    rows, out = [], []
    for suite in SUITES:
        passed = 0
        for t in camp.SUITES[suite]:
            raw = complete(camp.build_prompt(t, ""))
            ans = clean_cut(raw)
            ok = bool(t.grader(ans)[0])
            passed += int(ok)
            if os.environ.get("LS_DEBUG"):
                print(f"    [{suite}/{t.task_id}] raw={raw[:70]!r} ans={ans[:40]!r} {'PASS' if ok else 'fail'}")
            rows.append({"tag": TAG, "suite": suite, "task_id": t.task_id, "passed": ok})
        n = len(camp.SUITES[suite])
        out.append((suite, passed, n))
        print(f"{TAG:<24} {suite:<8} {passed}/{n}")
    stamp = time.strftime("%Y%m%d-%H%M%S")
    rd = os.path.join(ROOT, "rag_runs", f"llamaserver-{TAG}-{stamp}")
    os.makedirs(rd, exist_ok=True)
    with open(os.path.join(rd, "trials.csv"), "w", newline="", encoding="utf-8") as fh:
        w = csv.DictWriter(fh, fieldnames=["tag", "suite", "task_id", "passed"]); w.writeheader(); w.writerows(rows)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
