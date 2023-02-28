use std::borrow::Cow;
use std::str::FromStr;

use regex::Regex;
use yaml_rust::{Yaml, YamlLoader};

use crate::errors::MatcherError;
use crate::models::{Device, OS};
use crate::regexes::{DeviceMatcher, OSMatcher};

pub struct UAP {
    replacement_regex: Regex,
    device_regexes: Vec<DeviceMatcher>,
    os_regexes: Vec<OSMatcher>,
}

impl UAP {
    pub fn from_str<S: AsRef<str>>(yaml: S) -> Result<UAP, MatcherError> {
        let yaml_vec = YamlLoader::load_from_str(yaml.as_ref())?;
        let yaml = &yaml_vec[0];

        match yaml.as_hash() {
            Some(yaml) => {
                let os_parsers = yaml.get(&Yaml::String("os_parsers".to_string()));
                let device_parsers = yaml.get(&Yaml::String("device_parsers".to_string()));

                let os_regexes = match os_parsers {
                    Some(os_parsers) => OSMatcher::from_yaml(os_parsers)?,
                    None => Vec::new(),
                };

                let device_regexes = match device_parsers {
                    Some(device_parsers) => DeviceMatcher::from_yaml(device_parsers)?,
                    None => Vec::new(),
                };
                Ok(UAP {
                    replacement_regex: Regex::new(r"\$(\d){1,9}").unwrap(),
                    device_regexes,
                    os_regexes,
                })
            }
            None => Err(MatcherError::IncorrectSource),
        }
    }
}

macro_rules! get_string {
    ($index:expr, $replacement:expr, $replacement_regex:expr, $captures:expr) => {
        match $replacement.as_ref() {
            Some(replacement) => {
                let replacement_captures_vec: Vec<_> =
                    $replacement_regex.captures_iter(replacement).collect();

                if replacement_captures_vec.is_empty() {
                    Some(Cow::from(replacement))
                } else {
                    let mut replacement = replacement.to_string();

                    let captures_len = $captures.len();

                    for replacement_captures in replacement_captures_vec.into_iter().rev() {
                        let index = replacement_captures.get(1).unwrap().as_str().parse::<usize>().unwrap();

                        let pos = replacement_captures.get(1).unwrap();

                        if index < captures_len {
                            replacement.replace_range(
                                (pos.start() - 1)..pos.end(),
                                $captures.get(index).unwrap().as_str(),
                            );
                        } else {
                            replacement.replace_range((pos.start() - 1)..pos.end(), "");
                        }
                    }

                    let start_trimmed_replacement = replacement.trim_start();

                    if start_trimmed_replacement.len() != replacement.len() {
                        replacement = start_trimmed_replacement.trim_end().to_string();
                    } else {
                        replacement.truncate(replacement.trim_end().len());
                    }

                    if replacement.is_empty() {
                        None
                    } else {
                        Some(Cow::from(replacement))
                    }
                }
            }
            None => {
                match $captures.get($index) {
                    Some(s) => {
                        let s = s.as_str().trim();

                        if s.is_empty() {
                            None
                        } else {
                            Some(Cow::from(s))
                        }
                    }
                    None => None,
                }
            }
        }
    };
}

impl UAP {
    pub fn parse_device<'a, S: AsRef<str> + ?Sized>(&'a self, user_agent: &'a S) -> Device<'a> {
        let mut device = Device::default();

        for device_regex in self.device_regexes.iter() {
            if let Some(captures) = device_regex.regex.captures(user_agent.as_ref()) {
                device.device = get_string!(
                    1,
                    device_regex.device,
                    self.replacement_regex,
                    captures
                );

                break;
            }
        }

        if device.device.is_none() {
            device.device = Some(Cow::from("Other"));
        }

        device
    }

    pub fn parse_os<'a, S: AsRef<str> + ?Sized>(&'a self, user_agent: &'a S) -> OS<'a> {
        let mut os = OS::default();

        for os_regex in self.os_regexes.iter() {
            if let Some(captures) = os_regex.regex.captures(user_agent.as_ref()) {
                os.os = get_string!(1, os_regex.os, self.replacement_regex, captures);

                break;
            }
        }

        if os.os.is_none() {
            os.os = Some(Cow::from("Other"));
        }

        os
    }
}

impl FromStr for UAP {
    type Err = MatcherError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        UAP::from_str(s)
    }
}
