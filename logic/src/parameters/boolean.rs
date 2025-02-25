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
    fn generate_random_value(&self, rng: &mut dyn RandomNumberGenerator) -> String {
        if rng.generate_range(0, 1) == 0 {
            "false".to_string()
        } else {
            "true".to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::parameters::boolean::BooleanParameter;
    use std::str::FromStr;

    #[test]
    fn test_from_str() {
        let ret = BooleanParameter::from_str("@{boolean}");
        assert!(ret.is_ok());
    }

    #[test]
    fn test_from_str_errors() {
        // Parameters provided
        let ret = BooleanParameter::from_str("@{boolean[0, 1]}");
        assert!(ret.is_err());

        // Wrong type
        let ret = BooleanParameter::from_str("@{int}");
        assert!(ret.is_err());
    }
}
