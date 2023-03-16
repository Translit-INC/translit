mod helper;
use helper::*;

use std::fmt;

use crate::{Type, Arg, InstructionCode::*, IR};
use super::{AssemblyGenerationError, AssemblyGenerationResult};

pub fn generate_assembly_nasm_x86_64(ir: IR) -> AssemblyGenerationResult<String> {
    let Some(main_func) = ir.functions.first() else {
        return Err(AssemblyGenerationError::NoMainFunction);
    };

    let main_func_instructions =
        ir.instructions[main_func.start..=main_func.end.unwrap() - 1].to_vec();

    let data_section = helper::init_data_section();
    let text_section = helper::init_text_section();
    let mut main_function = helper::init_main_function();

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
            (NOT, &[a])    => format!("\n\tpush {}", if get_value(a).unwrap() != 0 { 0  } else { 1 }),
            (SHL, &[a, b]) => format!("\n\tpush {}", get_value(a).unwrap() << get_value(b).unwrap()),
            (SHR, &[a, b]) => format!("\n\tpush {}", get_value(a).unwrap() >> get_value(b).unwrap()),
            (EQ , &[a, b]) => format!("\n\tpush {}", get_value(a).unwrap() >> get_value(b).unwrap()),

            _ => { unimplemented!() },
        }.as_str();
    }

    // generated assembly
    Ok(format!("{data_section}\n{text_section}\n{main_function}\n{}\n", exit_syscall()))
}



// Don't touch this
// let (typ, val1) = helper::get_info(a);
// let (_, val2) = helper::get_info(b);
// let register_name: &str = helper::get_register(&typ);

// main_function += format!("\n\tlea {register_name}, [{val1}+{val2}]\n\tpush {register_name}").as_str();
