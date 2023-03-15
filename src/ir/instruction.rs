use super::types::{FunctionID, Label, Literal, Variable};

/// An instruction in the IR
#[derive(Debug, Clone)]
pub struct Instruction(pub(crate) InstructionCode, pub(crate) Vec<Arg>);

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

impl Arg {
    pub(crate) fn format(&self) -> String {
        match self {
            Arg::Label(Label(id)) => format!("l{id}"),
            Arg::Function(FunctionID(id)) => format!("f{id}"),
            Arg::Var(Variable(id)) => format!("v{id}"),
            Arg::Literal(Literal(t, a)) => format!("{a}{t:?}"),
        }
    }
}

/// Instruction Code
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InstructionCode {
    /// a + b
    ADD,
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
    /// a >> b
    SHL,
    /// a << b
    SHR,
    /// a == b
    EQ,
    /// a >= b
    CMPEQ,
    /// a > b
    CMP,
    /// JUMP instruction
    JMP,
    /// CALL instruction
    CALL,
    /// return from function
    RET,
}
