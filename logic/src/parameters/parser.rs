use regex::Regex;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

use super::{
    boolean::BooleanParameter, int::IntParameter, populator::RandomNumberGenerator,
    string::StringParameter, FromStrWithConfig, GenerateRandomValues, ParameterError,
};
use crate::Logic;

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", content = "data")]
pub enum SerializableParameter {
    Int(IntParameter),
    String(StringParameter),
    Boolean(BooleanParameter),
}

impl GenerateRandomValues for SerializableParameter {
    fn generate_random_value(&self, rng: &mut dyn RandomNumberGenerator) -> String {
        match self {
            SerializableParameter::Int(param) => param.generate_random_value(rng),
            SerializableParameter::String(param) => param.generate_random_value(rng),
            SerializableParameter::Boolean(param) => param.generate_random_value(rng),
        }
    }
}

impl Logic {
    pub fn parse_parameters(
        &self,
        command: String,
    ) -> Result<(Vec<String>, Vec<SerializableParameter>), ParameterError> {
        let regex_string = r"\@\{(?P<param>([^}]*))\}";
        let re = Regex::new(regex_string)
            .map_err(|e| ParameterError::InvalidRegex(regex_string.to_string(), e.to_string()))?;

        let mut parameters = Vec::new();
        let mut non_parameter_strs = Vec::new();
        let mut last_end = 0;

        for mat in re.find_iter(&command) {
            let param = self.parse_parameter(mat.as_str().to_owned())?;
            parameters.push(param);

            non_parameter_strs.push(command[last_end..mat.start()].to_string());
            last_end = mat.end();
        }

        if last_end < command.len() {
            non_parameter_strs.push(command[last_end..].to_string());
        } else {
            non_parameter_strs.push("".to_string());
        }

        // There should be a parameter for each "gap" between strings
        assert_eq!(non_parameter_strs.len() - 1, parameters.len());

        Ok((non_parameter_strs, parameters))
    }

    fn parse_parameter(&self, s: String) -> Result<SerializableParameter, ParameterError> {
        if let Ok(string_param) = StringParameter::from_str(&s, &self.config) {
            return Ok(SerializableParameter::String(string_param));
        }

        if let Ok(int_param) = IntParameter::from_str(&s, &self.config) {
            return Ok(SerializableParameter::Int(int_param));
        }

        if let Ok(bool_param) = BooleanParameter::from_str(&s) {
            return Ok(SerializableParameter::Boolean(bool_param));
        }

        Err(ParameterError::InvalidParameter)
    }
}
