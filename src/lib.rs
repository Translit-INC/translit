#![allow(dead_code)]
#![allow(unused)]
#![allow(clippy::new_without_default)]

pub mod error;
pub mod ir;

mod assembly_generator;

pub use assembly_generator::error::{AssemblyGenerationError, AssemblyGenerationResult};
pub use assembly_generator::{generate_assembly, Architecture};
pub use error::{TranslitError, TranslitResult};
pub use ir::builder::IRBuilder;
pub use ir::instruction::{Arg, Instruction, InstructionCode, InstructionOuput};
pub use ir::types::{
    Function, FunctionID, Label, Literal, Signature, Type, VarAssignable, Variable,
};
pub use ir::IR;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gen_ir() -> TranslitResult<()> {
        let mut builder = IRBuilder::new();
        let mut var = builder.create_var(Type::i8);
        let _main_func = builder.start_function(&Signature::new(&[], Type::none))?;

        let output = builder.push(
            InstructionCode::ADD,
            vec![Literal::int8(1).into(), Literal::int8(2).into()],
        )?;

        builder.set_var(&mut var, output)?;
        builder.set_var(&mut var, Literal::int8(4))?;

        builder.push(
            InstructionCode::SUB,
            vec![Literal::int8(3).into(), var.into()],
        )?;

        builder.end_function()?;

        let ir = builder.build()?;
        ir.print();

        Ok(())
    }

    #[test]
    fn test_interpreter() -> TranslitResult<()> {
        let mut builder = IRBuilder::new();
        let _test_func = builder.start_function(&Signature::new(&[], Type::none))?;

        builder.push(
            InstructionCode::MUL,
            vec![Literal::int8(6).into(), Literal::int8(9).into()],
        )?;

        builder.push(
            InstructionCode::ADD,
            vec![Literal::int8(6).into(), Literal::int8(9).into()],
        )?;

        builder.end_function()?;

        let _main_func = builder.start_function(&Signature::new(&[], Type::none))?;

        builder.push(
            InstructionCode::ADD,
            vec![Literal::int8(1).into(), Literal::int8(2).into()],
        )?;

        builder.push(
            InstructionCode::SUB,
            vec![Literal::int8(3).into(), Literal::int8(1).into()],
        )?;

        builder.push(InstructionCode::CALL, vec![Arg::Function(_test_func)])?;
        builder.end_function()?;

        let ir = builder.build()?;
        let asm = generate_assembly(Architecture::x86_64, ir)?;
        println!("asm: {}", asm);
        Ok(())
    }
}
