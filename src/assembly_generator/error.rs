use std::error;
use std::fmt;

pub type AssemblyGenerationResult<T> = Result<T, AssemblyGenerationError>;

#[derive(Debug, Clone)]
pub enum AssemblyGenerationError {
    NoMainFunction,
}

impl fmt::Display for AssemblyGenerationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AssemblyGenerationError::NoMainFunction => write!(f, "No main function found"),
        }
    }
}

impl error::Error for AssemblyGenerationError {}
