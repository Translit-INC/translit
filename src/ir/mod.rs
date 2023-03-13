use super::error::{TranslitError, TranslitResult};
pub use instruction::{Arg, Instruction, InstructionCode};
pub use types::{Block, BlockID, Function, FunctionID, Literal, Signature, Type, Variable};

pub mod instruction;
pub mod types;

/// IR Builder
#[derive(Debug, Clone, Default)]
pub struct IRBuilder {
    instructions: Vec<Instruction>,
    functions: Vec<Function>,
    blocks: Vec<Block>,
}

impl IRBuilder {
    /// Create a new builder
    pub fn new() -> Self {
        IRBuilder {
            instructions: Vec::new(),
            functions: Vec::new(),
            blocks: Vec::new(),
        }
    }

    /// Start a function.
    /// Every instruction will be placed inside this function till you call `end_function`.
    /// Returns an error if a function is already going on
    pub fn start_function(&mut self, sig: Signature) -> TranslitResult<FunctionID> {
        if let Some(Function { end: Some(_), .. }) = self.functions.last() {
            let f = Function {
                id: self.functions.len(),
                start: self.instructions.len(),
                end: None,
                sig,
            };
            self.functions.push(f);
            Ok(FunctionID(self.functions.len() - 1))
        } else {
            Err(TranslitError::FunctionEndError)
        }
    }

    /// End the ongoing function. Returns an error if there is no function ongoing
    pub fn end_function(&mut self) -> TranslitResult<()> {
        let Some(Function { end: end @ None, .. }) = self.functions.last_mut() else {
            return Err(TranslitError::FunctionEndError);
        };

        *end = Some(self.instructions.len()); // then END instruction we just pushed

        self.push(InstructionCode::END, []).unwrap();
        Ok(())
    }

    /// Start a basic block.
    /// Returns an error if another basic block is still ongoing
    pub fn start_block(&mut self) -> TranslitResult<BlockID> {
        if let Some(Block { end: Some(_), .. }) = self.blocks.last() {
            let block = Block {
                id: self.blocks.len(),
                start: self.instructions.len(),
                end: None,
            };
            self.blocks.push(block);
            Ok(BlockID(self.blocks.len() - 1))
        } else {
            Err(TranslitError::BlockEndError)
        }
    }

    /// End the ongoing basic block. Returns an error if there is no basic block ongoing
    pub fn end_block(&mut self) -> TranslitResult<()> {
        let Some(Block { end: end @ None, .. }) = self.blocks.last_mut() else {
            return Err(TranslitError::BlockEndError);
        };

        *end = Some(self.instructions.len());
        Ok(())
    }

    /// Push an instruction into the IR. Returns an error if a RET instruction is passed outside a function.
    pub fn push<const N: usize>(
        &mut self,
        code: InstructionCode,
        args: [Arg; N],
    ) -> TranslitResult<Variable> {
        if let (Some(Function { end: Some(_), .. }), InstructionCode::RET) =
            (self.functions.last(), code)
        {
            return Err(TranslitError::RetOutsideFuncError);
        }
        match args.as_slice() {
            [] => self
                .instructions
                .push(Instruction::new(code, [Arg::NONE; 3])),
            &[a] => self
                .instructions
                .push(Instruction::new(code, [a, Arg::NONE, Arg::NONE])),
            &[a, b] => self
                .instructions
                .push(Instruction::new(code, [a, b, Arg::NONE])),
            &[a, b, c] => self.instructions.push(Instruction::new(code, [a, b, c])),
            _ => panic!("Too many arguments"),
        };

        Ok(Variable(self.instructions.len() - 1))
    }
}
