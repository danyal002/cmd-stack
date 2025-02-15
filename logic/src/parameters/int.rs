use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::config::Config;

use super::{
    populator::RandomNumberGenerator, FromStrWithConfig, GenerateRandomValues, ParameterError,
};

#[derive(Serialize, Deserialize, Debug)]
pub struct IntParameter {
    min: i32,
    max: i32,
}

impl FromStrWithConfig for IntParameter {
    fn from_str(s: &str, config: &Config) -> Result<Self, ParameterError> {
        let int_param_regex = r"@\{int(?:\[(?P<min>-?\d+),\s*(?P<max>-?\d+)\])?\}";
        let re = Regex::new(int_param_regex).map_err(|e| {
            ParameterError::InvalidRegex(int_param_regex.to_string(), e.to_string())
        })?;

        if let Some(caps) = re.captures(s) {
            let min: i32 = if let Some(min) = caps.name("min") {
                min.as_str().parse::<i32>().map_err(|_| {
                    ParameterError::TypeParsing(
                        std::any::type_name::<i32>().to_string(),
                        min.as_str().to_owned(),
                    )
                })?
            } else {
                config.param_int_range_min
            };

            let max: i32 = if let Some(max) = caps.name("max") {
                max.as_str().parse::<i32>().map_err(|_| {
                    ParameterError::TypeParsing(
                        std::any::type_name::<i32>().to_string(),
                        max.as_str().to_owned(),
                    )
                })?
            } else {
                config.param_int_range_max
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

impl GenerateRandomValues for IntParameter {
    fn generate_random_values(&self, rng: &mut dyn RandomNumberGenerator) -> String {
        rng.generate_range(self.min, self.max + 1).to_string()
    }
}
