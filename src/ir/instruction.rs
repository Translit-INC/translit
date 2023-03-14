use num_derive::FromPrimitive;

use super::types::{FunctionID, Label, Literal, Variable};

/// An instruction in the IR
#[derive(Debug, Clone)]
pub struct Instruction(pub(crate) u64, pub(crate) Vec<Arg>);

/// Argument passed to an instruction
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Arg {
    /// Variable
    Var(Variable),
    /// Literal
    Literal(Literal),
    /// Function
    Function(FunctionID),
    /// Basic block
    Label(Label),
}

impl From<Variable> for Arg {
    fn from(value: Variable) -> Self {
        Arg::Var(value)
    }
}

impl From<Literal> for Arg {
    fn from(value: Literal) -> Self {
        Arg::Literal(value)
    }
}

impl From<Label> for Arg {
    fn from(value: Label) -> Self {
        Arg::Label(value)
    }
}

impl From<FunctionID> for Arg {
    fn from(value: FunctionID) -> Self {
        Arg::Function(value)
    }
}

/// Instruction Code
#[repr(u64)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, FromPrimitive)]
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
    /// return from function
    RET,
}
