use crate::types::{AInstruction, CInstruction, Instruction};

pub fn parse(instruction: &String) -> Option<Instruction> {
    let mut cloned = instruction.clone();

    if cloned.chars().nth(0).unwrap() == '@' {
        cloned.remove(0);

        return Some(Instruction::AInstruction(AInstruction {
            decimal: cloned.to_string(),
        }));
    }

    let slice: Vec<&str> = cloned.split("=").collect();

    match slice.as_slice() {
        [dest, comp_and_jump] => match comp_and_jump.find(";") {
            Some(with_jump) => {
                let second_slice: Vec<&str> = comp_and_jump.split(";").collect();

                match second_slice.as_slice() {
                    [comp, jump] => Some(Instruction::CInstruction(CInstruction {
                        dest: dest.to_string(),
                        comp: comp.to_string(),
                        jump: Some(jump.to_string()),
                    })),
                    _ => None,
                }
            }
            None => Some(Instruction::CInstruction(CInstruction {
                dest: dest.to_string(),
                comp: comp_and_jump.to_string(),
                jump: None,
            })),
        },
        _ => None,
    }
}
