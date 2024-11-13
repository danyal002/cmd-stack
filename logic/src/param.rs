use rand::Rng;
use regex::Regex;
use thiserror::Error;

pub trait Parameter {
    fn generate_random(&self) -> String;
}

pub struct StringParameter {}
impl Parameter for StringParameter {
    fn generate_random(&self) -> String {
        let charset: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                               abcdefghijklmnopqrstuvwxyz\
                               0123456789";
        let mut rng = rand::thread_rng();
        let random_string: String = (0..10)
            .map(|_| {
                let idx = rng.gen_range(0..charset.len());
                charset[idx] as char
            })
            .collect();
        random_string
    }
}

pub struct IntParameter {}
impl Parameter for IntParameter {
    fn generate_random(&self) -> String {
        let mut rng = rand::thread_rng();
        let random_int: i32 = rng.gen_range(0..10000);
        random_int.to_string()
    }
}

#[derive(Error, Debug)]
pub enum ParameterError {
    #[error("error parsing parameters from string")]
    Parsing,
    #[error("error invalid parameter")]
    InvalidParameter,
}

pub fn parse_parameter(s: String) -> Result<Box<dyn Parameter>, ParameterError> {
    match s.as_str() {
        "@string" => Ok(Box::new(StringParameter {})),
        "@int" => Ok(Box::new(IntParameter {})),
        _ => Err(ParameterError::InvalidParameter),
    }
}

pub fn validate_parameters(s: String) -> Result<(), ParameterError> {
    let _ = replace_parameters(s)?;
    Ok(())
}

pub fn replace_parameters(s: String) -> Result<String, ParameterError> {
    let re = Regex::new(r"\{([^}]*)\}").map_err(|_| ParameterError::Parsing)?;
    let result = re.replace_all(&s, |caps: &regex::Captures| {
        let param_str = &caps[1];
        let param = parse_parameter(param_str.to_owned()).unwrap();
        param.generate_random()
    });
    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::replace_parameters;
    use super::validate_parameters;

    #[test]
    fn test_zero_parameters() {
        let ret = validate_parameters("fasd @email @wadsf @test".to_string());
        assert!(ret.is_ok());
    }

    #[test]
    fn test_replace_parameters() {
        let ret = replace_parameters("red-{@int} @nothing {@string} {@int}".to_string());
        assert!(ret.is_ok());
        println!("{}", ret.unwrap());
    }
}
