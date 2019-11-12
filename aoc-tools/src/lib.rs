
pub mod prelude {
    pub use std::collections::{
        HashMap,
        HashSet,
        BinaryHeap,
        VecDeque,
    };
    pub use std::cmp::{max, min};

    pub use aoc_tools_macros::{parse, parse_multiple};
    pub use std::str::FromStr;
    //pub use crate::{MyFromStr, MyParse};
}

pub mod __imp {
    pub use lazy_static::lazy_static;
    pub use regex::Regex;
}

/*
pub trait MyFromStr: Sized {
    fn my_from_str(s: &str) -> Result<Self, String>;
}

pub trait FromStrMultiple: Sized {
    fn my_from_str_multiple(s: &str) -> Vec<Self>;
}

impl<T> MyFromStr for Vec<T>
where T: FromStrMultiple {
    fn my_from_str(s: &str) -> Result<Self, String> {
        Ok(T::my_from_str_multiple(s))
    }
}

pub trait MyParse {
    fn my_parse<T: MyFromStr>(&self) -> Result<T, String>;
}

impl MyParse for str {
    fn my_parse<T: MyFromStr>(&self) -> Result<T, String> {
        T::my_from_str(self)
    }
}

trait MyStrDerive: std::str::FromStr {
}

impl<T> MyFromStr for T where T: MyStrDerive {
    fn my_from_str(s: &str) -> Result<Self, String> {
        Self::from_str(s).map_err(|e| "error".to_string())
    }
}

impl MyStrDerive for usize {}
impl MyStrDerive for i32 {}
impl MyStrDerive for char {}
impl MyStrDerive for String {}
*/