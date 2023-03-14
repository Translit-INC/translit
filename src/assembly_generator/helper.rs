use crate::ir::{Instruction, InstructionCode::*, Arg, Literal, BlockID};
use crate::error::{TranslitError, TranslitResult};

pub fn init_data_section() -> String {
    format!("section .data\n").to_string()
}

pub fn init_text_section() -> String {
    format!("section .text\nglobal _start\n\n").to_string()
}

pub fn init_main_function() -> String {
    format!("_start:\n").to_string()
}

pub fn convert_asm(instr: Instruction) -> TranslitResult<String> {
    return match (num::FromPrimitive::from_u64(instr.0).unwrap(), instr.1.as_slice()) {
        (ADD, &[a, b]) => Ok("".to_string()),
        _ => Err(TranslitError::AssemblyGenerationError("Function not found".to_string())),
    };

    Ok("".to_string())
}
