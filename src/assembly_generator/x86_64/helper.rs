use crate::Type;

pub const INIT_DATA_SECTION: &str = "section .data\n";
pub const INIT_TEXT_SECTION: &str = "section .text\nglobal _start\n";
pub const INIT_MAIN_SECTION: &str = "_start:\n";
pub const EXIT_SYSCALL: &str = "\tmov eax, 60\n\tmov edi, 0\n\tsyscall\n";
pub const END_FUNCTION: &str = "\tret\n\n";
pub fn begin_function(idx: usize) -> String {
    format!("func_{idx}:\n")
}

pub const fn get_register(typ: Type) -> &'static str {
    match typ {
        Type::i8 => "bpl",
        Type::i16 => "bp",
        Type::i32 => "ebp",
        Type::i64 => "rbp",

        _ => unreachable!(),
    }
}
