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
        self.gen_range(low..high)
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

        return low + (value as i32 % length);
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
    pub fn default() -> Self {
        Self { min: 5, max: 10 }
    }

    pub fn new(min: u32, max: u32) -> Self {
        Self { min, max }
    }
}

impl Parameter for StringParameter {
    fn generate_random(&self, rng: &mut dyn RandomNumberGenerator) -> String {
        let charset: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";

        let random_string: String = (self.min..self.max)
            .map(|_| {
                let idx = rng.generate_range(0, charset.len() as i32);
                charset[idx as usize] as char
            })
            .collect();
        random_string
    }
}

pub struct IntParameter {
    min: i32,
    max: i32,
}

impl IntParameter {
    pub fn default() -> Self {
        Self { min: 5, max: 10 }
    }

    pub fn new(min: i32, max: i32) -> Self {
        Self { min, max }
    }
}

impl Parameter for IntParameter {
    fn generate_random(&self, rng: &mut dyn RandomNumberGenerator) -> String {
        let random_int = rng.generate_range(self.min, self.max);
        random_int.to_string()
    }
}

pub struct ParameterHandler {
    rng: Box<dyn RandomNumberGenerator>,
}

impl ParameterHandler {
    pub fn default() -> Self {
        let rng = rand::thread_rng();
        Self { rng: Box::new(rng) }
    }

    pub fn new(rng: Box<dyn RandomNumberGenerator>) -> Self {
        Self { rng }
    }

    pub fn parse_parameter(&self, s: String) -> Result<Box<dyn Parameter>, ParameterError> {
        match s.as_str() {
            "@string" => Ok(Box::new(StringParameter::default())),
            "@int" => Ok(Box::new(IntParameter::default())),
            _ => Err(ParameterError::InvalidParameter),
        }
    }

    pub fn validate_parameters(&mut self, s: String) -> Result<(), ParameterError> {
        let _ = self.replace_parameters(s)?;
        Ok(())
    }

    pub fn replace_parameters(&mut self, s: String) -> Result<String, ParameterError> {
        let re = Regex::new(r"\{([^}]*)\}").map_err(|_| ParameterError::Parsing)?;

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

        if err.is_some() {
            return Err(err.unwrap());
        }

        Ok(result.to_string())
    }
}

#[cfg(test)]
mod tests {
    use crate::param::{MockRng, ParameterHandler};

    #[test]
    fn test_zero_parameters() {
        let mut ph = ParameterHandler::new(Box::new(MockRng::new(vec![0, 2])));
        let ret = ph.replace_parameters("fasd @email @wadsf @test".to_string());
        assert!(ret.is_ok());
        assert_eq!("fasd @email @wadsf @test", ret.unwrap());
    }

    #[test]
    fn test_replace_parameters() {
        let mut ph = ParameterHandler::new(Box::new(MockRng::new(vec![0, 1, 2, 4])));
        let ret = ph.replace_parameters("red-{@int} @nothing {@string} {@int}".to_string());
        assert!(ret.is_ok());
        assert_eq!("red-5 @nothing BCEAB 7", ret.unwrap());

        let mut ph = ParameterHandler::new(Box::new(MockRng::new(vec![0, 2, 0, 2])));
        let ret = ph.replace_parameters("red-{@int} @nothing {@string} {@int}".to_string());
        assert!(ret.is_ok());
        assert_eq!("red-5 @nothing CACAC 5", ret.unwrap());
    }

    #[test]
    fn test_validate_parameters() {
        let mut ph = ParameterHandler::new(Box::new(MockRng::new(vec![0, 2])));
        let ret = ph.validate_parameters("fasd {@bad-command}".to_string());
        assert!(ret.is_err());
    }
}
