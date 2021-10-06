#!/usr/bin/env bash
set -eu


echo "Generating reference output file ($1)"

DIR=$(dirname "$1")

(cd "$DIR" && npx babel -o output.js input.js)