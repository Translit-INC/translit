/// Type of a literal or variable
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Type {
    /// 8-bit integer
    I8,
    /// 16-bit integer
    I16,
    /// 32-bit integer
    I32,
    /// 64-bit integer
    I64,
    /// None
    #[default]
    NONE,
}

/// A literal value
#[derive(Debug, Clone, Copy)]
pub struct Literal(pub(crate) Type, pub(crate) u64); // Actual value

/// A variable
#[derive(Debug, Clone, Copy)]
pub struct Variable(pub(crate) usize); // reference to value stored in IRBuilder
