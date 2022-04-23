# Renamer written in Rust

## Project Outlines

The project outlines as described in my blog post about [Open Source Software Collaboration](https://blog.fox21.at/2019/02/21/open-source-software-collaboration.html).

- The main purpose of this software is to rename files to a specific format based on given configuration files in a folder and/or subfolders.
- This list is open. Feel free to request features.

## Examples

TODO

## Dev

```bash
./bin/dev.sh -c ./config.json -p tmp/test2 -l 1 -n
./bin/dev.sh -p tmp/test2
```

## Arguments

| Name | Description |
|---|---|
| `-h` or `--help` | Show help. |
| `-V` or `--version` | Show version. |
| `-c` or `--config` | Path to main config. |
| `-p` or `--path` | Path to root directory. Multiple `-p`s possible. At least one `--path` has to be provided. Otherwise renamer will not to anything. |
| `-l` or `--limit` | Limit the files to consider for renaming. |
| `-d` or `--maxdepth` | Maximum directory depth to consider. |
| `-n` or `--dryrun` | Do not change anything. Only print what would happen. |
| `-v` or `--verbose` | Verbose Levels: 1,2,3 |
| `-v` | Verbose Level 1 |
| `-vv` | Verbose Level 2 |
| `-vvv` | Verbose Level 3 |

For example

```bash
renamer --path /path/to/dir1
renamer --path /path/to/dir1 --path /path/to/dir2

renamer --config ./renamer.json --path /path/to/dir1 --path /path/to/dir2 --limit 10 --maxdepth 1 --verbose 3 --dryrun
```

## Config Data Structure

### Example

See [config.json](config.json) example file.

### Fields

| Name | Alias | Type | Description |
|---|---|---|---|
| is_root | root | bool | If true, renamer will not go up the directory tree for the next config file (`.renamer.json` or `renamer.json`). |
| verbose | - | int(u8) | 1,2,3 |
| name | - | string | - |
| exts | - | array | White-list for file extensions to consider for renaming. Ignore every other file. |
| vars | - | object | - |
| finds | - | object | - |

#### vars Field

#### finds Field
