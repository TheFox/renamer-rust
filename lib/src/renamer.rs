
use std::fs::read_dir;
use std::path::Path;
use std::path::PathBuf;
use std::ffi::OsStr;
use std::error::Error;
use std::result::Result;
use std::fs::DirEntry;

use crate::types::ConfigPath;
use crate::types::Paths;
use crate::types::Limit;

pub struct Renamer {
    config: ConfigPath,
}

impl Renamer {
    pub fn new(config: ConfigPath) -> Self {
        #[cfg(debug_assertions)]
        println!("-> Renamer::new({:?})", config);

        Self {
            config: config,
        }
    }

    pub fn rename(&self, paths: Paths, limit: Limit, dryrun: bool) {
        if cfg!(debug_assertions) {
            println!("-> Renamer::rename({:?}, {:?}, {:?})", paths, limit, dryrun);
        }

        println!("-> paths: {:?}", paths);

        // let mut renamed_c: usize = 0;
        let mut rest_c: usize = 0;
        if let Some(_limit) = limit {
            rest_c = _limit;
        }

        match paths {
            Some(_paths) => {
                for _path in &_paths {
                    let _p = &String::from(_path);
                    let _p = Path::new(OsStr::new(_p));

                    println!("-> path: {:?}", _path);

                    match read_dir(_p) {
                        Ok(_files) => {
                            for _file in _files {
                                match _file {
                                    Ok(_entry) => {
                                        println!("-> file: {:?} {:?}", _path, _entry);
                                        match _entry.metadata() {
                                            Ok(_metadata) => {
                                                println!("-> metadata: {:?}", _metadata);
                                                // println!("-> is file: {:?}", _metadata.is_file());

                                                if _metadata.is_dir() {
                                                    let _spaths = Some(vec![_entry.path().display().to_string()]);
                                                    let _renamer = Self::new(self.config.clone());
                                                    _renamer.rename(_spaths, Some(rest_c), dryrun);
                                                } else if _metadata.is_file() {
                                                    // TODO
                                                }
                                            },
                                            Err(_error) => {
                                                panic!("No metadata available for {:?}: {}", _entry, _error);
                                            },
                                        }
                                    },
                                    Err(_error) => {
                                        println!("-> File ERROR: {}", _error);
                                    },
                                }
                            }
                        },
                        Err(_error) => {
                            println!("-> ERROR: {:?}", _error.description());
                            break; // TODO
                        },
                    }
                }
            },
            None => {},
        }

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
