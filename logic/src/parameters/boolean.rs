use serde::{Deserialize, Serialize};
use std::str::FromStr;

use super::{populator::RandomNumberGenerator, GenerateRandomValues, ParameterError};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct BooleanParameter {}

impl FromStr for BooleanParameter {
    type Err = ParameterError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "@{boolean}" => Ok(Self {}),
            _ => Err(ParameterError::InvalidParameter),
        }
    }
}

impl GenerateRandomValues for BooleanParameter {
    fn generate_random_values(&self, rng: &mut dyn RandomNumberGenerator) -> String {
        if rng.generate_range(0, 1) == 0 {
            "false".to_string()
        } else {
            "true".to_string()
        }
    }
}
