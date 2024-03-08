use crate::types::Instruction;

pub fn parse(mut instruction: String) -> Instruction {
    if instruction.chars().nth(0).unwrap() == '@' {
        instruction.remove(0);
        return Instruction::AInstruction { decimal: instruction };
    }

    Instruction::CInstruction {
        dest: "1".to_owned(),
        comp: "2".to_owned(),
        jump: "3".to_owned(),
    }
}
