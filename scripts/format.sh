#!/usr/bin/env bash
set -e

CHANGED=0

if ! dx fmt --check; then
	echo "dx fmt would make changes."
	CHANGED=1
fi

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
