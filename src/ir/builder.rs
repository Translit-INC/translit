use super::instruction::{Arg, Instruction, InstructionCode};
use super::types::{Block, BlockID, Function, FunctionID, Signature, Variable};
use crate::error::{TranslitError, TranslitResult};

/// IR Builder
#[derive(Debug, Clone, Default)]
pub struct IRBuilder {
    pub(crate) instructions: Vec<Instruction>,
    pub(crate) functions: Vec<Function>,
    pub(crate) blocks: Vec<Block>,

    // i'll see what to do with this     -- wizard
    // pub(crate) generated_assembly: Option<String>,
}

impl IRBuilder {
    /// Create a new builder
    pub fn new() -> Self {
        IRBuilder {
            instructions: Vec::new(),
            functions: Vec::new(),
            blocks: Vec::new(),
            // generated_assembly: None,
        }
    }

    /// Start a function.
    /// Every instruction after this function call will be considered as the part of the function until end_function is called.
    /// Returns an error if a function is already going on
    pub fn start_function(&mut self, sig: &Signature) -> TranslitResult<FunctionID> {
        let f = Function {
            start: self.instructions.len(),
            end: None,
            sig: sig.to_owned(),
        };
        self.functions.push(f);
        Ok(FunctionID(self.functions.len() - 1))
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
                start: self.instructions.len(),
                end: None,
            };
            self.blocks.push(block);
            Ok(BlockID(self.blocks.len() - 1))
        } else {
            Err(TranslitError::BlockEndError)
        }
    }
    /// Verify the instruction
    pub fn verify(&self, instr: &Instruction) -> TranslitResult<()> {
        let mut instr_args = instr.1.to_vec();
        instr_args.retain(|&x| x != Arg::NONE);
        let params_err = |length: usize| {
            if instr_args.len() != length {
                return TranslitResult::Err(TranslitError::InstrParamLenError);
            }
            return TranslitResult::Ok(());
        };
        match num::FromPrimitive::from_u64(instr.0).unwrap() {
            InstructionCode::ADD
            | InstructionCode::SUB
            | InstructionCode::MUL
            | InstructionCode::DIV
            | InstructionCode::MOD => params_err(2),

            InstructionCode::CMP => params_err(2),
            InstructionCode::AND => params_err(2),
            InstructionCode::OR => params_err(2),
            InstructionCode::NOT => params_err(1),
            InstructionCode::EQ => params_err(2),
            InstructionCode::CMPEQ => params_err(2),
            InstructionCode::RET => {
                if let Some(Function { end: Some(_), .. }) = self.functions.last() {
                    return Err(TranslitError::RetOutsideFuncError);
                }
                params_err(1)
            }
            InstructionCode::END => params_err(0),
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
        match args.as_slice() {
            [] => {
                let instr = Instruction::new(code, [Arg::NONE; 3]);
                self.verify(&instr)?;
                self.instructions.push(instr)
            }
            &[a] => {
                let instr = Instruction::new(code, [a, Arg::NONE, Arg::NONE]);
                self.verify(&instr)?;
                self.instructions.push(instr)
            }
            &[a, b] => {
                let instr = Instruction::new(code, [a, b, Arg::NONE]);
                self.verify(&instr)?;
                self.instructions.push(instr)
            }
            &[a, b, c] => {
                let instr = Instruction::new(code, [a, b, c]);
                self.verify(&instr)?;
                self.instructions.push(instr)
            }
            _ => panic!("Too many arguments"),
        };

        Ok(Variable(self.instructions.len() - 1))
    }
}
