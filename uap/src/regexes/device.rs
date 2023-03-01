use regex::Regex;
use yaml_rust::Yaml;

use crate::errors::MatcherError;
use crate::regexes::clean_escapes;

#[derive(Debug, Clone)]
pub struct Matcher {
    pub regex: Regex,
    pub device: Option<String>,
}

impl Matcher {
    pub fn from_yaml(yaml: &Yaml) -> Result<Vec<Matcher>, MatcherError> {
        let yaml_vec = yaml.as_vec().ok_or(MatcherError::IncorrectSource)?;

        let mut matchers = Vec::with_capacity(yaml_vec.len());
        let yaml_regex = Yaml::String("regex".to_string());
        let yaml_device_replacement = Yaml::String("device_replacement".to_string());

        for yaml in yaml_vec {
            let yaml = yaml.as_hash().ok_or(MatcherError::IncorrectSource)?;
            let device = match yaml.get(&yaml_device_replacement) {
                Some(yaml) => {
                    yaml.as_str().map(|s| Some(s.to_string())).ok_or(MatcherError::IncorrectSource)?
                }
                None => None,
            };

            let regex = Regex::new(
                &clean_escapes(yaml.get(&yaml_regex)
                    .ok_or(MatcherError::IncorrectSource)?
                    .as_str()
                    .ok_or(MatcherError::IncorrectSource)?)
            )?;

            let matcher = Matcher {
                regex,
                device,
            };

            matchers.push(matcher);
        }


        Ok(matchers)
    }
}
