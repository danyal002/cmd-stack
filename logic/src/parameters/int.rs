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

impl Default for IntParameter {
    fn default() -> Self {
        IntParameter { min: 5, max: 10 }
    }
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
    fn generate_random_value(&self, rng: &mut dyn RandomNumberGenerator) -> String {
        rng.generate_range(self.min, self.max).to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        parameters::{int::IntParameter, FromStrWithConfig},
        Config,
    };

    #[test]
    fn test_from_str_no_params() {
        let ret = IntParameter::from_str("@{int}", &Config::default());
        assert!(ret.is_ok());
        let param = ret.unwrap();
        assert_eq!(param.min, 5);
        assert_eq!(param.max, 10);
    }

    #[test]
    fn test_from_str_params() {
        let ret = IntParameter::from_str("@{int[-100, -99]}", &Config::default());
        assert!(ret.is_ok());
        let param = ret.unwrap();
        assert_eq!(param.min, -100);
        assert_eq!(param.max, -99);

        let ret = IntParameter::from_str("@{int[-100, 100]}", &Config::default());
        assert!(ret.is_ok());
        let param = ret.unwrap();
        assert_eq!(param.min, -100);
        assert_eq!(param.max, 100);

        let ret = IntParameter::from_str("@{int[99, 100]}", &Config::default());
        assert!(ret.is_ok());
        let param = ret.unwrap();
        assert_eq!(param.min, 99);
        assert_eq!(param.max, 100);

        let ret = IntParameter::from_str("@{int[0, 0]}", &Config::default());
        assert!(ret.is_ok());
        let param = ret.unwrap();
        assert_eq!(param.min, 0);
        assert_eq!(param.max, 0);
    }

    #[test]
    fn test_from_str_errors() {
        // Min and max swapped
        let ret = IntParameter::from_str("@{int[1, 0]}", &Config::default());
        assert!(ret.is_err());

        // Max missing
        let ret = IntParameter::from_str("@{int[1, ]}", &Config::default());
        assert!(ret.is_err());

        // Min missing
        let ret = IntParameter::from_str("@{int[, 1]}", &Config::default());
        assert!(ret.is_err());

        // Min and max missing
        let ret = IntParameter::from_str("@{int[, ]}", &Config::default());
        assert!(ret.is_err());

        // Bracket missing
        let ret = IntParameter::from_str("@{int[0, 1}", &Config::default());
        assert!(ret.is_err());

        // Wrong type
        let ret = IntParameter::from_str("@{string[0, 1]}", &Config::default());
        assert!(ret.is_err());
    }
}
