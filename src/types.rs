pub struct AInstruction {
    pub decimal: String,
}

pub struct CInstruction {
    pub dest: Option<String>,
    pub comp: Option<String>,
    pub jump: Option<String>,
}

pub enum Instruction {
    AInstruction(AInstruction),
    CInstruction(CInstruction),
}