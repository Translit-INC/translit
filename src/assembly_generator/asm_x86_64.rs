#[path = "helper.rs"] mod helper;

use crate::{ir::{IRBuilder, Instruction}, error::{TranslitError, TranslitResult}};

impl IRBuilder {
    pub(crate) fn generate_assembly_nasm_x86_64(&mut self) -> TranslitResult<()> {
        let mut builder = self.clone();

        let Some(main_func) = builder.functions.first() else {
            return Err(TranslitError::AssemblyGenerationError("Main function not found".to_string()));
        };

        let main_func_instructions = builder.instructions[main_func.start..=main_func.end.unwrap() - 1].to_vec();

        let mut data_section = helper::init_data_section();
        let mut text_section = helper::init_text_section();
        let mut main_function = helper::init_main_function();

        for inst in main_func_instructions {

        }

        Ok(())
    }
}
