use crate::{Arg, Instruction, InstructionCode::*, TranslitResult, Type, Literal, Variable, FunctionID, IR};

pub const INIT_DATA_SECTION: &str = "section .data";
pub const INIT_TEXT_SECTION: &str = "section .text\nglobal _start\n";
pub const INIT_MAIN_SECTION: &str = "_start:";
pub const EXIT_SYSCALL: &str = "\tmov eax, 60\n\tmov edi, 0\n\tsyscall";

pub fn get_info(a: Arg) -> Option<(Type, u64)> {
    if let Arg::Literal(x) = a {
        Some((x.0, x.1))
    } else {
        None
    }
}

pub fn get_value(a: Arg) -> Option<u64> {
    match a {
        Arg::Literal(Literal(_, x)) => Some(x),
        Arg::Var(Variable(x)) => Some(x as _),

        _ => unimplemented!()
    }
}

pub fn get_register(typ: Type) -> &'static str {
    match typ {
        Type::i8 => "bpl",
        Type::i16 => "bp",
        Type::i32 => "ebp",
        Type::i64 => "rbp",

        _ => unreachable!(),
    }
}

pub fn begin_function(idx: usize) -> String {
    format!("\nfunc_{idx}:")
}

pub fn end_function() -> String {
    format!("\n\tret\n")
}

pub fn gen(inst: &Instruction, ir: &IR) -> String {
    match (inst.0, inst.1.as_slice()) {
        (ADD, &[a, b]) => format!("\n\tpush {}", get_value(a).unwrap() + get_value(b).unwrap()),
        (SUB, &[a, b]) => format!("\n\tpush {}", get_value(a).unwrap() - get_value(b).unwrap()),
        (MUL, &[a, b]) => format!("\n\tpush {}", get_value(a).unwrap() * get_value(b).unwrap()),
        (DIV, &[a, b]) => format!("\n\tpush {}", get_value(a).unwrap() / get_value(b).unwrap()), // zero division is checking in the builder
        (MOD, &[a, b]) => format!("\n\tpush {}", get_value(a).unwrap() % get_value(b).unwrap()),
        (AND, &[a, b]) => format!("\n\tpush {}", get_value(a).unwrap() & get_value(b).unwrap()),
        (OR,  &[a, b]) => format!("\n\tpush {}", get_value(a).unwrap() | get_value(b).unwrap()),
        (NOT, &[a])    => format!("\n\tpush {}", !get_value(a).unwrap() as u8),
        (SHL, &[a, b]) => format!("\n\tpush {}", get_value(a).unwrap() << get_value(b).unwrap()),
        (SHR, &[a, b]) => format!("\n\tpush {}", get_value(a).unwrap() >> get_value(b).unwrap()),
        (EQ,  &[a, b]) => format!("\n\tpush {}", get_value(a).unwrap() == get_value(b).unwrap()),
        (CMP, &[a, b]) => format!("\n\tpush {}", get_value(a).unwrap() > get_value(b).unwrap()),
        (CMPEQ , &[a, b]) => format!("\n\tpush {}", get_value(a).unwrap() >= get_value(b).unwrap()),
        (CALL, &[Arg::Function(FunctionID(id))]) => {
            if ir.functions.iter().find(|i| i.start == id).is_none() {
                panic!("Function not defined");
            } else {
                format!("\n\tcall func_{id}")
            }
        }

        _ => unimplemented!()
    }
}
