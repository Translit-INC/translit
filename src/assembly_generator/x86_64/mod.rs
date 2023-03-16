mod helper;
use helper::*;

use super::{AssemblyGenerationError, AssemblyGenerationResult};
use crate::{Arg, InstructionCode::*, Literal, Type, IR};

pub fn generate_assembly_nasm_x86_64(ir: IR) -> AssemblyGenerationResult<String> {
    let Some(main_func) = ir.functions.first() else {
        return Err(AssemblyGenerationError::NoMainFunction);
    };

    let main_func_instructions = &ir.instructions[main_func.start..main_func.end.unwrap()];

    let data_section = INIT_DATA_SECTION.to_string();
    let mut text_section = INIT_TEXT_SECTION.to_string();
    let mut main_function = INIT_MAIN_SECTION.to_string();

    for inst in main_func_instructions {
        main_function += gen(inst).as_str();
    }

    for func in &ir.functions[1..] {
        text_section += begin_function(func.start).as_str();

        for inst in &ir.instructions[func.start..func.end.unwrap()] {
            text_section += gen(inst).as_str();
        }

        text_section += end_function().as_str();
    }

    // generated assembly
    Ok(format!(
        "{data_section}\n{text_section}\n{main_function}\n{EXIT_SYSCALL}\n",
    ))
}
