use super::error::{TranslitError, TranslitResult};
pub use instruction::{Arg, Instruction, InstructionCode};
pub use types::{Block, BlockID, Function, FunctionID, Literal, Signature, Type, Variable};

pub mod builder;
pub mod instruction;
pub mod types;
pub use builder::IRBuilder;

#[cfg(test)]
mod tests {

    use super::{Function, IRBuilder, InstructionCode, Signature};

    #[test]
    /// Starting a function test
    fn start_function() {
        let mut builder = IRBuilder::new();
        let sig = Signature::new(&[], super::Type::NONE);
        match builder.make_function(&sig, || {}) {
            Ok(_) => {}
            Err(e) => println!("{}", e),
        }
    }
}
