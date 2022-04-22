
use std::ops::Add;
use std::ops::AddAssign;

pub struct Verbose {
    verbose: u8,
    is_init: bool,
}

impl Verbose {
    pub fn new() -> Self {
        Self {
            verbose: 0,
            is_init: false,
        }
    }
}

impl Add for Verbose {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        if self.is_init && other.is_init {
            Self {
                verbose: other.verbose,
                is_init: true,
            }
        } else if self.is_init {
            self
        } else if other.is_init {
            other
        } else {
            Self::new()
        }
    }
}

#[cfg(test)]
mod tests_verbose {
    use super::Verbose;

    #[test]
    fn test_verbose_new() {
        let v1 = Verbose::new();

        assert_eq!(0, v1.verbose);
        assert_eq!(false, v1.is_init);
    }

    type SingleVerbose = (u8, bool);
    type AddData = (SingleVerbose, SingleVerbose, SingleVerbose);

    #[test]
    fn test_verbose_add() {
        let data: Vec<AddData> = vec![
            ((1, true), (2, true), (2, true)),
            ((1, true), (2, false), (1, true)),
            ((3, false), (2, true), (2, true)),
            ((4, false), (5, false), (0, false)),
        ];

        for (_v1, _v2, _res) in data {
            let mut v1 = Verbose::new();
            v1.verbose = _v1.0;
            v1.is_init = _v1.1;

            let mut v2 = Verbose::new();
            v2.verbose = _v2.0;
            v2.is_init = _v2.1;

            let v3 = v1 + v2;
            assert_eq!(_res.0, v3.verbose);
            assert_eq!(_res.1, v3.is_init);
        }
    }
}
