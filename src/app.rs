/// App

use renamer_lib::types::ConfigPath;
use renamer_lib::types::Paths;
use renamer_lib::types::Limit;

#[derive(Debug)]
pub struct App {
    pub config: ConfigPath,
    pub paths: Paths,
    pub limit: Limit,
    pub dryrun: bool,
    pub verbose: u8,
}

impl App {
    pub fn new() -> Self {
        Self {
            config: None,
            paths: None,
            limit: None,
            dryrun: false,
            verbose: 1,
        }
    }
}
