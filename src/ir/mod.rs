pub use instruction::{Arg, Instruction, InstructionCode};
pub use types::{Block, BlockID, Function, FunctionID, Literal, Signature, Type, Variable};

pub mod builder;
pub mod instruction;
pub mod types;
pub use builder::IRBuilder;

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    /// Starting a function test
    fn make_function() {
        let mut builder = IRBuilder::new();
        let sig = Signature::new(&[], Type::NONE);
        let _f = builder.start_function(&sig).unwrap();
        builder
            .push(
                InstructionCode::ADD,
                [Literal::int16(12).into(), Literal::int16(43).into()],
            )
            .unwrap();
        builder.end_function().unwrap();
    }
}
