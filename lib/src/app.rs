/// App

use crate::types::ConfigPath;
use crate::types::Paths;
use crate::types::Limit;

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
