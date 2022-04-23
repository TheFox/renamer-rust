
use std::ops::Add;
use std::ops::AddAssign;
use std::cmp::PartialOrd;
use std::cmp::Ordering;

#[derive(Debug, Clone, Copy)]
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

    pub fn from_u8(verbose: u8) -> Self {
        Self {
            verbose: verbose,
            is_init: true,
        }
    }

    pub fn from_option(o: Option<u8>) -> Self {
        match o {
            Some(_v) => Self::from_u8(_v),
            None => Self::new(),
        }
    }

    pub fn set(&mut self, v: u8) {
        self.verbose = v;
        self.is_init = true;
    }

    pub fn get(&self) -> u8 {
        self.verbose
    }

    pub fn is_init(&self) -> bool {
        self.is_init
    }
}

impl Add for Verbose {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        //println!("->");
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

impl PartialEq<i32> for Verbose {
    fn eq(&self, other: &i32) -> bool {
        self.verbose == *other as u8
    }
}

impl PartialOrd<i32> for Verbose {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.verbose.partial_cmp(&(*other as u8))
    }

    fn lt(&self, other: &i32) -> bool {
        self.verbose < *other as u8
    }

    fn le(&self, other: &i32) -> bool {
        self.verbose <= *other as u8
    }

    fn gt(&self, other: &i32) -> bool {
        self.verbose > *other as u8
    }

    fn ge(&self, other: &i32) -> bool {
        self.verbose >= *other as u8
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

    type EqRes = (bool, bool, bool, bool, bool);
    type EqData = (u8, i32, EqRes);

    #[test]
    fn test_verbose_eq() {
        let data: Vec<EqData> = vec![
            (1, 1, (true, false, true, false, true)),
            (1, 2, (false, true, true, false, false)),
            (2, 2, (true, false, true, false, true)),
            (5, 2, (false, false, false, true, true)),
        ];

        for (_v, _c, (_eq, _lt, _le, _gt, _ge)) in data {
            let v1 = Verbose::from_u8(_v);

            assert_eq!(_eq, v1 == _c);

            assert_eq!(_lt, v1 < _c);
            assert_eq!(_le, v1 <= _c);

            assert_eq!(_gt, v1 > _c);
            assert_eq!(_ge, v1 >= _c);
        }
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
