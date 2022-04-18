/// App

use renamer_lib::types::ConfigPath;
use renamer_lib::types::Paths;
use renamer_lib::types::Limit;
use renamer_lib::types::MaxDepth;

#[derive(Debug)]
pub struct App {
    pub config: ConfigPath,
    pub paths: Paths,
    pub limit: Limit,
    pub max_depth: MaxDepth,
    pub dryrun: bool,
    pub verbose: u8,
}

impl App {
    pub fn new() -> Self {
        Self {
            config: None,
            paths: None,
            limit: None,
            max_depth: None,
            dryrun: false,
            verbose: 0,
        }
    }
}
