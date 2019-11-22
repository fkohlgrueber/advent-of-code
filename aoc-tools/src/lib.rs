
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
    pub use crate::Challenge;
    pub use crate::RunChallenge;
    pub use regex::Regex;
    pub use lazy_static::lazy_static;
    pub use chrono::prelude::*;
}

pub mod __imp {
    pub use lazy_static::lazy_static;
    pub use regex::Regex;
}

pub trait Challenge {
    type Input: Clone; 

    fn parse(input: String) -> Self::Input;

    fn part_1(input: Self::Input) -> String;
    
    fn part_2(input: Self::Input) -> String;

    fn part_1_str(input: String) -> String {
        Self::part_1(Self::parse(input))
    }

    fn part_2_str(input: String) -> String {
        Self::part_2(Self::parse(input))
    }

    fn test_part_1<T: std::fmt::Display>(input: &str, exp: T) {
        assert_eq!(Self::part_1_str(input.to_string()), exp.to_string());
    } 

    fn test_part_2<T: std::fmt::Display>(input: &str, exp: T) {
        assert_eq!(Self::part_2_str(input.to_string()), exp.to_string());
    } 
}

pub trait RunChallenge {
    fn run(&self, input: String) -> (String, String);
}

impl<T> RunChallenge for T 
where T: Challenge {
    fn run(&self, input: String) -> (String, String) {
        let parsed = T::parse(input);
        (T::part_1(parsed.clone()), T::part_2(parsed))
    }
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

use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct AocResults(BTreeMap<usize, BTreeMap<usize, (String, String)>>);

impl AocResults {
    pub fn get(&self, year: usize, day: usize) -> Option<&(String, String)> {
        if let Some(y) = self.0.get(&year) {
            return y.get(&day);
        }
        None
    }

    pub fn from_file(s: &str) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(ron::de::from_str(&std::fs::read_to_string(s)?)?)
    }

    pub fn write_to_file(&self, s: &str) -> Result<(), Box<dyn std::error::Error>> {
        std::fs::write(s, ron::ser::to_string_pretty(self, ron::ser::PrettyConfig::default())?)?;
        Ok(())
    }

    pub fn insert(&mut self, year: usize, day: usize, res: (String, String)) {
        self.0.entry(year).or_default().insert(day, res);
    }
}