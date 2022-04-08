#!/usr/bin/env bash

SCRIPT_BASEDIR=$(dirname "$0")

cd "${SCRIPT_BASEDIR}/.."

mkdir -p tmp/test1
mkdir -p tmp/test2/test3

touch tmp/test1/test1a.{txt,mkv}

touch tmp/test2/test2a.{txt,mkv}
touch tmp/test2/test2b.{txt,mkv}

touch tmp/test2/test3/test3a.{txt,mkv}
touch tmp/test2/test3/test3b.{txt,mkv}

echo '{
    "is_root": false
}' > tmp/test1/renamer.json

echo '{
    "is_root": true
}' > tmp/test2/renamer.json
