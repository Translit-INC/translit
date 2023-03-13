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

#[derive(Debug, Clone)]
/// a function
pub struct Function {
    pub(crate) id: usize,
    pub(crate) sig: Signature,
    pub(crate) end: Option<usize> // this hold the index of the last instruction of the function
}

impl Function {
    pub(crate) fn new(id: usize, sig: Signature) -> Self {
        Function {
            id,
            sig,
            end: None
        }
    }
}

#[derive(Debug, Clone)]
/// Function Signature
pub struct Signature {
    pub(crate) params: Vec<Type>,
    pub(crate) returns: Type,
}

impl Signature {
    pub fn new(params: &[Type], returns: Type) -> Self {
        Self {
            params: params.to_vec(),
            returns,
        }
    }
}
