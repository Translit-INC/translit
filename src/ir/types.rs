use crate::InstructionOuput;

/// Type of a literal or variable
#[repr(u64)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[allow(non_camel_case_types)]
pub enum Type {
    /// 1 byte (8-bit)
    Byte = 8,
    /// 2 bytes (16-bit)
    Word = 16,
    /// 4 bytes (32-bit)
    DWord = 32,
    /// 8 bytes (64-bit)
    QWord = 64,
    /// None
    #[default]
    none = 0,
}

/// A literal value
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Literal(pub(crate) Type, pub(crate) u64); // Actual value

impl Literal {
    pub fn int8(n: i8) -> Literal {
        Literal(Type::Byte, n as _)
    }

    pub fn int16(n: i16) -> Literal {
        Literal(Type::Word, n as _)
    }

    pub fn int32(n: i32) -> Literal {
        Literal(Type::DWord, n as _)
    }

    pub fn int64(n: i64) -> Literal {
        Literal(Type::QWord, n as _)
    }
}

/// A variable
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Variable(pub(crate) Type, pub(crate) usize); // index of builder.memory

/// Something which can be assigned to a variable
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VarAssignable {
    Literal(Literal),
    InstOut(InstructionOuput),
}

impl From<Literal> for VarAssignable {
    fn from(value: Literal) -> Self {
        VarAssignable::Literal(value)
    }
}

impl From<InstructionOuput> for VarAssignable {
    fn from(value: InstructionOuput) -> Self {
        VarAssignable::InstOut(value)
    }
}

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

impl From<Function> for FunctionID {
    fn from(source: Function) -> Self {
        Self(source.start)
    }
}

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
