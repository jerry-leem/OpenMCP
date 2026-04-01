#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT_DIR"

log(){
  printf '\n==> %s\n' "$1"
}

log "cargo check"
cargo check

log "cargo test"
cargo test

log "cargo build --release"
cargo build --release

if [ "${OPENMCP_SMOKE_RUN_GUI:-0}" = "1" ]; then
  if command -v timeout >/dev/null 2>&1; then
    log "cargo run --release (5s smoke)"
    log_file="/tmp/openmcp-smoke-gui.log"
    if timeout 5s cargo run --release >"$log_file" 2>&1; then
      exit_code=0
    else
      exit_code=$?
      if [ "$exit_code" -ne 124 ]; then
        echo "GUI smoke run failed (exit code: $exit_code)."
        echo "log: $log_file"
        cat "$log_file"
        exit "$exit_code"
      fi
    fi
  else
    echo "timeout command is missing. Set OPENMCP_SMOKE_RUN_GUI=0 or install coreutils."
  fi
fi

log "OpenMCP smoke check complete"
