use std::error;
use std::fmt;

pub type TranslitResult<T> = Result<T, TranslitError>;

#[derive(Debug, Clone)]
pub enum TranslitError {
    FunctionStartError,
    FunctionEndError,
    RetOutsideFuncError,
    BlockStartError,
    BlockEndError,
}

impl fmt::Display for TranslitError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TranslitError::FunctionStartError => write!(f, "Cannot start a new function"),
            TranslitError::FunctionEndError => write!(f, "There's no on going function to end it."),
            TranslitError::RetOutsideFuncError => write!(f, "Cannot return outside a function"),
            TranslitError::BlockStartError => write!(f, "Cannot start a new block"),
            TranslitError::BlockEndError => write!(f, "Cannot end the block"),
        }
    }
}

impl error::Error for TranslitError {}