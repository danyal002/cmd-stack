use populator::RandomNumberGenerator;
use thiserror::Error;

use crate::config::Config;

pub mod boolean;
pub mod int;
pub mod parser;
pub mod populator;
pub mod string;

pub trait FromStrWithConfig: Sized {
    fn from_str(s: &str, config: &Config) -> Result<Self, ParameterError>;
}

pub trait GenerateRandomValues {
    fn generate_random_values(&self, rng: &mut dyn RandomNumberGenerator) -> String;
}

#[derive(Error, Debug)]
pub enum ParameterError {
    #[error("Failed to parse into {0} type from string value {1}")]
    TypeParsing(String, String),
    #[error("Invalid Parameter")]
    InvalidParameter,
    #[error("Invalid regex pattern: {0} Error: {1}")]
    InvalidRegex(String, String),
    #[error("Invalid (min,max): ({0},{1}) provided")]
    InvalidMinMax(String, String),
}
