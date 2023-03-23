use crate::{Arg, Function, FunctionID, Instruction, Label, Literal, Variable, Type};

pub mod builder;
pub mod instruction;
pub mod types;

#[derive(Debug, Clone)]
pub struct IR {
    pub(crate) instructions: Vec<Instruction>,
    pub(crate) functions: Vec<Function>,
    pub(crate) labels: Vec<Label>,
    pub(crate) memory: Vec<(usize, Type)>,
}

impl IR {
    pub fn print(&self) {
        let mut iter = self
            .functions
            .iter()
            .enumerate()
            .map(|(i, f)| ("f", i, f.start))
            .chain(self.labels.iter().enumerate().map(|(i, l)| ("l", i, l.0)))
            .collect::<Vec<_>>();
        iter.sort_by(|(_, _, i), (_, _, j)| j.cmp(i));
        for (i, instr) in self.instructions.iter().enumerate() {
            if let Some(&(f, j, a)) = iter.last() {
                if a == i {
                    println!("{f}{j}:");
                }
            }
            println!(
                "\t{:?} {}",
                instr.0,
                instr.1.iter().fold(String::new(), |s, a| match a {
                    Arg::Label(Label(id)) => format!("{s}l{id} "),
                    Arg::Function(FunctionID(id)) => format!("{s}f{id} "),
                    Arg::Var(Variable(_, id)) => format!("{s}@{id} "),
                    Arg::Literal(Literal(t, a)) => format!("{s}{a}{t:?} "),
                })
            )
        }
    }
}
