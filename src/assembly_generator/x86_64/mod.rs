mod helper;
use helper::*;

use crate::{
    Arg::*, AssemblyGenerationError, AssemblyGenerationResult, FunctionID, Instruction,
    InstructionCode::*, IR,
};

pub fn generate_assembly_nasm_x86_64(ir: IR) -> AssemblyGenerationResult<String> {
    let Some(main_func) = ir.functions.last() else {
        return Err(AssemblyGenerationError::NoMainFunction);
    };

    let main_func_instructions = &ir.instructions[main_func.start..main_func.end.unwrap()];

    let data_section = INIT_DATA_SECTION.to_string();
    let mut text_section = INIT_TEXT_SECTION.to_string();
    let mut main_function = INIT_MAIN_SECTION.to_string();

    for inst in main_func_instructions {
        main_function += gen(inst, &ir).as_str();
    }

    for func in &ir.functions[..ir.functions.len() - 1] {
        text_section += begin_function(func.start).as_str();

        for inst in &ir.instructions[func.start..func.end.unwrap()] {
            text_section += gen(inst, &ir).as_str();
        }

        text_section += END_FUNCTION;
    }

    // generated assembly
    Ok(format!(
        "{data_section}{text_section}{main_function}{EXIT_SYSCALL}",
    ))
}

pub fn gen(inst: &Instruction, ir: &IR) -> String {
    match (inst.0, inst.1.as_slice()) {
        (ADD, &[Literal(a), Literal(b)]) => format!("\tpush {}\n", a.1 + b.1),
        (SUB, &[Literal(a), Literal(b)]) => format!("\tpush {}\n", a.1 - b.1),
        (MUL, &[Literal(a), Literal(b)]) => format!("\tpush {}\n", a.1 * b.1),
        (DIV, &[Literal(a), Literal(b)]) => format!("\tpush {}\n", a.1 / b.1),
        (MOD, &[Literal(a), Literal(b)]) => format!("\tpush {}\n", a.1 % b.1),
        (AND, &[Literal(a), Literal(b)]) => format!("\tpush {}\n", a.1 & b.1),
        (OR, &[Literal(a), Literal(b)]) => format!("\tpush {}\n", a.1 | b.1),
        (NOT, &[Literal(a)]) => format!("\tpush {}\n", !a.1 as u8),
        (SHL, &[Literal(a), Literal(b)]) => format!("\tpush {}\n", a.1 << b.1),
        (SHR, &[Literal(a), Literal(b)]) => format!("\tpush {}\n", a.1 >> b.1),
        (EQ, &[Literal(a), Literal(b)]) => format!("\tpush {}\n", a.1 == b.1),
        (CMP, &[Literal(a), Literal(b)]) => format!("\tpush {}\n", a.1 > b.1),
        (CMPEQ, &[Literal(a), Literal(b)]) => format!("\tpush {}\n", a.1 >= b.1),
        (CALL, &[Function(FunctionID(id))]) => {
            if !ir.functions.iter().any(|i| i.start == id) {
                panic!("Function not defined");
            } else {
                format!("\tcall func_{id}\n")
            }
        }

        _ => unimplemented!(),
    }
}
