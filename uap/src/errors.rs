use std::error::Error;
use std::fmt::{Display, Error as FmtError, Formatter};

use regex::Error as RegexError;
use yaml_rust::ScanError;

#[derive(Debug)]
pub enum MatcherError {
    ScanError(ScanError),
    RegexError(RegexError),
    IncorrectSource,
}

impl Display for MatcherError {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), FmtError> {
        match self {
            MatcherError::ScanError(err) => Display::fmt(&err, f),
            MatcherError::RegexError(err) => Display::fmt(&err, f),
            MatcherError::IncorrectSource => f.write_str("The source of regular expressions is incorrect."),
        }
    }
}

impl Error for MatcherError {}

impl From<ScanError> for MatcherError {
    #[inline]
    fn from(error: ScanError) -> MatcherError {
        MatcherError::ScanError(error)
    }
}

impl From<RegexError> for MatcherError {
    #[inline]
    fn from(error: RegexError) -> MatcherError {
        MatcherError::RegexError(error)
    }
}
