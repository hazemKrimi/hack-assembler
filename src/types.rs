pub enum Instruction {
    AInstruction {
        decimal: String
    },
    CInstruction {
        dest: String,
        comp: String,
        jump: String
    }
}