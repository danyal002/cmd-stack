use std::str::FromStr;

use rand::{rngs::ThreadRng, Rng};
use regex::Regex;
use serde::{Deserialize, Serialize};
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

pub trait RandomStringGenerator {
    fn generate_random(&self, rng: &mut dyn RandomNumberGenerator) -> String;
}

#[derive(Serialize, Deserialize, Debug)]
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

impl RandomStringGenerator for StringParameter {
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
        let re =
            Regex::new(r"@\{string\[(\d+),\s*(\d+)\]\}").map_err(|_| ParameterError::Parsing)?;
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
            "@{string}" => Ok(StringParameter::default()),
            _ => Err(ParameterError::InvalidParameter),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BlankParameter {}

impl BlankParameter {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for BlankParameter {
    fn default() -> Self {
        Self::new()
    }
}

impl RandomStringGenerator for BlankParameter {
    fn generate_random(&self, _rng: &mut dyn RandomNumberGenerator) -> String {
        "@{}".to_string()
    }
}

impl FromStr for BlankParameter {
    type Err = ParameterError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "@{}" => Ok(BlankParameter::default()),
            _ => Err(ParameterError::InvalidParameter),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
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
            "@{int}" => Ok(IntParameter::default()),
            _ => Err(ParameterError::InvalidParameter),
        }
    }
}

impl RandomStringGenerator for IntParameter {
    fn generate_random(&self, rng: &mut dyn RandomNumberGenerator) -> String {
        let random_int = rng.generate_range(self.min, self.max);
        random_int.to_string()
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct BooleanParameter {}

impl FromStr for BooleanParameter {
    type Err = ParameterError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "@{boolean}" => Ok(BooleanParameter::default()),
            _ => Err(ParameterError::InvalidParameter),
        }
    }
}

impl RandomStringGenerator for BooleanParameter {
    fn generate_random(&self, rng: &mut dyn RandomNumberGenerator) -> String {
        let random_int = rng.generate_range(0, 1);
        if random_int == 0 {
            "false".to_string()
        } else {
            "true".to_string()
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", content = "data")]
pub enum SerializableParameter {
    Blank(BlankParameter),
    Int(IntParameter),
    String(StringParameter),
    Boolean(BooleanParameter),
}

impl RandomStringGenerator for SerializableParameter {
    fn generate_random(&self, rng: &mut dyn RandomNumberGenerator) -> String {
        match self {
            SerializableParameter::Blank(param) => param.generate_random(rng),
            SerializableParameter::Int(param) => param.generate_random(rng),
            SerializableParameter::String(param) => param.generate_random(rng),
            SerializableParameter::Boolean(param) => param.generate_random(rng),
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

    fn parse_parameter(&self, s: String) -> Result<SerializableParameter, ParameterError> {
        let ret = BlankParameter::from_str(&s);
        if let Ok(ph) = ret {
            return Ok(SerializableParameter::Blank(ph));
        }

        let ret = StringParameter::from_str(&s);
        if let Ok(ph) = ret {
            return Ok(SerializableParameter::String(ph));
        }

        let ret = IntParameter::from_str(&s);
        if let Ok(ph) = ret {
            return Ok(SerializableParameter::Int(ph));
        }

        let ret: Result<BooleanParameter, ParameterError> = BooleanParameter::from_str(&s);
        if let Ok(ph) = ret {
            return Ok(SerializableParameter::Boolean(ph));
        }

        Err(ParameterError::InvalidParameter)
    }

    pub fn validate_parameters(&mut self, s: String) -> Result<(), ParameterError> {
        let _ = self.replace_parameters(s)?;
        Ok(())
    }

    pub fn replace_parameters(
        &mut self,
        s: String,
    ) -> Result<(String, Vec<String>), ParameterError> {
        let (other_strings, parameters) = self.parse_parameters(s)?;

        // Build the string by generating parameters
        let mut generated_result = String::new();
        let mut generated_parameters = Vec::new();

        for (i, other_string) in other_strings.iter().enumerate() {
            generated_result.push_str(other_string);

            if i < other_strings.len() - 1 {
                let s = parameters[i].generate_random(self.rng.as_mut());
                generated_result.push_str(&s);
                generated_parameters.push(s);
            }
        }

        Ok((generated_result, generated_parameters))
    }

    pub fn parse_parameters(
        &mut self,
        s: String,
    ) -> Result<(Vec<String>, Vec<SerializableParameter>), ParameterError> {
        let re = Regex::new(r"\@\{([^}]*)\}").map_err(|_| ParameterError::Parsing)?;

        let mut parameters = Vec::new();

        let mut other_strings = Vec::new();
        let mut last_end = 0;

        for mat in re.find_iter(&s) {
            match self.parse_parameter(mat.as_str().to_owned()) {
                Ok(param) => {
                    parameters.push(param);
                }
                Err(e) => {
                    return Err(e);
                }
            }

            other_strings.push(s[last_end..mat.start()].to_string());

            last_end = mat.end();
        }

        if last_end < s.len() {
            other_strings.push(s[last_end..].to_string());
        } else {
            other_strings.push("".to_string());
        }

        // There should be a parameter for each "gap" between strings
        assert_eq!(other_strings.len() - 1, parameters.len());

        Ok((other_strings, parameters))
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
        let (generated_string, generated_parameters) = ret.unwrap();
        assert_eq!("fasd @ @email @wadsf @test {} @", generated_string);
    }

    #[test]
    fn test_replace_parameters() {
        let mut ph = ParameterHandler::new(Box::new(MockRng::new(vec![0, 1, 2, 4])));
        let ret = ph.replace_parameters("red-@{int} @nothing @{string} @{int}".to_string());
        assert!(ret.is_ok());
        let (generated_string, generated_parameters) = ret.unwrap();
        assert_eq!("red-5 @nothing CEABCE 5", generated_string);

        let mut ph = ParameterHandler::new(Box::new(MockRng::new(vec![0, 2, 0, 2])));
        let ret = ph.replace_parameters("red-@{int} @nothing @{string} @{int}".to_string());
        assert!(ret.is_ok());
        let (generated_string, generated_parameters) = ret.unwrap();
        assert_eq!("red-5 @nothing ACACACA 7", generated_string);

        let mut ph = ParameterHandler::new(Box::new(MockRng::new(vec![2, 3, 4, 7])));
        let ret = ph.replace_parameters("ls @{int} @{int} @{string} @{string}".to_string());
        assert!(ret.is_ok());
        let (generated_string, generated_parameters) = ret.unwrap();
        assert_eq!("ls 7 8 HCDEHCDEH DEHCDEH", generated_string);

        let mut ph = ParameterHandler::new(Box::new(MockRng::new(vec![2, 2, 2, 2])));
        let ret = ph.replace_parameters("ls @{string[7,7]} @{string[3,3]}".to_string());
        assert!(ret.is_ok());
        let (generated_string, generated_parameters) = ret.unwrap();
        assert_eq!("ls CCCCCCC CCC", generated_string);

        let mut ph = ParameterHandler::new(Box::new(MockRng::new(vec![1, 2, 3])));
        let ret = ph.replace_parameters("ls @{boolean} @{boolean} @{boolean}".to_string());
        assert!(ret.is_ok());
        let (generated_string, generated_parameters) = ret.unwrap();
        assert_eq!("ls true false true", generated_string);

        let mut ph = ParameterHandler::new(Box::new(MockRng::new(vec![1, 2, 3])));
        let ret = ph.replace_parameters("@{string}".to_string());
        assert!(ret.is_ok());
        let (generated_string, generated_parameters) = ret.unwrap();
        assert_eq!("CDBCDB", generated_string);
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
