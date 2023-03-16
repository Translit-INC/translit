mod helper;
use helper::*;

use std::fmt;

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
            (ADD, &[Arg::Literal(Literal(_, a)), Arg::Literal(Literal(_, b))]) => {
                format!("\tpush {}\n", a + b)
            }
            (SUB, &[Arg::Literal(Literal(_, a)), Arg::Literal(Literal(_, b))]) => {
                format!("\tpush {}\n", a - b)
            }
            (MUL, &[Arg::Literal(Literal(_, a)), Arg::Literal(Literal(_, b))]) => {
                format!("\tpush {}\n", a * b)
            }
            (DIV, &[Arg::Literal(Literal(_, a)), Arg::Literal(Literal(_, b))]) => {
                format!("\tpush {}\n", a / b)
            }
            (MOD, &[Arg::Literal(Literal(_, a)), Arg::Literal(Literal(_, b))]) => {
                format!("\tpush {}\n", a % b)
            }
            (AND, &[Arg::Literal(Literal(_, a)), Arg::Literal(Literal(_, b))]) => {
                format!("\tpush {}\n", a & b)
            }
            (OR, &[Arg::Literal(Literal(_, a)), Arg::Literal(Literal(_, b))]) => {
                format!("\tpush {}\n", a | b)
            }
            (NOT, &[Arg::Literal(Literal(_, a))]) => format!("\tpush {}\n", !a as u8),
            (SHL, &[Arg::Literal(Literal(_, a)), Arg::Literal(Literal(_, b))]) => {
                format!("\tpush {}\n", a << b)
            }
            (SHR, &[Arg::Literal(Literal(_, a)), Arg::Literal(Literal(_, b))]) => {
                format!("\tpush {}\n", a >> b)
            }
            (EQ, &[Arg::Literal(Literal(_, a)), Arg::Literal(Literal(_, b))]) => {
                format!("\tpush {}\n", (a == b) as u8)
            }
            (CMP, &[Arg::Literal(Literal(_, a)), Arg::Literal(Literal(_, b))]) => {
                format!("\tpush {}\n", (a > b) as u8)
            }
            (CMPEQ, &[Arg::Literal(Literal(_, a)), Arg::Literal(Literal(_, b))]) => {
                format!("\tpush {}\n", (a >= b) as u8)
            }

            _ => {
                unimplemented!()
            }
        }
        .as_str();
    }

    // generated assembly
    Ok(format!(
        "{data_section}\n{text_section}\n{main_function}\n{EXIT_SYSCALL}\n",
    ))
}

// Don't touch this
// let (typ, val1) = helper::get_info(a);
// let (_, val2) = helper::get_info(b);
// let register_name: &str = helper::get_register(&typ);

// main_function += format!("\n\tlea {register_name}, [{val1}+{val2}]\n\tpush {register_name}").as_str();
