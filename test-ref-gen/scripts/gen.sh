#!/usr/bin/env bash
set -eu

# Usage: gen.sh path/to/input/dir
echo "Generating reference output file for directory ($1)"


find . -name input.js -exec scripts/gen-file.sh {} \;