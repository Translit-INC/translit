use crate::{Instruction, InstructionCode::*, TranslitResult, Type, Arg};

pub fn init_data_section() -> String {
    "section .data\n".to_owned()
}

pub fn init_text_section() -> String {
    "section .text\nglobal _start\n\n".to_owned()
}

pub fn init_main_function() -> String {
    "_start:\n".to_owned()
}

pub fn exit_syscall() -> String {
  "\n\tmov eax, 60\
\tmov edi, 0\
\tsyscall".to_owned()
}

// pub fn convert_asm(instr: Instruction) -> TranslitResult<String> {
//     match (instr.0, instr.1.as_slice()) {
//         (ADD, &[_a, _b]) => todo!(),
//         _ => todo!(),
//     }
// }

pub fn get_info(a: Arg) -> Option<(Type, u64)> {
    if let Arg::Literal(x) = a { Some((x.0, x.1)) }
    else { None }
}

pub fn get_value(a: Arg) -> Option<u64> {
    if let Arg::Literal(x) = a { Some(x.1) }
    else { None }
}

pub fn get_register<'a>(typ: &'a Type) -> &'a str {
    match typ {
        Type::i8 => "bpl",
        Type::i16 => "bp",
        Type::i32 => "ebp",
        Type::i64 => "rbp",

        _ => unreachable!()
    }
}
