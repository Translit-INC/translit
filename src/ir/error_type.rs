use std::fmt;

pub trait TranslitError {
    fn description(&self) -> &String;
    fn cause(&self) -> &String;
}

#[derive(Debug, Clone)]
pub struct FunctionError {
    pub(crate) desc: String,
    pub(crate) cause: String
}

impl FunctionError {
    pub(crate) fn new(cause: &str, description: &str) -> FunctionError {
        FunctionError {
            desc: description.to_string(),
            cause: cause.to_string()
        }
    }
}

impl TranslitError for FunctionError {
    fn cause(&self) -> &String {
        &self.cause
    }

    fn description(&self) -> &String {
        &self.desc
    }
}

impl fmt::Display for FunctionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FunctionErro: \n\tCause: {}\n\tDescription: {}", self.cause(), self.description())
    }
}
