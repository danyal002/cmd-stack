use regex::Regex;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

use super::{
    blank::BlankParameter, boolean::BooleanParameter, int::IntParameter,
    populator::RandomNumberGenerator, string::StringParameter, FromStrWithConfig,
    GenerateRandomValues, ParameterError,
};
use crate::Logic;

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", content = "data")]
pub enum SerializableParameter {
    Int(IntParameter),
    String(StringParameter),
    Boolean(BooleanParameter),
    Blank,
}

impl GenerateRandomValues for SerializableParameter {
    fn generate_random_value(&self, rng: &mut dyn RandomNumberGenerator) -> String {
        match self {
            SerializableParameter::Int(param) => param.generate_random_value(rng),
            SerializableParameter::String(param) => param.generate_random_value(rng),
            SerializableParameter::Boolean(param) => param.generate_random_value(rng),
            SerializableParameter::Blank => String::new(),
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
        if let Ok(_) = BlankParameter::from_str(&s) {
            return Ok(SerializableParameter::Blank);
        }

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

#[cfg(test)]
mod tests {
    use crate::{parameters::parser::SerializableParameter, Logic};

    #[test]
    fn test_parse_parameters_no_parameter() {
        let logic = Logic::try_default().unwrap();

        let ret = logic.parse_parameters("cmd @ @email @wadsf @test {} @".to_string());
        assert!(ret.is_ok());
        let (non_parameter_strings, parameters) = ret.unwrap();
        assert_eq!(parameters.len(), 0);
        assert_eq!(
            non_parameter_strings,
            vec!["cmd @ @email @wadsf @test {} @".to_string()]
        );
    }

    #[test]
    fn test_parse_parameters() {
        let logic = Logic::try_default().unwrap();

        let ret = logic.parse_parameters("cmd @{boolean} @{int} @{string}".to_string());
        assert!(ret.is_ok());
        let (non_parameter_strings, parameters) = ret.unwrap();
        assert_eq!(parameters.len(), 3);
        matches!(
            parameters.get(0).unwrap(),
            SerializableParameter::Boolean(_)
        );
        matches!(parameters.get(1).unwrap(), SerializableParameter::Int(_));
        matches!(parameters.get(2).unwrap(), SerializableParameter::String(_));
        assert_eq!(
            non_parameter_strings,
            vec![
                "cmd ".to_string(),
                " ".to_string(),
                " ".to_string(),
                "".to_string()
            ]
        );
    }

    #[test]
    fn test_parse_parameter_int() {
        let logic = Logic::try_default().unwrap();

        let ret = logic.parse_parameter("@{int}".to_string());
        assert!(ret.is_ok());
        matches!(ret.unwrap(), SerializableParameter::Int(_));
    }

    #[test]
    fn test_parse_parameter_string() {
        let logic = Logic::try_default().unwrap();

        let ret = logic.parse_parameter("@{string}".to_string());
        assert!(ret.is_ok());
        matches!(ret.unwrap(), SerializableParameter::String(_));
    }

    #[test]
    fn test_parse_parameter_boolean() {
        let logic = Logic::try_default().unwrap();

        let ret = logic.parse_parameter("@{boolean}".to_string());
        assert!(ret.is_ok());
        matches!(ret.unwrap(), SerializableParameter::Boolean(_));
    }
}
