#!/usr/bin/env bash

SCRIPT_BASEDIR=$(dirname "$0")
export APP_BUILD_AT=$(date)

which cargo &> /dev/null || { echo 'ERROR: cargo not found in PATH'; exit 1; }
which strip &> /dev/null || { echo 'ERROR: strip not found in PATH'; exit 1; }

cd "${SCRIPT_BASEDIR}/.."

set -e
cargo build --release
ls -la target/release/renamer

strip target/release/renamer
ls -la target/release/renamer
