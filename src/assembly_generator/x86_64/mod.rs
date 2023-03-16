mod helper;
use helper::*;

use super::{AssemblyGenerationError, AssemblyGenerationResult};
use crate::{Arg, InstructionCode::*, Literal, Type, IR};

pub fn generate_assembly_nasm_x86_64(ir: IR) -> AssemblyGenerationResult<String> {
    let Some(main_func) = ir.functions.first() else {
        return Err(AssemblyGenerationError::NoMainFunction);
    };

    let main_func_instructions =
        ir.instructions[main_func.start..=main_func.end.unwrap() - 1].to_vec();

    let data_section = INIT_DATA_SECTION.to_string();
    let text_section = INIT_TEXT_SECTION.to_string();
    let mut main_function = INIT_MAIN_SECTION.to_string();

    for inst in main_func_instructions {
        // I will do some cheating in literals
        main_function += match (inst.0, inst.1.as_slice()) {
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
            (EQ , &[a, b]) => format!("\n\tpush {}", get_value(a).unwrap() == get_value(b).unwrap()),
            (CMP , &[a, b]) => format!("\n\tpush {}", get_value(a).unwrap() > get_value(b).unwrap()),
            (CMPEQ , &[a, b]) => format!("\n\tpush {}", get_value(a).unwrap() >= get_value(b).unwrap()),

            _ => unimplemented!()
        }.as_str();
    }

    // generated assembly
    Ok(format!(
        "{data_section}\n{text_section}\n{main_function}\n{EXIT_SYSCALL}\n",
    ))
}
