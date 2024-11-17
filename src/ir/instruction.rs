

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
    /// a << b
    SHL,
    /// a >> b
    SHR,
    /// a == b
    EQ,
    /// a >= b
    CMPEQ,
    /// a > b
    CMP,
    /// Jump to label
    JMP,
    /// call a function
    CALL,
    /// Jump to label if condition is true
    JMPIF,
    /// return from function
    RET,
    /// Insert a PHI node
    PHI,
    /// Push something to memory
    PUSH,
    /// variable
    VAR,
    /// change the value
    SET
}
