use builder::IRBuilder;
use instruction::{Arg, Instruction, InstructionCode};
use types::{Function, FunctionID, Label, Literal, Signature, Type, Variable};

pub mod builder;
pub mod instruction;
pub mod types;
// pub mod generator;

pub struct IR {
    pub(crate) instructions: Vec<Instruction>,
    pub(crate) functions: Vec<Function>,
    pub(crate) labels: Vec<Label>,
}
