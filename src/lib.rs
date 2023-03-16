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
    fn gen_ir() -> TranslitResult<()> {
        let mut builder = IRBuilder::new();
        let _main_func = builder.start_function(&Signature::new(&[], Type::none))?;
        builder.push(
            InstructionCode::ADD,
            vec![Literal::int8(1).into(), Literal::int8(2).into()],
        )?;
        builder.push(
            InstructionCode::SUB,
            vec![Literal::int8(3).into(), Literal::int8(1).into()],
        )?;
        builder.end_function()?;
        let ir = builder.build()?;
        ir.print();
        Ok(())
    }

    #[test]
    fn test_interpreter() -> TranslitResult<()> {
        let mut builder = IRBuilder::new();
        let _main_func = builder.start_function(&Signature::new(&[], Type::none))?;
        builder.push(
            InstructionCode::ADD,
            vec![Literal::int8(1).into(), Literal::int8(2).into()],
        )?;
        builder.push(
            InstructionCode::SUB,
            vec![Literal::int8(3).into(), Literal::int8(1).into()],
        )?;
        builder.end_function()?;
        let ir = builder.build()?;
        let asm = generate_assembly(Architecture::x86_64, ir)?;
        println!("{}", asm);
        Ok(())
    }
}
