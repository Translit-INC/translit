use crate::{Instruction, InstructionCode::*, TranslitResult};

pub fn init_data_section() -> String {
    "section .data\n".to_string()
}

pub fn init_text_section() -> String {
    "section .text\nglobal _start\n\n".to_string()
}

pub fn init_main_function() -> String {
    "_start:\n".to_string()
}

pub fn convert_asm(instr: Instruction) -> TranslitResult<String> {
    match (instr.0, instr.1.as_slice()) {
        (ADD, &[_a, _b]) => todo!(),
        _ => todo!(),
    }
}
