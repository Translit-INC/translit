use std::error;
use std::fmt;

use crate::Arg;
use crate::AssemblyGenerationError;

pub type TranslitResult<T> = Result<T, TranslitError>;

#[derive(Debug, Clone)]
pub enum TranslitError {
    FunctionStartError,
    FunctionEndError,
    UnendedFunctionError,

    RetOutsideFuncError,
    InstrParamLenError,
    InvalidParamError(Arg),
    InvalidTypeError(Arg),

    DivideByZeroError,
    CalledMainFunction,
    FunctionNotFound,
    CallOutsideFunction,

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
            TranslitError::InvalidParamError(arg) => {
                write!(f, "Did not expect to see {} there", arg.format())
            }
            TranslitError::InvalidTypeError(arg) => {
                write!(f, "The type of {} is wrong", arg.format())
            }
            TranslitError::AssemblyGenerationError(info) => {
                write!(f, "Error generating assembly: {}", info)
            }
            TranslitError::DivideByZeroError => write!(f, "Cannot divide by zero."),
            TranslitError::CalledMainFunction => write!(f, "Cannot call main function itself."),
            TranslitError::FunctionNotFound => write!(f, "Function not found (in call instruction)"),
            TranslitError::CallOutsideFunction => write!(f, "Tried to call function outside a function")
        }
    }
}

impl error::Error for TranslitError {}
