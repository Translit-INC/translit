// TODO: everything

pub(crate) enum Architecture {
    Nasm86_64
}

pub(crate) struct AssemblyGenerator {
    /// Data section of the assembly
    data_asm: String,

    /// Main function of the assembly
    start_asm: String,

    /// Functions will be stored just above the main function
    function_asm: String,

    architecture: Architecture
}

impl AssemblyGenerator {
    pub fn new(architecture: Architecture) -> Self {
        AssemblyGenerator { data_asm: String::new(), start_asm: String::new(), function_asm: String::new(), architecture }
    }
}
