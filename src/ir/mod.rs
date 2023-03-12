use std::ops::Range;

pub use instruction::{Arg, Instruction, InstructionCode};
pub use types::{Type, Literal, Variable};

use self::types::{Function, Signature};

pub mod instruction;
pub mod types;

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
            funcs: Vec::new()
        }
    }
    pub fn create_function(&mut self, sig: Signature) -> Function {
        let f = Function {
            id: self.funcs.len() - 1,
            sig,
        };
        self.funcs.push(f.clone());
        f
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
