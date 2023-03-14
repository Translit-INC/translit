pub use instruction::{Arg, Instruction, InstructionCode};
pub use types::{Block, BlockID, Function, FunctionID, Literal, Signature, Type, Variable};
pub mod builder;
pub mod instruction;
pub mod types;
pub mod generator;

pub use builder::IRBuilder;
