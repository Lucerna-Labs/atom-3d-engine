#!/usr/bin/env bash
# Launch the 4-hour cross-domain primitive recipe simulation, detached, streaming to a fresh run dir.
# Game-engine primitives (dot/project/normalize/scale + graphics/geometry) x the whole cross-domain
# store, recipe_len=8 (deeper than the prior len=7 run), preserving all previous runs.
set -euo pipefail
STORE="/home/jesse/primitive-store/primitves math"
BIN="$HOME/spiderweb-sim/target/release/spiderweb-infer-primitives"
TS=$(date +%Y%m%d-%H%M%S)
RUN="$HOME/sim-runs/game-xd-$TS"
mkdir -p "$RUN"
{
  echo "started_at=$(date -Is)"
  echo "run_dir=$RUN"
  echo "store=$STORE"
  echo "mode=store"
  echo "recipe_len=8"
  echo "top_n=50"
  echo "max_primitives=160"
  echo "max_recipes=999999999"
  echo "run_seconds=14400"
  echo "purpose=game-engine primitives x cross-domain primitives, 4-hour deep recipe search"
  echo "prior_best_len7=typed=187.21 [dot->project-vec->normalize-vec->scale-vec->frobenius-norm->spectral-norm->tensor-contract]"
} > "$RUN/manifest.txt"
cd "$HOME/spiderweb-sim"
nohup "$BIN" store "$STORE" 8 50 160 999999999 14400 > "$RUN/final.log" 2> "$RUN/hits.log" < /dev/null &
PID=$!
echo "$PID" > "$RUN/pid"
echo "RUN_DIR=$RUN"
echo "PID=$PID"
