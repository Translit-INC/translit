use super::builder::IRBuilder;

impl IRBuilder {
    pub fn generate_assembly(&mut self) {
        let asm = String::new();

        let functions = self.functions.clone();
        let instructions = self.instructions.clone();
        let blocks = self.blocks.clone();
    }
}
