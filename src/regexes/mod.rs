use std::borrow::Cow;

use regex::Regex;

pub use device::Matcher as DeviceMatcher;
pub use os::Matcher as OSMatcher;

mod device;
mod os;

thread_local!(static INVALID_ESCAPES: Regex = Regex::new("\\\\([! /])").unwrap());

pub fn clean_escapes(pattern: &str) -> Cow<'_, str> {
    INVALID_ESCAPES.with(|escapes| escapes.replace_all(pattern, "$1"))
}
