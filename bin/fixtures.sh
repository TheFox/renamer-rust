#!/usr/bin/env bash

SCRIPT_BASEDIR=$(dirname "$0")

cd "${SCRIPT_BASEDIR}/.."

rm -r tmp/

# Dirs
mkdir -p tmp/test1
mkdir -p tmp/test2/test3
mkdir -p tmp/test3/test4
mkdir -p tmp/test4/test5/test6
mkdir -p tmp/test5/test6/test7
mkdir -p tmp/test6/.test7

# Files
touch tmp/test1/test1a.{txt,mkv}
touch tmp/test2/test2a.{txt,mkv}
touch tmp/test2/test2b.{txt,mkv}
touch tmp/test2/test3/test3a.{txt,mkv}
touch tmp/test2/test3/test3b.{txt,mkv}
touch tmp/test3/test4/test4.mkv

# Configs
echo '{
    "is_root": false
}' > tmp/test1/renamer.json

echo '{
    "is_root": true,
    "name": "hallo_%num%_%char%%ext%",
    "exts": ["mkv"],
    "vars": {
        "%num%": {
            "type": "int",
            "format": "N%02d"
        },
        "%char%": {
            "type": "str",
            "format": "C%s"
        }
    },
    "finds": {
        "test(\\d)(.)": ["%num%", "%char%"]
    }
}' > tmp/test2/renamer.json

echo '{
    "is_root": false,
    "name": "test3",
    "exts": ["mkv"]
}' > tmp/test3/renamer.json

echo '{
    "is_root": false,
    "name": "test4",
    "exts": ["mkv"]
}' > tmp/test4/renamer.json

echo '{
    "name": "test5",
    "exts": ["avi"]
}' > tmp/test4/test5/renamer.json
