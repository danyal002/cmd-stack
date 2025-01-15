use std::str::FromStr;

use rand::{rngs::ThreadRng, Rng};
use regex::Regex;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParameterError {
    #[error("error parsing parameters from string")]
    Parsing,
    #[error("error invalid parameter")]
    InvalidParameter,
}

pub trait RandomNumberGenerator {
    fn generate_range(&mut self, low: i32, high: i32) -> i32;
}

impl RandomNumberGenerator for ThreadRng {
    fn generate_range(&mut self, low: i32, high: i32) -> i32 {
        self.gen_range(low..high + 1)
    }
}

pub struct MockRng {
    values: Vec<u32>,
    index: usize,
}

impl MockRng {
    pub fn new(values: Vec<u32>) -> Self {
        Self { values, index: 0 }
    }
}

impl RandomNumberGenerator for MockRng {
    fn generate_range(&mut self, low: i32, high: i32) -> i32 {
        let value = self.values[self.index];
        self.index = (self.index + 1) % self.values.len();

        let length = high - low + 1;

        low + (value as i32 % length)
    }
}

pub trait Parameter {
    fn generate_random(&self, rng: &mut dyn RandomNumberGenerator) -> String;
}

pub struct StringParameter {
    min: u32,
    max: u32,
}

impl StringParameter {
    pub fn new(min: u32, max: u32) -> Self {
        Self { min, max }
    }
}

impl Default for StringParameter {
    fn default() -> Self {
        Self { min: 5, max: 10 }
    }
}

impl Parameter for StringParameter {
    fn generate_random(&self, rng: &mut dyn RandomNumberGenerator) -> String {
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

impl FromStr for StringParameter {
    type Err = ParameterError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"string\[(\d+),\s*(\d+)\]").map_err(|_| ParameterError::Parsing)?;
        if let Some(caps) = re.captures(s) {
            let min = caps[1]
                .parse::<u32>()
                .map_err(|_| ParameterError::Parsing)?;
            let max = caps[2]
                .parse::<u32>()
                .map_err(|_| ParameterError::Parsing)?;

            if min > max {
                return Err(ParameterError::InvalidParameter);
            }

            return Ok(StringParameter::new(min, max));
        }

        match s {
            "string" => Ok(StringParameter::default()),
            _ => Err(ParameterError::InvalidParameter),
        }
    }
}

pub struct IntParameter {
    min: i32,
    max: i32,
}

impl IntParameter {
    pub fn new(min: i32, max: i32) -> Self {
        Self { min, max }
    }
}

impl Default for IntParameter {
    fn default() -> Self {
        Self { min: 5, max: 10 }
    }
}

impl FromStr for IntParameter {
    type Err = ParameterError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "int" => Ok(IntParameter::default()),
            _ => Err(ParameterError::InvalidParameter),
        }
    }
}

impl Parameter for IntParameter {
    fn generate_random(&self, rng: &mut dyn RandomNumberGenerator) -> String {
        let random_int = rng.generate_range(self.min, self.max);
        random_int.to_string()
    }
}

pub struct BooleanParameter {
    min: i32,
    max: i32,
}

impl Default for BooleanParameter {
    fn default() -> Self {
        Self { min: 0, max: 1 }
    }
}

impl FromStr for BooleanParameter {
    type Err = ParameterError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "boolean" => Ok(BooleanParameter::default()),
            _ => Err(ParameterError::InvalidParameter),
        }
    }
}

impl Parameter for BooleanParameter {
    fn generate_random(&self, rng: &mut dyn RandomNumberGenerator) -> String {
        let random_int = rng.generate_range(self.min, self.max);
        if random_int == 0 {
            "false".to_string()
        } else {
            "true".to_string()
        }
    }
}

pub struct ParameterHandler {
    rng: Box<dyn RandomNumberGenerator>,
}

impl ParameterHandler {
    pub fn new(rng: Box<dyn RandomNumberGenerator>) -> Self {
        Self { rng }
    }

    fn parse_parameter(&self, s: String) -> Result<Box<dyn Parameter>, ParameterError> {
        let ret = StringParameter::from_str(&s);
        if let Ok(ph) = ret {
            return Ok(Box::new(ph));
        }

        let ret = IntParameter::from_str(&s);
        if let Ok(ph) = ret {
            return Ok(Box::new(ph));
        }

        let ret: Result<BooleanParameter, ParameterError> = BooleanParameter::from_str(&s);
        if let Ok(ph) = ret {
            return Ok(Box::new(ph));
        }

        Err(ParameterError::InvalidParameter)
    }

    pub fn validate_parameters(&mut self, s: String) -> Result<(), ParameterError> {
        let _ = self.replace_parameters(s)?;
        Ok(())
    }

    pub fn replace_parameters(&mut self, s: String) -> Result<String, ParameterError> {
        let re = Regex::new(r"\@\{([^}]*)\}").map_err(|_| ParameterError::Parsing)?;

        let mut err: Option<ParameterError> = None;

        let result = re.replace_all(&s, |caps: &regex::Captures| {
            let param_str = &caps[1];
            match self.parse_parameter(param_str.to_owned()) {
                Ok(param) => param.generate_random(self.rng.as_mut()),
                Err(e) => {
                    err = Some(e);
                    "".to_string()
                }
            }
        });

        if let Some(e) = err {
            return Err(e);
        }

        Ok(result.to_string())
    }
}

impl Default for ParameterHandler {
    fn default() -> Self {
        let rng = rand::thread_rng();
        Self { rng: Box::new(rng) }
    }
}

#[cfg(test)]
mod tests {
    use crate::param::{MockRng, ParameterHandler};

    #[test]
    fn test_zero_parameters() {
        let mut ph = ParameterHandler::new(Box::new(MockRng::new(vec![0, 2])));
        let ret = ph.replace_parameters("fasd @ @email @wadsf @test {} @".to_string());
        assert!(ret.is_ok());
        assert_eq!("fasd @ @email @wadsf @test {} @", ret.unwrap());
    }

    #[test]
    fn test_replace_parameters() {
        let mut ph = ParameterHandler::new(Box::new(MockRng::new(vec![0, 1, 2, 4])));
        let ret = ph.replace_parameters("red-@{int} @nothing @{string} @{int}".to_string());
        assert!(ret.is_ok());
        assert_eq!("red-5 @nothing CEABCE 5", ret.unwrap());

        let mut ph = ParameterHandler::new(Box::new(MockRng::new(vec![0, 2, 0, 2])));
        let ret = ph.replace_parameters("red-@{int} @nothing @{string} @{int}".to_string());
        assert!(ret.is_ok());
        assert_eq!("red-5 @nothing ACACACA 7", ret.unwrap());

        let mut ph = ParameterHandler::new(Box::new(MockRng::new(vec![2, 3, 4, 7])));
        let ret = ph.replace_parameters("ls @{int} @{int} @{string} @{string}".to_string());
        assert!(ret.is_ok());
        assert_eq!("ls 7 8 HCDEHCDEH DEHCDEH", ret.unwrap());

        let mut ph = ParameterHandler::new(Box::new(MockRng::new(vec![2, 2, 2, 2])));
        let ret = ph.replace_parameters("ls @{string[7,7]} @{string[3,3]}".to_string());
        assert!(ret.is_ok());
        assert_eq!("ls CCCCCCC CCC", ret.unwrap());

        let mut ph = ParameterHandler::new(Box::new(MockRng::new(vec![1, 2, 3])));
        let ret = ph.replace_parameters("ls @{boolean} @{boolean} @{boolean}".to_string());
        assert!(ret.is_ok());
        assert_eq!("ls true false true", ret.unwrap());
    }

    #[test]
    fn test_validate_parameters() {
        let mut ph = ParameterHandler::new(Box::new(MockRng::new(vec![])));
        let ret = ph.validate_parameters("fasd @{bad-command}".to_string());
        assert!(ret.is_err());

        let mut ph = ParameterHandler::new(Box::new(MockRng::new(vec![])));
        let ret = ph.validate_parameters("fasd @{string[cat, dog]}".to_string());
        assert!(ret.is_err());

        let mut ph = ParameterHandler::new(Box::new(MockRng::new(vec![])));
        let ret = ph.validate_parameters("fasd @{string[3]}".to_string());
        assert!(ret.is_err());

        let mut ph = ParameterHandler::new(Box::new(MockRng::new(vec![])));
        let ret = ph.validate_parameters("fasd @{string[,]}".to_string());
        assert!(ret.is_err());

        let mut ph = ParameterHandler::new(Box::new(MockRng::new(vec![])));
        let ret = ph.validate_parameters("fasd @{string[-1,5]}".to_string());
        assert!(ret.is_err());

        let mut ph = ParameterHandler::new(Box::new(MockRng::new(vec![])));
        let ret = ph.validate_parameters("fasd @{string[-1,5]}".to_string());
        assert!(ret.is_err());

        let mut ph = ParameterHandler::new(Box::new(MockRng::new(vec![])));
        let ret = ph.validate_parameters("fasd @{string[7,5]}".to_string());
        assert!(ret.is_err());
    }

    #[test]
    fn test_default_random() {
        let mut ph = ParameterHandler::default();
        let ret = ph.validate_parameters("asdfjkf  @{string[1, 1]}".to_string());
        assert!(!ret.is_err());

        let ret = ph.validate_parameters("asdfjkf  @{string[1, 0]}".to_string());
        assert!(ret.is_err());
    }
}
