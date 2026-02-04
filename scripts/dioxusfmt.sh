#!/usr/bin/env bash
set -euo pipefail

/usr/local/cargo/bin/rustfmt --edition=2024 \
  | /usr/local/cargo/bin/dx fmt -f -
