pub struct AInstruction {
    pub decimal: String,
}

pub struct CInstruction {
    pub dest: String,
    pub comp: String,
    pub jump: Option<String>,
}

pub enum Instruction {
    AInstruction(AInstruction),
    CInstruction(CInstruction),
}
