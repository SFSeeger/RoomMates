#!/bin/bash
# Format Rust and Dioxus code, suitable for pre-commit hooks
set -e

CHANGED=0

# Run dx fmt and check if it would make changes
if ! dx fmt --check; then
	echo "dx fmt would make changes."
	CHANGED=1
fi

# Run cargo fmt in check mode
if ! cargo fmt -- --check; then
	echo "cargo fmt would make changes."
	CHANGED=1
fi

if [ "$CHANGED" -eq 1 ]; then
    cargo fmt --all
    dx fmt
	exit 1
else
	exit 0
fi
