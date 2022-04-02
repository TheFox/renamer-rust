
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
use crate::config::Config;
use crate::stats::Stats;

pub struct Renamer<'a> {
    config: &'a Config,
    limit: Limit,
    dryrun: bool,
}

impl Renamer {
    pub fn new(config: &Config, limit: Limit, dryrun: bool) -> Self {
        #[cfg(debug_assertions)]
        println!("-> Renamer::new()");

        Self {
            config: config,
            limit: limit,
            dryrun: dryrun,
        }
    }

    pub fn rename(&self, paths: Paths) -> Stats {
        if cfg!(debug_assertions) {
            println!("-> Renamer::rename({:?})", paths);
            dbg!(self.config);
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
                    println!("-> _config1: {:?} {}", _config1, _config1.exists());
                    println!("-> _config2: {:?} {}", _config2, _config2.exists());

                    let local_config: Option<Config> = if (&_config1).exists() {
                        Some(Config::from_path(_config1))
                    } else if (&_config2).exists() {
                        Some(Config::from_path(_config2))
                    }
                    else { None };

                    match read_dir(_ppath) {
                        Ok(_files) => {
                            'files_loop: for _file in _files {
                                match _file {
                                    Ok(_entry) => {
                                        println!("{}-> file: {:?} {:?}{}", BLUE, _entry, _entry.path().file_name(), NO_COLOR);
                                        match _entry.path().file_name() {
                                            Some(_file_name) => {
                                                if _file_name == "renamer.json" || _file_name == "renamer.json" {
                                                    println!("{}  -> skip{}", BLUE, NO_COLOR);
                                                    continue 'files_loop;
                                                }
                                            },
                                            None => {},
                                        }
                                        match _entry.metadata() {
                                            Ok(_metadata) => {
                                                // println!("-> metadata: {:?}", _metadata);
                                                println!("  -> mode: {:02o}", _metadata.mode());

                                                if _metadata.is_dir() {
                                                    println!("  -> is dir");
                                                    stats.dirs += 1;

                                                    let _spaths = Some(vec![_entry.path().display().to_string()]);

                                                    // let rest = Some(self.stats.rest);
                                                    let rest: Option<u8> = None;

                                                    // let _renamer = Self::new(None, rest, self.dryrun);
                                                    // let _sstats = _renamer.rename(_spaths);
                                                    // stats += _sstats;
                                                } else if _metadata.is_file() {
                                                    println!("  -> is file");
                                                    stats.files += 1;

                                                    match _entry.path().extension() {
                                                        Some(_ext) => {
                                                            // TODO: skip when extension is not in the whitelist
                                                        },
                                                        None => {},
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
                                                println!("{}-> No metadata available for {:?}: {}{}", RED, _entry, _error, NO_COLOR);
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
                            println!("{}-> read dir ERROR: {}: {}{}", RED, _error.description(), _path, NO_COLOR);
                            stats.errors += 1;
                        },
                    }
                }
            },
            None => {},
        }

        stats
    } // pub fn rename()

    pub fn print_stats(&self) {

    }
}

#[cfg(test)]
mod tests_renamer {
    use super::Renamer;

    #[test]
    fn test_renamer1() {
        assert!(true);
    }
}
