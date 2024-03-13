use crate::types::{AInstruction, CInstruction, Instruction};
use regex::Regex;

pub fn parse(instruction: &String) -> Instruction {
    let mut cloned = instruction.clone();

    if cloned.chars().nth(0).unwrap() == '@' {
        cloned.remove(0);

        return Instruction::AInstruction(AInstruction {
            decimal: cloned.trim().to_string(),
        });
    } else {
        let re_dest = Regex::new(r"=").unwrap();
        let re_jump = Regex::new(r";").unwrap();

        let dest_exists = re_dest.is_match(&cloned);
        let jump_exists = re_jump.is_match(&cloned);

        let dest = match dest_exists {
            true => {
                let slice: Vec<&str> = cloned.split("=").collect();

                Some(slice.as_slice()[0].to_string())
            }
            false => None,
        };

        let comp = match (dest_exists, jump_exists) {
            (true, false) => {
                let slice: Vec<&str> = cloned.split("=").collect();

                Some(slice.as_slice()[1].to_string())
            }
            (false, true) => {
                let slice: Vec<&str> = cloned.split(";").collect();

                Some(slice.as_slice()[0].to_string())
            }
            (true, true) => {
                let slice: Vec<&str> = cloned.split("=").collect();
                let another_slice: Vec<&str> = slice.as_slice()[1].split(";").collect();

                Some(another_slice.as_slice()[0].to_string())
            }
            (false, false) => None,
        };

        let jump = match jump_exists {
            true => {
                let slice: Vec<&str> = cloned.split(";").collect();

                Some(slice.as_slice()[1].to_string())
            }
            false => None,
        };

        Instruction::CInstruction(CInstruction { dest, comp, jump })
    }
}
