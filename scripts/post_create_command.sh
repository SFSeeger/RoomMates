#!/bin/bash
pre-commit install
cp -n .env.dist .env
make dependencies
echo "All set! You can now run 'make dev-server' to start the development server."
