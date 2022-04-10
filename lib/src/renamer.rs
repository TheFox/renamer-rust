
use std::fs::read_dir;
use std::fs::DirEntry;
use std::path::Path;
use std::path::PathBuf;
use std::ffi::OsStr;
use std::error::Error;
use std::result::Result;
// use std::io::prelude::*;
use std::fs::FileType;
use std::os::unix::fs::MetadataExt;

use crate::types::ConfigPath;
use crate::types::Paths;
use crate::types::Limit;
use crate::types::FileCount;
use crate::colors::NO_COLOR;
use crate::colors::RED;
use crate::colors::BLUE;
use crate::colors::GREEN;
use crate::colors::YELLOW;
use crate::config::Config;
use crate::stats::Stats;

pub struct Renamer {
    config: Config,
    limit: Limit,
    dryrun: bool,
    level: u64,
}

impl Renamer {
    pub fn new(config: Config, limit: Limit, dryrun: bool) -> Self {
        #[cfg(debug_assertions)]
        println!("-> Renamer::new()");

        // dbg!(&config);

        Self {
            config: config,
            limit: limit,
            dryrun: dryrun,
            level: 0,
        }
    }

    #[cfg(feature="test1")]
    pub fn test1(&self) {
        println!("Renamer::test1");
        panic!("ok");
    }

    fn traverse(config: Config, limit: Limit, dryrun: bool, level: u64) -> Self {
        #[cfg(debug_assertions)]
        println!("-> Renamer::traverse({})", level);

        // dbg!(&config);

        Self {
            config: config,
            limit: limit,
            dryrun: dryrun,
            level: level,
        }
    }

    pub fn rename(&self, paths: Paths) -> Stats {
        if cfg!(debug_assertions) {
            println!("-> Renamer::rename(l={}, {:?})", self.level, paths);
            // dbg!(self.config);
        }

        let mut stats = Stats::new();
        if let Some(_limit) = self.limit {
            stats.rest = Some(_limit);
        }

        println!("-> paths: {:?}", paths);

        match paths {
            Some(_paths) => {
                'paths_loop: for _path in &_paths {
                    let _ppath = &String::from(_path);
                    let _ppath = Path::new(OsStr::new(_ppath));

                    let _config1 = _ppath.join("renamer.json");
                    let _config2 = _ppath.join(".renamer.json");

                    println!("-> path: {:?}", _ppath);
                    println!("  -> config1: {:?} {}", _config1, _config1.exists());
                    println!("  -> config2: {:?} {}", _config2, _config2.exists());

                    let local_config: Option<Config> = if (&_config1).exists() {
                        println!("  -> read config1");
                        Some(Config::from_path_buf(_config1))
                    } else if (&_config2).exists() {
                        println!("  -> read config2");
                        Some(Config::from_path_buf(_config2))
                    }
                    else { None };

                    let merged_config: Config = match local_config {
                        Some(_config) => {
                            if _config.is_root() {
                                println!("{}  -> take local config{}", BLUE, NO_COLOR);
                                _config
                            } else {
                                println!("{}  -> merge config{}", BLUE, NO_COLOR);
                                self.config.merge(&_config)
                            }
                        },
                        None => {
                            println!("{}  -> clone config: {:?}{}", BLUE, self.config, NO_COLOR);
                            self.config.clone()
                        },
                    };

                    println!("{}  -> merged config: {:?}{}", YELLOW, merged_config, NO_COLOR);

                    match read_dir(_ppath) {
                        Ok(_files) => {
                            'files_loop: for _file in _files {
                                match _file {
                                    Ok(_entry) => {
                                        println!("{}-> file: {:?} {:?}{}", GREEN, _entry, _entry.path().file_name(), NO_COLOR);

                                        let file_name: String;
                                        match _entry.path().file_name() {
                                            Some(_file_name) => {
                                                if _file_name == "renamer.json" || _file_name == ".renamer.json" {
                                                    println!("{}  -> skip, renamer config{}", BLUE, NO_COLOR);
                                                    continue 'files_loop;
                                                }
                                                file_name = _file_name.to_str().unwrap().into();
                                            },
                                            None => {
                                                println!("{}  -> skip, cannot extract file name from path{}", BLUE, NO_COLOR);
                                                continue 'files_loop
                                            },
                                        }

                                        println!("-> file name: '{}'", file_name);

                                        match _entry.metadata() {
                                            Ok(_metadata) => {
                                                // println!("-> metadata: {:?}", _metadata);
                                                println!("  -> mode: {:02o}", _metadata.mode());

                                                if _metadata.is_dir() {
                                                    println!("  -> is dir");
                                                    stats.dirs += 1;

                                                    let _spaths = Some(vec![_entry.path().display().to_string()]);

                                                    let _renamer = Self::traverse(merged_config.clone(), stats.rest, self.dryrun, self.level + 1);
                                                    let _sstats = _renamer.rename(_spaths);
                                                    stats += _sstats;
                                                } else if _metadata.is_file() {
                                                    println!("  -> is file");
                                                    stats.files += 1;

                                                    let ext: String;
                                                    match _entry.path().extension() {
                                                        Some(_ext) => {
                                                            ext = _ext.to_str().unwrap().to_string();
                                                            if !merged_config.has_ext(&ext) {
                                                                println!("  -> not contains ext: {:?}", _ext);
                                                                println!("{}  -> skip{}", BLUE, NO_COLOR);
                                                                continue 'files_loop;
                                                            }
                                                        },
                                                        None => {
                                                            println!("{}  -> skip, cannot extract extension from path{}", BLUE, NO_COLOR);
                                                            continue 'files_loop;
                                                        },
                                                    }

                                                    let mut name = merged_config.name();
                                                    println!("  -> name A: '{}'", name);
                                                    name.replace("%ext%", &ext);
                                                    println!("  -> name B: '{}'", name);

                                                    println!("  -> check regex");
                                                    'finds_loop: for (regex, vars) in merged_config.regex_finds() {
                                                        println!("  -> find: {:?}", regex);
                                                        match regex.captures(&file_name) {
                                                            Some(caps) => {
                                                                // println!("  -> caps: {:?}", caps);
                                                                let mut i = 1;
                                                                for var in vars {
                                                                    let value = merged_config.format_var(var.clone(), caps[i].to_string());
                                                                    name.replace(&var, &value);
                                                                    println!("  -> name C: '{}'", name);
                                                                    println!("  -> var: #{} {:?} => {} '{}'",
                                                                        i, var, &caps[i], value);
                                                                    i += 1;
                                                                }

                                                                // Break loop after the first regex match.
                                                                break 'finds_loop;
                                                            },
                                                            None => {
                                                                println!("{}  -> no match: {}{}", BLUE, regex, NO_COLOR);
                                                            },
                                                        }
                                                    }

                                                    // TODO: rename file here
                                                    stats += 1;
                                                }

                                                if stats.end() {
                                                    println!("{}-> abort, reached limit{}", BLUE, NO_COLOR);
                                                    break 'paths_loop;
                                                }
                                            },
                                            Err(_error) => {
                                                println!("{}-> no metadata available for {:?}: {}{}", RED, _entry, _error, NO_COLOR);
                                                stats.errors += 1;
                                            },
                                        }
                                    },
                                    Err(_error) => {
                                        println!("{}-> File ERROR: {}{}", RED, _error, NO_COLOR);
                                        stats.errors += 1;
                                    },
                                }
                            }
                        },
                        Err(_error) => {
                            println!("{}-> read dir ERROR: {}: {}{}", RED, _error.to_string(), _path, NO_COLOR);
                            stats.errors += 1;
                        },
                    }
                }
            },
            None => {},
        }

        stats
    } // pub fn rename()
}

#[cfg(test)]
mod tests_renamer {
    use super::Renamer;

    #[test]
    fn test_renamer1() {
        assert!(true);
    }
}
