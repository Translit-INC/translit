pub use instruction::{Arg, Instruction, InstructionCode};
pub use types::{Block, BlockID, Function, FunctionID, Literal, Signature, Type, Variable};

pub mod builder;
pub mod instruction;
pub mod types;
pub use builder::IRBuilder;

#[cfg(test)]
mod tests {

    use crate::error::TranslitResult;

    use super::*;

    #[test]
    /// Starting a function test
    fn make_function() {
        let mut builder = IRBuilder::new();
        let sig = Signature::new(&[], Type::NONE);
        match builder.make_function(&sig, |builder| {
            builder.push(InstructionCode::ADD, [])?; // Test
            TranslitResult::Ok(())
        }) {
            Ok(_) => {},
            Err(e) => println!("{}", e)
        };
    }
}
