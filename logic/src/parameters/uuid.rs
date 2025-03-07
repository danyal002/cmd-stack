use regex::Regex;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use uuid::Uuid;

use super::ParameterError;

#[derive(Serialize, Deserialize, Debug)]
pub struct UuidParameter;

impl FromStr for UuidParameter {
    type Err = ParameterError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let uuid_param_regex = r"@\{uuid\}";
        let re = Regex::new(uuid_param_regex).map_err(|e| {
            ParameterError::InvalidRegex(uuid_param_regex.to_string(), e.to_string())
        })?;
        if re.is_match(s) {
            Ok(UuidParameter)
        } else {
            Err(ParameterError::InvalidParameter)
        }
    }
}

impl UuidParameter {
    pub fn generate_random_value(&self) -> String {
        Uuid::new_v4().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_parse_uuid() {
        let result = UuidParameter::from_str("@{uuid}");
        assert!(result.is_ok());
    }

    #[test]
    fn test_invalid_uuid() {
        // This should fail because the format doesn't match exactly.
        let result = UuidParameter::from_str("@{uuid[3,4]}");
        assert!(result.is_err());
    }
}
