use std::borrow::Cow;

use lazy_static::lazy_static;
use regex::Regex;

pub use device::Matcher as DeviceMatcher;
pub use os::Matcher as OSMatcher;

mod device;
mod os;

lazy_static! {
    static ref INVALID_ESCAPES: Regex = Regex::new("\\\\([! /])").unwrap();
}

pub fn clean_escapes(pattern: &str) -> Cow<'_, str> {
    INVALID_ESCAPES.replace_all(pattern, "$1")
}
