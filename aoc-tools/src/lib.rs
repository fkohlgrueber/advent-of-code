
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
}

pub mod __imp {
    pub use lazy_static::lazy_static;
    pub use regex::Regex;
}
