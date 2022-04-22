
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
    pub skipped: FileCount,

    pub rest: Limit,
}

impl Stats {
    pub fn new() -> Self {
        // #[cfg(debug_assertions)]
        // println!("-> Stats::new()");

        Self {
            dirs: 0,
            files: 0,

            renamed: 0,
            errors: 0,
            warnings: 0,
            skipped: 0,

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
        // #[cfg(debug_assertions)]
        // println!("-> Stats::add_assign(Self)");

        self.dirs += other.dirs;
        self.files += other.files;
        self.renamed += other.renamed;
        self.errors += other.errors;
        self.warnings += other.warnings;
        self.skipped += other.skipped;

        if let Some(_rest) = &mut self.rest {
            if *_rest > 0 {
                *_rest -= other.renamed;
            }
        }
    }
}

impl AddAssign<FileCount> for Stats {
    fn add_assign(&mut self, other: FileCount) {
        // #[cfg(debug_assertions)]
        // println!("-> Stats::add_assign(FileCount)");

        self.renamed += other;

        if let Some(_rest) = &mut self.rest {
            if *_rest > 0 {
                *_rest -= other;
            }
        }
    }
}


#[cfg(test)]
mod tests_stats {
    use super::Stats;

    #[test]
    fn test_stats1() {
        let mut s1 = Stats::new();
        s1.dirs += 1;
        s1 += 1;

        assert_eq!(1, s1.dirs);
        assert_eq!(1, s1.renamed);
    }

    #[test]
    fn test_stats2() {
        let mut s1 = Stats::new();
        s1.dirs += 1;
        s1 += 2;

        let mut s2 = Stats::new();
        s2.dirs += 2;
        s2 += 1;
        s2 += 1;
        s2 += s1;

        assert_eq!(3, s2.dirs);
        assert_eq!(4, s2.renamed);
    }

    #[test]
    fn test_stats3() {
        let mut s1 = Stats::new();
        s1.rest = Some(10);
        assert_eq!(10, s1.rest.unwrap());
        assert!(!s1.end());

        s1 += 2;
        assert_eq!(8, s1.rest.unwrap());
        assert!(!s1.end());

        s1 += 8;
        assert_eq!(0, s1.rest.unwrap());
        assert!(s1.end());

        s1 += 2;
        assert_eq!(0, s1.rest.unwrap());
        assert!(s1.end());
    }
}
