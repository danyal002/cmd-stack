use rand::{rngs::ThreadRng, Rng};

use super::{parser::SerializableParameter, GenerateRandomValues, ParameterError};
use crate::Logic;

pub trait RandomNumberGenerator {
    fn generate_range(&mut self, low: i32, high: i32) -> i32;
}

impl RandomNumberGenerator for ThreadRng {
    fn generate_range(&mut self, low: i32, high: i32) -> i32 {
        self.gen_range(low..high + 1)
    }
}

impl Logic {
    pub fn populate_parameters(
        &self,
        non_parameter_strs: Vec<String>,
        parameters: Vec<SerializableParameter>,
    ) -> Result<(String, Vec<String>), ParameterError> {
        let mut rng = rand::rngs::ThreadRng::default();

        // Build the string by generating parameters
        let mut generated_result = String::new();
        let mut generated_parameters = Vec::new();

        for (i, other_string) in non_parameter_strs.iter().enumerate() {
            generated_result.push_str(other_string);

            if i < non_parameter_strs.len() - 1 {
                let s = parameters[i].generate_random_values(&mut rng);
                generated_result.push_str(&s);
                generated_parameters.push(s);
            }
        }

        Ok((generated_result, generated_parameters))
    }
}
