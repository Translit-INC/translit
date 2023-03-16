use super::instruction::{Arg, Instruction, InstructionCode as IC};
use super::types::{Function, FunctionID, Label, Signature, Variable};
use super::IR;
use crate::error::{TranslitError, TranslitResult};
use crate::Literal;

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

        *end = Some(self.instructions.len());
        self.instructions.push(Instruction(IC::RET, vec![]));
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
        let params_err = |length: usize| {
            (instr.1.len() == length)
                .then_some(())
                .ok_or(TranslitError::InstrParamLenError)
        };

        let same_typed = || match instr.1.as_slice() {
            &[Arg::Literal(Literal(t1, _)), a @ Arg::Literal(Literal(t2, _))] => (t1 == t2)
                .then_some(())
                .ok_or(TranslitError::InvalidTypeError(a)),
            _ => Err(TranslitError::InvalidParamError(instr.1[0])),
        };

        match instr.0 {
            IC::ADD
            | IC::SUB
            | IC::MUL
            | IC::MOD
            | IC::CMPEQ
            | IC::EQ
            | IC::CMP
            | IC::AND
            | IC::SHL
            | IC::SHR
            | IC::OR => params_err(2).and_then(|_| same_typed()),

            IC::DIV => params_err(2).and_then(|_| same_typed()).and_then(|_| {
                if let Arg::Literal(Literal(_, 0)) = instr.1[1] {
                    Err(TranslitError::DivideByZeroError)
                } else {
                    Ok(())
                }
            }),

            IC::NOT => params_err(1).and_then(|_| {
                matches!(instr.1[0], Arg::Literal(_))
                    .then_some(())
                    .ok_or(TranslitError::InvalidParamError(instr.1[0]))
            }),
            IC::CALL => params_err(1).and_then(|_| {
                matches!(instr.1[0], Arg::Function(_))
                    .then_some(())
                    .ok_or(TranslitError::InvalidParamError(instr.1[0]))
            }),
            IC::JMP => params_err(1).and_then(|_| {
                matches!(instr.1[0], Arg::Label(_))
                    .then_some(())
                    .ok_or(TranslitError::InvalidParamError(instr.1[0]))
            }),
            IC::JMPIF => params_err(2)
                .and_then(|_| {
                    matches!(instr.1[0], Arg::Label(_))
                        .then_some(())
                        .ok_or(TranslitError::InvalidParamError(instr.1[0]))
                })
                .and_then(|_| {
                    matches!(instr.1[1], Arg::Var(_))
                        .then_some(())
                        .ok_or(TranslitError::InvalidParamError(instr.1[1]))
                }),
            IC::RET => {
                if let Some(Function { end: Some(_), .. }) | None = self.functions.last() {
                    return Err(TranslitError::RetOutsideFuncError);
                }
                params_err(1)
            }
        }
    }

    /// Push an instruction into the IR. Returns an error if a RET instruction is passed outside a function.
    pub fn push(&mut self, code: IC, args: Vec<Arg>) -> TranslitResult<Variable> {
        let instr = Instruction(code, args);
        self.verify(&instr)?;
        self.instructions.push(instr);

        Ok(Variable(self.instructions.len() - 1))
    }
}
