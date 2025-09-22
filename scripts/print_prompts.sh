#!/usr/bin/env bash
set -euo pipefail

# Run the prompt-printing tests and show only the printed prompt bodies
# Note: we use --nocapture so println! output is shown

cargo test -q --test generate_prompts_test -- --nocapture

