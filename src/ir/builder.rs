use super::instruction::{Arg, Instruction, InstructionCode};
use super::types::{Function, FunctionID, Label, Signature, Variable};
use super::IR;
use crate::error::{TranslitError, TranslitResult};

/// IR Builder
#[derive(Debug, Clone, Default)]
pub struct IRBuilder {
    instructions: Vec<Instruction>,
    functions: Vec<Function>,
    labels: Vec<Label>,
}

impl IRBuilder {
    /// Create a new builder
    pub fn new() -> Self {
        IRBuilder {
            instructions: Vec::new(),
            functions: Vec::new(),
            labels: Vec::new(),
            // generated_assembly: None,
        }
    }

    /// Returns the build IR
    pub fn build(self) -> TranslitResult<IR> {
        if let Some(Function { end: None, .. }) = self.functions.last() {
            return Err(TranslitError::UnendedFunctionError);
        }
        Ok(IR {
            instructions: self.instructions,
            functions: self.functions,
            labels: self.labels,
        })
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

    /// Inserts a label
    pub fn insert_label(&mut self) -> Label {
        let label = Label(self.instructions.len());
        self.labels.push(label);
        Label(self.labels.len() - 1)
    }

    /// Verify the instruction arguments
    pub fn verify(&self, instr: &Instruction) -> TranslitResult<()> {
        let mut instr_args = instr.1.to_vec();
        instr_args.retain(|&x| x != Arg::NONE);
        let params_err = |length: usize| {
            (instr_args.len() == length)
                .then_some(())
                .ok_or(TranslitError::InstrParamLenError)
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
                if let Some(Function { end: Some(_), .. }) | None = self.functions.last() {
                    return Err(TranslitError::RetOutsideFuncError);
                }
                params_err(1)
            }
            InstructionCode::END => params_err(0),
        }
    }

    /// Push an instruction into the IR. Returns an error if a RET instruction is passed outside a function.
    pub fn push<const N: usize>(
        &mut self,
        code: InstructionCode,
        args: [Arg; N],
    ) -> TranslitResult<Variable> {
        let instr = match args.as_slice() {
            [] => Instruction::new(code, [Arg::NONE; 3]),
            &[a] => Instruction::new(code, [a, Arg::NONE, Arg::NONE]),
            &[a, b] => Instruction::new(code, [a, b, Arg::NONE]),
            &[a, b, c] => Instruction::new(code, [a, b, c]),
            _ => panic!("Too many arguments"),
        };
        self.verify(&instr)?;
        self.instructions.push(instr);

        Ok(Variable(self.instructions.len() - 1))
    }
}
