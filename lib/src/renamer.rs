
use std::fs::read_dir;
use std::fs::rename;
use std::fs::ReadDir;
use std::fs::DirEntry;
use std::path::Path;
use std::path::PathBuf;
use std::ffi::OsStr;
use std::error::Error;
use std::result::Result;
use std::fs::FileType;
use std::os::unix::fs::MetadataExt;
use log::debug;
// use std::convert::From;

use crate::types::ConfigPath;
use crate::types::Paths;
use crate::types::Limit;
use crate::types::FileCount;
use crate::types::MaxDepth;
use crate::colors::NO_COLOR;
use crate::colors::RED;
use crate::colors::BLUE;
use crate::colors::GREEN;
use crate::colors::YELLOW;
use crate::config::Config;
use crate::config::ConfigOption;
use crate::stats::Stats;

// extern crate renamer_app;
// use renamer_app::app::App;

#[cfg(debug_assertions)]
fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn read_local_config(path: &Path) -> ConfigOption {
    let _config1 = path.join("renamer.json");
    let _config2 = path.join(".renamer.json");

    if (&_config1).exists() {
        Some(Config::from_path_buf(_config1))
    } else if (&_config2).exists() {
        Some(Config::from_path_buf(_config2))
    }
    else {
        None
    }
}

fn find_root_path(path: &Path) -> &Path {
    // println!("{}-> find_root_path({:?}){}", RED, path, NO_COLOR);

    let mut paths: Vec<&Path> = vec![];

    let mut ancestors = path.ancestors();
    for _path in path.ancestors() {
        if _path.display().to_string().len() > 0 {
            paths.push(_path);
        } else {
            paths.push(&Path::new("."));
        }
    }

    let mut current_path: &Path = &Path::new(".");

    for _path in &paths {
        // println!("  {}-> path: {:?}{}", YELLOW, _path, NO_COLOR);

        match read_local_config(&_path) {
            Some(_config) => {
                // println!("    -> config: {:?} {:?} {:?}", _config.name(), _config.is_root(), _config.is_initialized());
                current_path = _path;
                if _config.is_root() {
                    break;
                }
            },
            None => {},
        }
    }

    // println!("-> current_path: {:?}", current_path);
    current_path
}

fn find_root_config(path: &Path, level: u64) -> ConfigOption {
    // println!("{}-> find_root_config({:?}, {}){}", RED, path, level, NO_COLOR);

    match read_local_config(&path) {
        Some(_config) => {
            // Found at the given path, loaded to _config.

            // println!("{}  -> config: {:?}{}", YELLOW, _config.name(), NO_COLOR);

            if _config.is_root() {
                // println!("{}  -> is root{}", YELLOW, NO_COLOR);
                Some(_config)
            } else {
                // Found config is not the root, go up.
                // println!("{}  -> no root{}", YELLOW, NO_COLOR);
                match path.parent() {
                    Some(parent) => {
                        // println!("{}  -> found parent{}", YELLOW, NO_COLOR);

                        match find_root_config(&parent, level + 1) {
                            Some(_config) => Some(_config),
                            None => Some(_config),
                        }
                    },
                    None => {
                        // println!("{}  -> no parent{}", YELLOW, NO_COLOR);
                        Some(_config)
                    },
                }
            }
        },
        None => {
            // There is no config at the given path, go up.
            match path.parent() {
                Some(parent) => find_root_config(&parent, level + 1),
                None => None,
            }
        },
    }
}

fn merge_child_configs(path: &Path) -> Config {
    println!("-> merge_child_configs({:?})", path);

    let mut paths: Vec<&Path> = vec![];

    let mut ancestors = path.ancestors();

    for _path in path.ancestors() {
        if _path.display().to_string().len() > 0 {
            paths.insert(0, _path);
        } else {
            paths.insert(0, &Path::new("."));
        }
    }

    let mut merged_config = Config::new();
    for _path in paths {
        // println!("-> path: {:?}", _path);

        match read_local_config(&_path) {
            Some(local_config) => {
                // println!("  -> merge config: {:?}", _path);
                merged_config = merged_config.merge(&local_config);
            },
            None => {},
        }
    }
    merged_config
}

pub struct Renamer {
    config: Config,
    limit: Limit,
    dryrun: bool,
    verbose: u8,
    max_depth: MaxDepth,
    level: u64,
}

// impl From<App> for Renamer {
//     fn from(app: App) -> Self {
//         Self {
//             config: Config,
//             limit: Limit,
//             dryrun: bool,
//             verbose: u8,
//             max_depth: MaxDepth,
//             level: u64,
//         }
//     }
// }

impl Renamer {
    pub fn new(config: Config, limit: Limit, max_depth: MaxDepth, dryrun: bool, verbose: u8) -> Self {
        #[cfg(debug_assertions)]
        println!("-> Renamer::new({:?}, {}, {}, {:?})", limit, dryrun, verbose, max_depth);

        Self {
            config: config,
            limit: limit,
            max_depth: max_depth,
            dryrun: dryrun,
            verbose: verbose,
            level: 0,
        }
    }

    // pub fn from_app(config: Config, app: &App) -> Self {
    //     #[cfg(debug_assertions)]
    //     println!("-> Renamer::from_app()");

    //     Self {
    //         config: config,
    //         limit: app.limit,
    //         max_depth: app.max_depth,
    //         dryrun: app.dryrun,
    //         verbose: app.verbose,
    //         level: 0,
    //     }
    // }

    fn traverse(config: Config, limit: Limit, max_depth: MaxDepth, dryrun: bool, verbose: u8, level: u64) -> Self {
        #[cfg(debug_assertions)]
        println!("-> Renamer::traverse({}, {:?})", level, max_depth);

        Self {
            config: config,
            limit: limit,
            max_depth: max_depth,
            dryrun: dryrun,
            verbose: verbose,
            level: level,
        }
    }

    fn get_merged_config(&self, path: &Path) -> Config {
        // println!("-> Renamer::get_merged_config({:?})", path);

        match read_local_config(path) {
            Some(_config) => {
                if _config.is_root() {
                    // println!("{}    -> take local config{}", BLUE, NO_COLOR);
                    _config
                } else {
                    // println!("{}    -> merge config{}", BLUE, NO_COLOR);
                    self.config.merge(&_config)
                }
            },
            None => {
                // println!("{}    -> clone config: {:?}{}", BLUE, self.config.is_initialized(), NO_COLOR);
                self.config.clone()
            },
        }
    }

    pub fn rename(&self, paths: Paths) -> Stats {
        if cfg!(debug_assertions) {
            println!("-> Renamer::rename(l={}, {:?}) [v={}]", self.level, paths, self.verbose);
        }

        let mut stats = Stats::new();
        if let Some(_limit) = self.limit {
            stats.rest = Some(_limit);
        }

        match paths {
            Some(_paths) => {
                'paths_loop: for _path in &_paths {
                    let _ppath = &String::from(_path);
                    let _ppath = Path::new(OsStr::new(_ppath));

                    if self.verbose >= 1 {
                        println!("{}-> path: {:?}{}", BLUE, _ppath, NO_COLOR);
                    }

                    let mut merged_config: Config = self.get_merged_config(&_ppath);
                    if !merged_config.is_initialized() {
                        // println!("  -> config is not initialized [A]");

                        let root_path = find_root_path(&_ppath);
                        merged_config = merge_child_configs(root_path);

                        if !merged_config.is_initialized() {
                            debug!("{:?}", merged_config);
                            panic!("No config provided");
                        }
                    }

                    //println!("{}  -> merged config: {:?}{}", YELLOW, merged_config.is_initialized(), NO_COLOR);

                    if self.verbose >= 3 {
                        println!("--- config ---");
                        // debug!("{:?}", &merged_config); // TODO
                        dbg!(&merged_config); // TODO
                        println!("--------------");
                    }

                    // Get files as array.
                    let files: ReadDir = match read_dir(_ppath) {
                        Ok(_files) => _files,
                        Err(_error) => {
                            if self.verbose >= 2 {
                                println!("{}-> Read Dir ERROR for {}: {}{}", RED, _ppath.display(), _error.to_string(), NO_COLOR);
                            }
                            stats.errors += 1;
                            continue 'paths_loop;
                        },
                    };

                    'files_loop: for _file in files {
                        // Dir Entry
                        let dir_entry: DirEntry = match _file {
                            Ok(_entry) => _entry,
                            Err(ref _error) => {
                                if self.verbose >= 2 {
                                    println!("{}-> File ERROR for {:?}: {}{}", RED, _file, _error.to_string(), NO_COLOR);
                                }
                                stats.errors += 1;
                                continue 'files_loop;
                            },
                        };

                        let path = dir_entry.path();

                        if self.verbose >= 2 {
                            println!("{}-> entry: {:?}{}", GREEN, path.display(), NO_COLOR);
                        }

                        // File Name
                        let file_name: String = match path.file_name() {
                            Some(_file_name) => {
                                if _file_name == "renamer.json" || _file_name == ".renamer.json" {
                                    //println!("{}-> skip, renamer config{}", BLUE, NO_COLOR);
                                    stats.skipped += 1;
                                    continue 'files_loop;
                                }

                                let _fn: String = _file_name.to_str().unwrap().into();
                                if _fn.as_bytes()[0] == 46 { // =='.'
                                    if self.verbose >= 2 {
                                        println!("{}-> skip, hidden entry{}", YELLOW, NO_COLOR);
                                    }
                                    stats.skipped += 1;
                                    continue 'files_loop;
                                }
                                _fn
                            },
                            None => {
                                if self.verbose >= 2 {
                                    println!("{}-> skip, cannot extract file name from path: {:?}{}", RED, dir_entry, NO_COLOR);
                                }
                                stats.errors += 1;
                                continue 'files_loop;
                            },
                        };

                        let entry_metadata = match dir_entry.metadata() {
                            Ok(_metadata) => _metadata,
                            Err(_error) => {
                                if self.verbose >= 2 {
                                    println!("{}-> no metadata available for {:?}: {}{}", RED, dir_entry, _error, NO_COLOR);
                                }
                                stats.errors += 1;
                                continue 'files_loop;
                            },
                        };

                        if entry_metadata.is_dir() {
                            if self.verbose >= 2 {
                                println!("-> is dir");
                            }
                            stats.dirs += 1;

                            let max_depth: MaxDepth = match self.max_depth {
                                Some(_md) => {
                                    let md = _md - 1;
                                    if md <= 0 {
                                        if self.verbose >= 2 {
                                            println!("{}-> skip directory, reached max depth{}", YELLOW, NO_COLOR);
                                        }
                                        // stats.skipped += 1; // wos moch ma do? keine Ahnung, Herr Kommissar.
                                        continue 'files_loop;
                                    }
                                    Some(md)
                                },
                                None => None,
                            };

                            let _spaths = Some(vec![path.display().to_string()]);

                            let _renamer = Self::traverse(merged_config.clone(), stats.rest, max_depth, self.dryrun, self.verbose, self.level + 1);
                            let _sstats = _renamer.rename(_spaths);
                            stats += _sstats;
                        } else if entry_metadata.is_file() {
                            if self.verbose >= 2 {
                                println!("-> is file");
                            }
                            stats.files += 1;

                            if !merged_config.has_name() {
                                if self.verbose >= 2 {
                                    println!("-> skip, config has no name");
                                }
                                stats.skipped += 1;
                                continue 'files_loop;
                            }

                            let ext: String = match path.extension() {
                                Some(_ext) => _ext.to_str().unwrap().into(),
                                None => {
                                    if self.verbose >= 1 {
                                        println!("{}-> skip, cannot extract extension from path: {}{}", RED, path.display(), NO_COLOR);
                                    }
                                    stats.errors += 1;
                                    continue 'files_loop;
                                },
                            };

                            if !merged_config.has_ext(&ext) {
                                if self.verbose >= 2 {
                                    println!("{}-> skip, wrong ext: '{}'{}", BLUE, &ext, NO_COLOR);
                                }
                                stats.skipped += 1;
                                continue 'files_loop;
                            }

                            let mut name = merged_config.name();
                            name = name.replace("%ext%", &(".".to_owned() + &ext));

                            if self.verbose >= 2 {
                                println!("-> check regex");
                            }
                            'finds_loop: for (regex, vars) in merged_config.regex_finds() {
                                // println!("  -> find: {:?}", regex);
                                match regex.captures(&file_name) {
                                    Some(caps) => {
                                        // println!("-> caps: {:?}", caps);

                                        let mut i = 1;
                                        for var in vars {
                                            let value = merged_config.format_var(var.clone(), caps[i].to_string());
                                            name = name.replace(&var, &value);

                                            // println!("  -> var: #{} {:?} => {} '{}'", i, var, &caps[i], value);
                                            // println!("  -> name C: '{}'", name);

                                            i += 1;
                                        }

                                        // Break loop after the first regex match.
                                        break 'finds_loop;
                                    },
                                    None => {
                                        // println!("{}  -> no match: {}{}", BLUE, regex, NO_COLOR);
                                    },
                                }
                            }

                            let parent = match path.parent() {
                                Some(_parent) => _parent,
                                None => panic!("Cannot get parent from path: {:?}", path),
                            };

                            let new_file_path = parent.join(name);

                            if new_file_path.exists() {
                                if self.verbose >= 2 {
                                    println!("{}-> skip file, already exists: {}{}", YELLOW, new_file_path.display(), NO_COLOR);
                                }
                                stats.warnings += 1;
                                continue 'files_loop;
                            }
                            if merged_config.path_has_var(&new_file_path) {
                                if self.verbose >= 2 {
                                    println!("{}-> skip file, variables not replaced{}", YELLOW, NO_COLOR);
                                }
                                stats.warnings += 1;
                                continue 'files_loop;
                            }

                            if self.verbose >= 1 {
                                if self.dryrun {
                                    println!("{}-> move (dry-run){}", GREEN, NO_COLOR);
                                }
                                else {
                                    println!("{}-> move{}", GREEN, NO_COLOR);
                                }
                                println!("{}   src: {}{}", GREEN, path.display(), NO_COLOR);
                                println!("{}  dest: {}{}", GREEN, new_file_path.display(), NO_COLOR);
                            }

                            if !self.dryrun {
                                if cfg!(feature="production") {
                                    // println!("{}   move{}", RED, NO_COLOR);
                                    rename(path, new_file_path);
                                }
                                stats += 1;
                            }
                        } // entry_metadata.is_file()

                        if stats.end() {
                            if self.verbose >= 1 {
                                println!("{}-> abort, reached limit{}", BLUE, NO_COLOR);
                            }
                            break 'paths_loop;
                        }
                    } // 'files_loop
                } // 'paths_loop
            }, // Some(_paths)
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
