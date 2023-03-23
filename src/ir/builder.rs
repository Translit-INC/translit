use crate::{
    Arg, Function, FunctionID, Instruction, InstructionCode as IC, InstructionOuput, Label,
    Literal, Signature, TranslitError, TranslitResult, Type, VarAssignable, Variable, IR,
};

/// IR Builder
pub struct IRBuilder {
    instructions: Vec<Instruction>,
    functions: Vec<Function>,
    labels: Vec<Label>,
    /// id and type of intructions in self.instructions
    memory: Vec<(usize, Type)>,
}

impl IRBuilder {
    /// Create a new builder
    pub fn new() -> Self {
        IRBuilder {
            instructions: Vec::new(),
            functions: Vec::new(),
            labels: Vec::new(),
            memory: Vec::new(),
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
            memory: self.memory,
        })
    }

    /// Start a function.
    /// Every instruction after this function call will be considered as the part of the function until end_function is called.
    /// Returns an error if a function is already going on
    pub fn start_function(&mut self, sig: &Signature) -> TranslitResult<FunctionID> {
        if let Some(Function { end: None, .. }) = self.functions.last() {
            return Err(TranslitError::FunctionStartError);
        }
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
    pub fn get_type(&self, instr: &Instruction) -> TranslitResult<Type> {
        let params_err = |length: usize| {
            (instr.1.len() == length)
                .then_some(())
                .ok_or(TranslitError::InstrParamLenError)
        };

        let same_typed = || match *instr.1.as_slice() {
            [Arg::Literal(Literal(t1, _)), a @ Arg::Literal(Literal(t2, _))] => (t1 == t2)
                .then_some(t1)
                .ok_or(TranslitError::InvalidTypeError(a)),
            [Arg::Var(Variable(t1, _)), a @ Arg::Literal(Literal(t2, _))] => (t1 == t2)
                .then_some(t1)
                .ok_or(TranslitError::InvalidTypeError(a)),
            [Arg::Literal(Literal(t1, _)), a @ Arg::Var(Variable(t2, _))] => (t1 == t2)
                .then_some(t1)
                .ok_or(TranslitError::InvalidTypeError(a)),
            [Arg::Var(Variable(t1, _)), a @ Arg::Var(Variable(t2, _))] => (t1 == t2)
                .then_some(t1)
                .ok_or(TranslitError::InvalidTypeError(a)),
            _ => Err(TranslitError::InvalidParamError(instr.1[0])),
        };

        let get_type = || {
            if let Arg::Literal(Literal(t, _)) | Arg::Var(Variable(t, _)) = instr.1[0] {
                Ok(t)
            } else {
                Err(TranslitError::InvalidParamError(instr.1[0]))
            }
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
            | IC::DIV
            | IC::OR => params_err(2).and_then(|_| same_typed()),

            IC::NOT => params_err(1).and_then(|_| get_type()),

            IC::CALL => params_err(1).and_then(|_| {
                if let Some(Function { end: Some(_), .. }) | None = self.functions.last() {
                    return Err(TranslitError::CallOutsideFunction);
                }

                let Arg::Function(FunctionID(id)) = instr.1[0] else {
                    return Err(TranslitError::InvalidParamError(instr.1[0]));
                };

                if self.functions.len() - 1 == id {
                    return Err(TranslitError::CalledMainFunction);
                }

                Ok(self.functions[id].sig.returns)
            }),

            IC::JMP => params_err(1).and_then(|_| {
                matches!(instr.1[0], Arg::Label(_))
                    .then_some(Type::none)
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
                        .then_some(Type::none)
                        .ok_or(TranslitError::InvalidParamError(instr.1[1]))
                }),
            IC::RET => {
                params_err(1)?;
                match self.functions.last() {
                    Some(Function { end: Some(_), .. }) | None => {
                        Err(TranslitError::RetOutsideFuncError)
                    }
                    Some(Function { sig, .. }) => {
                        if sig.returns != get_type()? {
                            return Err(TranslitError::InvalidTypeError(instr.1[0]));
                        }
                        Ok(Type::none)
                    }
                }
            }
            IC::PUSH => {
                panic!("Not meant for user");
            }
            IC::PHI => {
                let mut t = None;
                for arg in &instr.1 {
                    if let Arg::Var(Variable(t0, _)) = arg {
                        if let Some(t) = t {
                            if t != *t0 {
                                return Err(TranslitError::InvalidTypeError(*arg));
                            }
                        } else {
                            t = Some(*t0);
                        }
                    } else {
                        return Err(TranslitError::InvalidParamError(*arg));
                    }
                }
                t.ok_or(TranslitError::InstrParamLenError)
            }
        }
    }

    /// Push an instruction into the IR. Returns an error if a RET instruction is passed outside a function.
    pub fn push(&mut self, code: IC, args: Vec<Arg>) -> TranslitResult<InstructionOuput> {
        match self.functions.last() {
            Some(x) if x.end.is_none() => {}
            _ => return Err(TranslitError::InstructionOutsideFunction),
        };

        let instr = Instruction(code, args);
        let type_ = self.get_type(&instr)?;
        if type_ != Type::none {
            self.memory.push((self.instructions.len(), type_))
        }
        self.instructions.push(instr);

        Ok(InstructionOuput {
            memory: (type_ != Type::none).then_some(self.memory.len() - 1),
            type_,
        })
    }

    pub fn create_var(&mut self, type_: Type) -> Variable {
        Variable(type_, 0)
    }

    pub fn set_var(
        &mut self,
        var: &mut Variable,
        assign: impl Into<VarAssignable>,
    ) -> TranslitResult<()> {
        match assign.into() {
            VarAssignable::Literal(l @ Literal(t, _)) => {
                if var.0 != t {
                    return Err(TranslitError::InvalidTypeError(Arg::Literal(l)));
                }
                self.memory.push((self.instructions.len(), t));
                self.instructions
                    .push(Instruction(IC::PUSH, vec![l.into()]));
                var.1 = self.memory.len() - 1;
                Ok(())
            }
            VarAssignable::InstOut(InstructionOuput { memory, type_ }) => {
                let Some(memory) = memory else {
                    return Err(TranslitError::AssignedUnassignableValue);
                };
                if var.0 != type_ {
                    return Err(TranslitError::InvalidTypeError(Arg::Var(Variable(
                        type_, memory,
                    ))));
                }
                var.1 = memory;
                Ok(())
            }
        }
    }
}
