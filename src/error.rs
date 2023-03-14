use std::error;
use std::fmt;

use crate::AssemblyGenerationError;

pub type TranslitResult<T> = Result<T, TranslitError>;

#[derive(Debug, Clone)]
pub enum TranslitError {
    FunctionStartError,
    FunctionEndError,
    UnendedFunctionError,

    RetOutsideFuncError,
    InstrParamLenError,

    AssemblyGenerationError(AssemblyGenerationError),
}

impl fmt::Display for TranslitError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TranslitError::FunctionStartError => write!(f, "Cannot start a new function"),
            TranslitError::FunctionEndError => write!(f, "There's no on going function to end it."),
            TranslitError::UnendedFunctionError => {
                write!(f, "An ongoing function has not been ended yet")
            }
            TranslitError::RetOutsideFuncError => write!(f, "Cannot return outside a function"),
            TranslitError::InstrParamLenError => write!(
                f,
                "The number of parameters passed to instruction are incorrect."
            ),
            TranslitError::AssemblyGenerationError(info) => {
                write!(f, "Error generating assembly: {}", info)
            }
        }
    }
}

impl error::Error for TranslitError {}
