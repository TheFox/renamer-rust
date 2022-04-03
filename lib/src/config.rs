
use std::fs::read_to_string;
use std::path::PathBuf;
use std::collections::HashMap;

use serde::Serialize;
use serde::Deserialize;
use serde_json::Result;
use serde_json::from_str;
use serde_json::Value;

use crate::types::ConfigPath;
use crate::colors::NO_COLOR;
use crate::colors::RED;
use crate::colors::BLUE;

#[derive(Debug, Deserialize)]
struct Function {
    r#fn: String,
    search: Option<String>,
    replace: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Var {
    r#type: String,
    format: Option<String>,
    default: Option<u64>,
    fns: Option<Vec<Function>>,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    root: Option<bool>,
    errors: Option<bool>,
    name: Option<String>,
    exts: Option<Vec<String>>,
    vars: Option<HashMap<String, Var>>,
}

impl Config {
    pub fn new() -> Self {
        Self {
            root: None,
            errors: None,
            name: None,
            exts: None,
            vars: None,
        }
    }

    pub fn from_path(path: String) -> Self {
        let data = read_to_string(path).expect("Unable to read file");

        // if cfg!(debug_assertions) {
        //     let json: Value = from_str(&data).expect("JSON does not have correct format.");
        //     dbg!(json);
        // }

        from_str(&data).expect("JSON was not well-formatted")
    }

    pub fn from_config_path(config_path: ConfigPath) -> Self {
        println!("{}-> config path: {:?}{}", BLUE, config_path, NO_COLOR);
        match config_path {
            Some(_config_path) => Self::from_path(_config_path),
            None => Self::new(),
        }
    }

    pub fn from_path_buf(path_buf: PathBuf) -> Self {
        Self::from_path(path_buf.display().to_string())
    }

    pub fn merge(&mut self) -> Self {
        println!("{}-> merge{}", BLUE, config_path, NO_COLOR);

        let mut config = self.clone();

        config
    }
}

#[cfg(test)]
mod tests_config {
    use super::Config;

    #[test]
    fn test_config_merge() {
        let mut c1 = Config::new();
        c1.name = Some(String::from("c1"));

        let mut c2 = Config::new();
        c2.name = Some(String::from("c2"));
    }
}
