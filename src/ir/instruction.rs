use super::types::{Literal, Variable};

/// An instruction in the IR
#[derive(Debug, Clone)]
pub struct Instruction(u64, [Arg; 3]);

impl Instruction {
    /// Create a new instruction
    pub fn new(code: InstructionCode, args: [Arg; 3]) -> Instruction {
        Instruction(code as u64, args)
    }
}

/// Argument passed to an instruction
#[derive(Debug, Clone, Default, Copy)]
pub enum Arg {
    /// Variable
    Var(Variable),
    /// Literal
    Literal(Literal),
    /// None
    #[default]
    NONE,
}

/// Instruction Code
#[repr(u64)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InstructionCode {
    /// a + b
    ADD = 0x01,
    /// a - b
    SUB,
    /// a * b
    MUL,
    /// a / b
    DIV,
    /// a % b
    MOD,
    /// a & b
    AND,
    /// a | b
    OR,
    /// !a
    NOT,
    /// a == b
    EQ,
    /// a >= b
    CMPEQ,
    /// a > b
    CMP,

    /// end instruction for blocks
    END,
}
