#!/usr/bin/env bash

SCRIPT_BASEDIR=$(dirname "$0")

which cargo &> /dev/null || { echo 'ERROR: cargo not found in PATH'; exit 1; }

cd "${SCRIPT_BASEDIR}/.."
pwd

set -x
cargo publish --package renamer_app
cargo publish --package renamer_lib
