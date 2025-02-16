use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::config::Config;

use super::{
    populator::RandomNumberGenerator, FromStrWithConfig, GenerateRandomValues, ParameterError,
};

#[derive(Serialize, Deserialize, Debug)]
pub struct StringParameter {
    min: u32,
    max: u32,
}

impl Default for StringParameter {
    fn default() -> Self {
        StringParameter { min: 5, max: 10 }
    }
}

impl FromStrWithConfig for StringParameter {
    fn from_str(s: &str, config: &Config) -> Result<Self, ParameterError> {
        let string_param_regex = r"@\{string(?:\[(?P<min>(\d+)),\s*(?P<max>(\d+))\])?\}";
        let re = Regex::new(string_param_regex).map_err(|e| {
            ParameterError::InvalidRegex(string_param_regex.to_string(), e.to_string())
        })?;

        if let Some(caps) = re.captures(s) {
            let min: u32 = if let Some(min) = caps.name("min") {
                min.as_str().parse::<u32>().map_err(|_| {
                    ParameterError::TypeParsing(
                        std::any::type_name::<u32>().to_string(),
                        min.as_str().to_owned(),
                    )
                })?
            } else {
                config.param_string_length_min
            };

            let max: u32 = if let Some(max) = caps.name("max") {
                max.as_str().parse::<u32>().map_err(|_| {
                    ParameterError::TypeParsing(
                        std::any::type_name::<u32>().to_string(),
                        max.as_str().to_owned(),
                    )
                })?
            } else {
                config.param_string_length_max
            };

            if min > max {
                return Err(ParameterError::InvalidMinMax(
                    min.to_string(),
                    max.to_string(),
                ));
            }

            return Ok(Self { min, max });
        }
        Err(ParameterError::InvalidParameter)
    }
}

impl GenerateRandomValues for StringParameter {
    fn generate_random_value(&self, rng: &mut dyn RandomNumberGenerator) -> String {
        let charset: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";

        let length = rng.generate_range(self.min as i32, self.max as i32) as usize;

        assert!(!charset.is_empty());

        let random_string: String = (0..length)
            .map(|_| {
                let idx = rng.generate_range(0, (charset.len() - 1) as i32);
                charset[idx as usize] as char
            })
            .collect();
        random_string
    }
}
