use super::IRBuilder;
use crate::{TranslitError, TranslitResult};
use crate::assembly_generator::x86_64::generate_assembly_nasm_x86_64;

impl IRBuilder {
    pub fn generate_assembly(&mut self) -> TranslitResult<()> {
        let asm = String::new();

        if self.functions.len() == 0 {
            return Err(TranslitError::AssemblyGenerationError("Main function not found".to_string()));
        }

        self.generate_assembly_nasm_x86_64();

        Ok(())
    }
}
