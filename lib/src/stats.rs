
use std::ops::AddAssign;

use crate::types::FileCount;
use crate::types::Limit;

pub struct Stats {
    // traverse
    pub dirs: FileCount,
    pub files: FileCount,

    pub renamed: FileCount,
    pub errors: FileCount,
    pub warnings: FileCount,

    pub rest: Limit,
}

impl Stats {
    pub fn new() -> Self {
        #[cfg(debug_assertions)]
        println!("-> Stats::new()");

        Self {
            dirs: 0,
            files: 0,

            renamed: 0,
            errors: 0,
            warnings: 0,

            rest: None,
        }
    }

    pub fn end(&self) -> bool {
        match self.rest {
            Some(_rest) => {
                _rest <= 0
            },
            None => false,
        }
    }
}

impl AddAssign for Stats {
    fn add_assign(&mut self, other: Self) {
        #[cfg(debug_assertions)]
        println!("-> Stats::add_assign(Self)");

        self.dirs += other.dirs;
        self.files += other.files;
        self.renamed += other.renamed;
        self.errors += other.errors;
        self.warnings += other.warnings;

        if let Some(_rest) = &mut self.rest {
            if *_rest > 0 {
                *_rest -= other.renamed;
            }
        }
    }
}

impl AddAssign<FileCount> for Stats {
    fn add_assign(&mut self, other: FileCount) {
        #[cfg(debug_assertions)]
        println!("-> Stats::add_assign(FileCount)");

        self.renamed += other;

        if let Some(_rest) = &mut self.rest {
            if *_rest > 0 {
                *_rest -= other;
            }
        }
    }
}
