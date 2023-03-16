/// Type of a literal or variable
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[allow(non_camel_case_types)]
pub enum Type {
    /// 8-bit integer
    i8,
    /// 16-bit integer
    i16,
    /// 32-bit integer
    i32,
    /// 64-bit integer
    i64,
    /// 1-bit boolean (not really)
    boolean,
    /// None
    #[default]
    none,
}

/// A literal value
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Literal(pub(crate) Type, pub(crate) u64); // Actual value

impl Literal {
    pub fn int8(n: i8) -> Literal {
        Literal(Type::i8, n as _)
    }

    pub fn int16(n: i16) -> Literal {
        Literal(Type::i16, n as _)
    }

    pub fn int32(n: i32) -> Literal {
        Literal(Type::i32, n as _)
    }

    pub fn int64(n: i64) -> Literal {
        Literal(Type::i64, n as _)
    }

    pub fn boolean(n: bool) -> Literal {
        Literal(Type::boolean, n as _)
    }
}

/// A variable
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Variable(pub(crate) usize); // value returned by instruction at that index

#[derive(Debug, Clone)]
/// a function
pub struct Function {
    /// this will hold the index of the first instruction of the function
    pub(crate) start: usize,
    /// this will eventually hold the index of the last instruction of the function
    pub(crate) end: Option<usize>,
    pub(crate) sig: Signature,
}

/// Reference to a function
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FunctionID(pub(crate) usize);

#[derive(Debug, Clone)]
/// Function Signature
pub struct Signature {
    pub params: Vec<Type>,
    pub returns: Type,
}

impl Signature {
    pub fn new(params: &[Type], returns: Type) -> Self {
        Self {
            params: params.to_vec(),
            returns,
        }
    }
}

/// Label
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Label(pub(crate) usize); // Instruction at which it starts
