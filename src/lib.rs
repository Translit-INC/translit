#![allow(dead_code)]
#![allow(unused)]

extern crate num;
#[macro_use]
extern crate num_derive;

pub mod error;
pub mod ir;

pub(crate) mod assembly_generator;

#[cfg(test)]
mod tests {
    use crate::error::{TranslitError, TranslitResult};
    use crate::ir::{IRBuilder, Signature, Instruction, InstructionCode, Type, Arg, Literal};

    #[test]
    fn asm_gen() {
        let mut builder = IRBuilder::new();
        let main_func = builder.start_function(&Signature::new(&[Type::NONE], Type::NONE));

        builder.push(InstructionCode::ADD, [Arg::Literal(Literal::int8(1)), Arg::Literal(Literal::int8(2))]);
        builder.push(InstructionCode::SUB, [Arg::Literal(Literal::int8(3)), Arg::Literal(Literal::int8(1))]);

        builder.end_function().unwrap();

        builder.generate_assembly();
    }
}
