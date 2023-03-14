pub mod error;
pub mod x86_64;

use crate::{TranslitError, TranslitResult, IR};
use error::{AssemblyGenerationError, AssemblyGenerationResult};

#[allow(non_camel_case_types)]
pub enum Architecture {
    x86_64,
}

pub fn generate_assembly(arch: Architecture, ir: IR) -> TranslitResult<()> {
    match arch {
        Architecture::x86_64 => x86_64::generate_assembly_nasm_x86_64(ir)
            .map_err(TranslitError::AssemblyGenerationError),
    }
}
