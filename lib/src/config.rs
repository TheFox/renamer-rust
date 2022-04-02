
use std::fs::read_to_string;
use serde::Serialize;
use serde::Deserialize;
use serde_json::Result;
use serde_json::from_str;
use serde_json::Value;

use crate::types::ConfigPath;
use crate::colors::NO_COLOR;
use crate::colors::RED;
use crate::colors::BLUE;


#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    root: bool,
}

impl Config {
    pub fn new() -> Self {
        Self {
            root: true,
        }
    }

    pub fn from_file(config_path: ConfigPath) -> Self {
        println!("{}-> config path: {:?}{}", BLUE, config_path, NO_COLOR);
        match config_path {
            Some(_config_path) => {
                let data = read_to_string(_config_path).expect("Unable to read file");
                from_str(&data).expect("JSON was not well-formatted")
            }
            None => Self::new(),
        }
    }
}
