
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

#[cfg(debug_assertions)]
fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn read_local_config(path: &Path) -> Option<Config> {
    let _config1 = path.join("renamer.json");
    let _config2 = path.join(".renamer.json");

    println!("  -> config1: {:?} {}", _config1, _config1.exists());
    println!("  -> config2: {:?} {}", _config2, _config2.exists());

    if (&_config1).exists() {
        println!("  -> read config1");
        Some(Config::from_path_buf(_config1))
    } else if (&_config2).exists() {
        println!("  -> read config2");
        Some(Config::from_path_buf(_config2))
    }
    else {
        None
    }
}

pub struct Renamer {
    config: Config,
    limit: Limit,
    dryrun: bool,
    level: u64,
}

impl Renamer {
    pub fn new(config: Config, limit: Limit, dryrun: bool) -> Self {
        #[cfg(debug_assertions)]
        println!("-> Renamer::new({:?}, {})", limit, dryrun);

        Self {
            config: config,
            limit: limit,
            dryrun: dryrun,
            level: 0,
        }
    }

    fn traverse(config: Config, limit: Limit, dryrun: bool, level: u64) -> Self {
        #[cfg(debug_assertions)]
        println!("-> Renamer::traverse({})", level);

        Self {
            config: config,
            limit: limit,
            dryrun: dryrun,
            level: level,
        }
    }

    fn get_merged_config(&self, path: &Path) -> Config {
        println!("-> Renamer::get_merged_config({:?})", path);

        match read_local_config(path) {
            Some(_config) => {
                if _config.is_root() {
                    println!("{}    -> take local config{}", BLUE, NO_COLOR);
                    _config
                } else {
                    println!("{}    -> merge config{}", BLUE, NO_COLOR);
                    self.config.merge(&_config)
                }
            },
            None => {
                println!("{}    -> clone config: {:?}{}", BLUE, self.config, NO_COLOR);
                self.config.clone()
            },
        }
    }

    pub fn rename(&self, paths: Paths) -> Stats {
        if cfg!(debug_assertions) {
            println!("-> Renamer::rename(l={}, {:?})", self.level, paths);
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
                    println!("-> path: {:?}", _ppath);

                    let merged_config: Config = self.get_merged_config(&_ppath);

                    println!("{}  -> merged config: {:?}{}", YELLOW, merged_config, NO_COLOR);

                    let files: ReadDir = match read_dir(_ppath) {
                        Ok(_files) => _files,
                        Err(_error) => {
                            println!("{}-> read dir ERROR: {}: {}{}", RED, _error.to_string(), _path, NO_COLOR);
                            stats.errors += 1;
                            continue 'paths_loop;
                        },
                    };

                    'files_loop: for _file in files {
                        let dir_entry = match _file {
                            Ok(_entry) => _entry,
                            Err(_error) => {
                                println!("{}-> File ERROR: {}{}", RED, _error, NO_COLOR);
                                stats.errors += 1;
                                continue 'files_loop;
                            },
                        };

                        println!("{}-> file: {:?} {:?}{}", GREEN, dir_entry, dir_entry.path().file_name(), NO_COLOR);

                        let file_name: String = match dir_entry.path().file_name() {
                            Some(_file_name) => {
                                if _file_name == "renamer.json" || _file_name == ".renamer.json" {
                                    println!("{}  -> skip, renamer config{}", BLUE, NO_COLOR);
                                    continue 'files_loop;
                                }
                                _file_name.to_str().unwrap().into()
                            },
                            None => {
                                println!("{}  -> skip, cannot extract file name from path{}", BLUE, NO_COLOR);
                                continue 'files_loop;
                            },
                        };

                        println!("-> file name: '{}'", file_name);

                        let entry_metadata = match dir_entry.metadata() {
                            Ok(_metadata) => _metadata,
                            Err(_error) => {
                                println!("{}-> no metadata available for {:?}: {}{}", RED, dir_entry, _error, NO_COLOR);
                                stats.errors += 1;
                                continue 'files_loop;
                            },
                        };

                        println!("  -> mode: {:02o}", entry_metadata.mode());

                        if entry_metadata.is_dir() {
                            println!("  -> is dir");
                            stats.dirs += 1;

                            let _spaths = Some(vec![dir_entry.path().display().to_string()]);

                            let _renamer = Self::traverse(merged_config.clone(), stats.rest, self.dryrun, self.level + 1);
                            let _sstats = _renamer.rename(_spaths);
                            stats += _sstats;
                        } else if entry_metadata.is_file() {
                            println!("  -> is file");
                            stats.files += 1;

                            let ext: String = match dir_entry.path().extension() {
                                Some(_ext) => _ext.to_str().unwrap().into(),
                                None => {
                                    println!("{}  -> skip, cannot extract extension from path{}", BLUE, NO_COLOR);
                                    continue 'files_loop;
                                },
                            };

                            if !merged_config.has_ext(&ext) {
                                println!("{}  -> skip, wrong ext: '{}'{}", BLUE, &ext, NO_COLOR);
                                continue 'files_loop;
                            }

                            println!("  -> ext: '{}'", ext);

                            let mut name = merged_config.name();
                            println!("  -> name A: '{}'", name);
                            name = name.replace("%ext%", &(".".to_owned() + &ext));
                            println!("  -> name B: '{}'", name);

                            println!("  -> check regex");
                            'finds_loop: for (regex, vars) in merged_config.regex_finds() {
                                println!("  -> find: {:?}", regex);
                                match regex.captures(&file_name) {
                                    Some(caps) => {
                                        let mut i = 1;
                                        for var in vars {
                                            let value = merged_config.format_var(var.clone(), caps[i].to_string());
                                            name = name.replace(&var, &value);
                                            println!("  -> var: #{} {:?} => {} '{}'", i, var, &caps[i], value);
                                            println!("  -> name C: '{}'", name);
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

                            let path = dir_entry.path();
                            println!("-> path: {:?}", path.parent());

                            let parent = match path.parent() {
                                Some(_parent) => _parent,
                                None => panic!("Cannot get parent from path: {:?}", path),
                            };

                            println!("-> parent: {:?}", parent);

                            let new_file_path = parent.join(name);
                            println!("-> new: {:?}", new_file_path);

                            if self.dryrun {
                                println!("{}-> move (dry-run){}", GREEN, NO_COLOR);
                            }
                            else {
                                println!("{}-> move{}", GREEN, NO_COLOR);
                            }
                            println!("{}   src: {}{}", GREEN, path.display(), NO_COLOR);
                            println!("{}  dest: {}{}", GREEN, new_file_path.display(), NO_COLOR);

                            if !self.dryrun {
                                if cfg!(feature="production") {
                                    // println!("{}   move{}", RED, NO_COLOR); rename(path, new_file_path);
                                }
                                stats += 1;
                            }
                        }

                        if stats.end() {
                            println!("{}-> abort, reached limit{}", BLUE, NO_COLOR);
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
