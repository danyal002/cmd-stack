use std::str::FromStr;

use super::ParameterError;
use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct BlankParameter(String);

impl FromStr for BlankParameter {
    type Err = ParameterError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let blank_param_regex = r"@\{\s*\}";
        let re = Regex::new(blank_param_regex).map_err(|e| {
            ParameterError::InvalidRegex(blank_param_regex.to_string(), e.to_string())
        })?;

        if re.is_match(s) {
            return Ok(BlankParameter(String::new()));
        }
        Err(ParameterError::InvalidParameter)
    }
}
