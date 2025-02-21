use rand::{rngs::ThreadRng, Rng};

use super::{parser::SerializableParameter, GenerateRandomValues, ParameterError};
use crate::Logic;

pub trait RandomNumberGenerator {
    fn generate_range(&mut self, low: i32, high: i32) -> i32;
}

impl RandomNumberGenerator for ThreadRng {
    fn generate_range(&mut self, low: i32, high: i32) -> i32 {
        self.gen_range(low..=high)
    }
}

impl Logic {
    pub fn populate_parameters(
        &self,
        non_parameter_strs: Vec<String>,
        parameters: Vec<SerializableParameter>,
        blank_parameter_values: Vec<String>,
        rng: Option<Box<dyn RandomNumberGenerator>>,
    ) -> Result<(String, Vec<String>), ParameterError> {
        let mut rng = if let Some(rng) = rng {
            rng
        } else {
            Box::new(ThreadRng::default())
        };

        // Build the string by generating parameters
        let mut generated_result = String::new();
        let mut generated_parameters = Vec::new();
        let mut blank_param_used_index = 0;

        for (i, other_string) in non_parameter_strs.iter().enumerate() {
            generated_result.push_str(other_string);

            if i < non_parameter_strs.len() - 1 {
                let generated_value = match &parameters[i] {
                    SerializableParameter::Blank => {
                        if blank_param_used_index < blank_parameter_values.len() {
                            let user_val = blank_parameter_values[blank_param_used_index].clone();
                            blank_param_used_index += 1;
                            user_val
                        } else {
                            let total_blank_params_needed = parameters
                                .iter()
                                .filter(|p| matches!(p, SerializableParameter::Blank))
                                .count();
                            return Err(ParameterError::MissingBlankParamValues(
                                (blank_param_used_index + 1).to_string(),
                                total_blank_params_needed.to_string(),
                            ));
                        }
                    }
                    _ => parameters[i].generate_random_value(rng.as_mut()),
                };

                generated_result.push_str(&generated_value);
                generated_parameters.push(generated_value);
            }
        }

        Ok((generated_result, generated_parameters))
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        parameters::{
            boolean::BooleanParameter, int::IntParameter, parser::SerializableParameter,
            string::StringParameter, RandomNumberGenerator,
        },
        Logic,
    };

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

    #[test]
    fn test_populate_parameters_1() {
        let logic = Logic::try_default().unwrap();

        let rng = Box::new(MockRng::new(vec![0, 1, 2, 4]));

        let non_parameter_strs = vec![
            "red-".to_string(),
            " @nothing ".to_string(),
            " ".to_string(),
            "".to_string(),
        ];
        let parameters = vec![
            SerializableParameter::Int(IntParameter::default()),
            SerializableParameter::String(StringParameter::default()),
            SerializableParameter::Int(IntParameter::default()),
        ];

        let ret = logic.populate_parameters(non_parameter_strs, parameters, vec![], Some(rng));
        assert!(ret.is_ok());

        let (generated_string, generated_parameters) = ret.unwrap();
        assert_eq!(
            vec!["5".to_string(), "CEABCE".to_string(), "5".to_string()],
            generated_parameters
        );
        assert_eq!("red-5 @nothing CEABCE 5", generated_string);
    }

    #[test]
    fn test_populate_parameters_2() {
        let logic = Logic::try_default().unwrap();

        let rng = Box::new(MockRng::new(vec![2, 3, 4, 7]));

        let non_parameter_strs = vec![
            "ls ".to_string(),
            " ".to_string(),
            " ".to_string(),
            " ".to_string(),
            "".to_string(),
        ];
        let parameters = vec![
            SerializableParameter::Int(IntParameter::default()),
            SerializableParameter::Int(IntParameter::default()),
            SerializableParameter::String(StringParameter::default()),
            SerializableParameter::Boolean(BooleanParameter {}),
        ];

        let ret = logic.populate_parameters(non_parameter_strs, parameters, vec![], Some(rng));
        assert!(ret.is_ok());

        let (generated_string, generated_parameters) = ret.unwrap();
        assert_eq!(
            vec![
                "7".to_string(),
                "8".to_string(),
                "HCDEHCDEH".to_string(),
                "false".to_string()
            ],
            generated_parameters
        );
        assert_eq!("ls 7 8 HCDEHCDEH false", generated_string);
    }

    #[test]
    fn test_populate_parameters_blank_1() {
        let logic = Logic::try_default().unwrap();

        let rng = Box::new(MockRng::new(vec![2, 0]));

        let non_parameter_strs = vec![
            "ls ".to_string(),
            " ".to_string(),
            " ".to_string(),
            " ".to_string(),
            " ".to_string(),
            "".to_string(),
        ];
        let parameters = vec![
            SerializableParameter::Blank,
            SerializableParameter::Int(IntParameter::default()),
            SerializableParameter::Blank,
            SerializableParameter::Boolean(BooleanParameter {}),
            SerializableParameter::Blank,
        ];
        let blank_params_values = vec![
            "value1".to_string(),
            "value2".to_string(),
            "value3".to_string(),
        ];

        let ret = logic.populate_parameters(
            non_parameter_strs,
            parameters,
            blank_params_values,
            Some(rng),
        );
        assert!(ret.is_ok());

        let (generated_string, generated_parameters) = ret.unwrap();
        assert_eq!(
            vec![
                "value1".to_string(),
                "7".to_string(),
                "value2".to_string(),
                "false".to_string(),
                "value3".to_string(),
            ],
            generated_parameters
        );
        assert_eq!("ls value1 7 value2 false value3", generated_string);
    }

    #[test]
    fn test_populate_parameters_no_parameters() {
        let logic = Logic::try_default().unwrap();

        let rng = Box::new(MockRng::new(vec![0, 1, 2, 4]));

        let non_parameter_strs = vec!["some string".to_string()];
        let parameters = vec![];

        let ret = logic.populate_parameters(non_parameter_strs, parameters, vec![], Some(rng));
        assert!(ret.is_ok());

        let (generated_string, generated_parameters) = ret.unwrap();
        assert_eq!(generated_parameters.len(), 0);
        assert_eq!("some string", generated_string);
    }

    #[test]
    fn test_populate_parameters_empty() {
        let logic = Logic::try_default().unwrap();

        let rng = Box::new(MockRng::new(vec![0, 1, 2, 4]));

        let non_parameter_strs = vec![];
        let parameters = vec![];

        let ret = logic.populate_parameters(non_parameter_strs, parameters, vec![], Some(rng));
        assert!(ret.is_ok());

        let (generated_string, generated_parameters) = ret.unwrap();
        assert_eq!(generated_parameters.len(), 0);
        assert_eq!("", generated_string);
    }

    #[test]
    fn test_populate_parameters_blank_missing() {
        let logic = Logic::try_default().unwrap();

        let rng = Box::new(MockRng::new(vec![2, 0]));

        let non_parameter_strs = vec!["ls ".to_string(), "".to_string()];
        let parameters = vec![SerializableParameter::Blank];
        let blank_params_values = vec![];

        let ret = logic.populate_parameters(
            non_parameter_strs,
            parameters,
            blank_params_values,
            Some(rng),
        );
        assert!(ret.is_err());
    }
}
