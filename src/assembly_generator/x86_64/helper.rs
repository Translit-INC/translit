use crate::{Arg, Instruction, InstructionCode::*, TranslitResult, Type};

pub const INIT_DATA_SECTION: &str = "section .data";
pub const INIT_TEXT_SECTION: &str = "section .text\nglobal _start\n";
pub const INIT_MAIN_SECTION: &str = "_start:\n";
pub const EXIT_SYSCALL: &str = "\tmov eax, 60\n\tmov edi, 0\n\tsyscall";

pub fn get_info(a: Arg) -> Option<(Type, u64)> {
    if let Arg::Literal(x) = a {
        Some((x.0, x.1))
    } else {
        None
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
