mod helper;

use crate::IR;

use super::{AssemblyGenerationError, AssemblyGenerationResult};

pub fn generate_assembly_nasm_x86_64(ir: IR) -> AssemblyGenerationResult<()> {
    let Some(main_func) = ir.functions.first() else {
        return Err(AssemblyGenerationError::NoMainFunction);
    };

    let main_func_instructions =
        ir.instructions[main_func.start..=main_func.end.unwrap() - 1].to_vec();

    let data_section = helper::init_data_section();
    let text_section = helper::init_text_section();
    let main_function = helper::init_main_function();

    for inst in main_func_instructions {
        todo!()
    }

    Ok(())
}
