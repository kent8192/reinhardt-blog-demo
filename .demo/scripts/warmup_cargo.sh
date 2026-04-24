#!/usr/bin/env bash
# Pre-warm the cargo build cache for `manage` binary so that Scene 5's
# `cargo run --bin manage -- makemigrations` finishes within Demo Time's
# 5s shell-integration wait window.
#
# Demo Time's executeTerminalCommand waits for a 633 escape-sequence
# completion event, but hard-caps at 5s (see
# apps/vscode-extension/src/services/TerminalService.ts::waitForTerminalExecuted).
# First cargo run triggers a 60–90s workspace compile on a cold cache,
# which blows past that cap. Warming here keeps the demo on-rails
# without needing a 60s waitForTimeout.
set -u

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(cd "${SCRIPT_DIR}/../../fullstack_demo" 2>/dev/null && pwd)" || {
  echo "warmup-skipped-no-project"
  exit 0
}

LOG="/tmp/reinhardt-demo-cargo-warmup.log"

# Detach the build so executeScript returns immediately.
(
  cd "${PROJECT_DIR}"
  nohup cargo build --bin manage >"${LOG}" 2>&1 </dev/null &
  disown
) >/dev/null 2>&1

echo "warmup-started"
