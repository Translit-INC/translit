#![allow(dead_code)]
#![allow(unused)]

pub mod error;
pub mod ir;

mod assembly_generator;

pub use assembly_generator::error::{AssemblyGenerationError, AssemblyGenerationResult};
pub use assembly_generator::{generate_assembly, Architecture};
pub use error::{TranslitError, TranslitResult};
pub use ir::builder::IRBuilder;
pub use ir::instruction::{Arg, Instruction, InstructionCode};
pub use ir::types::{Function, FunctionID, Label, Literal, Signature, Type, Variable};
pub use ir::IR;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn asm_gen() -> TranslitResult<()> {
        let mut builder = IRBuilder::new();
        let main_func = builder.start_function(&Signature::new(&[], Type::NONE))?;

        builder.push(
            InstructionCode::ADD,
            [Literal::int8(1).into(), Literal::int8(2).into()],
        )?;
        builder.push(
            InstructionCode::SUB,
            [Literal::int8(3).into(), Literal::int8(1).into()],
        )?;

        builder.end_function()?;
        generate_assembly(Architecture::x86_64, builder.build()?)?;
        Ok(())
    }
}
