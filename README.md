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
```

## Config Data Structure

### Example

See [config.json](config.json) example file.

### Fields

| Name | Alias | Type | Description |
|---|---|---|---|
| is_root | root | bool | If true, renamer will not go up the directory tree for the next config file (`.renamer.json` or `renamer.json`). |
| errors | - | bool | Show errors. |
| name | - | string | - |
| exts | - | array | White-list for file extensions to consider for renaming. Ignore every other file. |
| vars | - | object | - |
| finds | - | object | - |

#### vars Field

#### finds Field
