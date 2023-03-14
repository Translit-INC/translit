use super::types::{BlockID, FunctionID, Literal, Variable};

/// An instruction in the IR
#[derive(Debug, Clone)]
pub struct Instruction(pub(crate) u64, pub(crate) [Arg; 3]);

impl Instruction {
    /// Create a new instruction
    pub fn new(code: InstructionCode, args: [Arg; 3]) -> Instruction {
        Instruction(code as u64, args)
    }

}

/// Argument passed to an instruction
#[derive(Debug, Clone, Default, Copy, PartialEq)]
pub enum Arg {
    /// Variable
    Var(Variable),
    /// Literal
    Literal(Literal),
    /// Function
    Function(FunctionID),
    /// Basic block
    Block(BlockID),
    /// None
    #[default]
    NONE,
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

impl From<BlockID> for Arg {
    fn from(value: BlockID) -> Self {
        Arg::Block(value)
    }
}

impl From<FunctionID> for Arg {
    fn from(value: FunctionID) -> Self {
        Arg::Function(value)
    }
}

/// Instruction Code
#[repr(u64)]

#[derive(FromPrimitive)]
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
    /// return from function
    RET,

    /// end instruction for blocks
    END,
}
