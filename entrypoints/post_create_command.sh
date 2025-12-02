#!/bin/bash
pre-commit install
cp -n .env.dist .env
echo "All set! You can now run 'dx serve --package frontend' to start the development server."
