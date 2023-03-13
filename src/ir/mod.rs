pub use instruction::{Arg, Instruction, InstructionCode};
pub use types::{Literal, Type, Variable};

use self::{types::{Function, Signature}, error_type::{FunctionError, TranslitError}};

pub mod instruction;
pub mod types;
pub mod error_type;

/// IR Builder
#[derive(Debug, Clone, Default)]
pub struct IRBuilder {
    instructions: Vec<Instruction>,
    funcs: Vec<Function>,
}

impl IRBuilder {
    /// Create a new builder
    pub fn new() -> Self {
        IRBuilder {
            instructions: Vec::new(),
            funcs: Vec::new(),
        }
    }

    /// Start a function (every instruction will be placed inside this function till you call end_function)
    pub fn start_function(&mut self, sig: Signature) -> Result<Function, impl TranslitError> {
        let lst_fn = self.funcs.last();

        if self.funcs.len() != 0 && lst_fn.unwrap().end.is_none() {
            return Err(FunctionError::new("Cannot start another function", "A function is already ongoing"));
        }

        let f = Function::new(self.instructions.len(), sig);
        self.funcs.push(f.clone());

        Ok(f)
    }

    pub fn end_function(&mut self, func: &mut Function) -> Result<(), impl TranslitError> {
        let lst_fn = self.funcs.last();

        if self.funcs.len() != 0 && lst_fn.unwrap().end.is_some() {
            return Err(FunctionError::new("Cannot end a function", "This function is already ended"));
        }

        self.instructions.push(Instruction::new(InstructionCode::END, [Arg::NONE, Arg::NONE, Arg::NONE]));

        func.end = Some(self.instructions.len() - 1); // then END instruction we just pushed

        Ok(())
    }

    /// Push an instruction into the IR
    pub fn push<const N: usize>(&mut self, code: InstructionCode, args: [Arg; N]) -> Variable {
        match N {
            0 => self
                .instructions
                .push(Instruction::new(code, [Arg::NONE; 3])),
            1 => self
                .instructions
                .push(Instruction::new(code, [args[0], Arg::NONE, Arg::NONE])),
            2 => self
                .instructions
                .push(Instruction::new(code, [args[0], args[1], Arg::NONE])),
            3 => self.instructions.push(Instruction::new(code, unsafe {
                std::mem::transmute_copy(&args)
            })),
            _ => panic!("Too many arguments"),
        };

        Variable(self.instructions.len() - 1)
    }
}
