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
        let re_dest_and_comp = Regex::new(r"=").unwrap();
        let re_comp_and_jump = Regex::new(r";").unwrap();

        let dest_and_comp_exist = re_dest_and_comp.is_match(&cloned);
        let comp_and_jump_exist = re_comp_and_jump.is_match(&cloned);

        let dest = match dest_and_comp_exist {
            true => {
                let slice: Vec<&str> = cloned.split("=").collect();

                Some(slice.as_slice()[0].to_string())
            }
            false => None,
        };

        let comp = match comp_and_jump_exist {
            true => {
                let slice: Vec<&str> = cloned.split("=").collect();

                // if jump_exist {
                //     let another_slice: Vec<&str> = slice.as_slice()[1].split(";").collect();

                //     Some(another_slice.as_slice()[0].to_string());
                // }

                Some(slice.as_slice()[1].to_string())
            }
            false => None,
        };

        let jump = match comp_and_jump_exist {
            true => {
                let slice: Vec<&str> = cloned.split(";").collect();

                Some(slice.as_slice()[1].to_string())
            }
            false => None,
        };

        Instruction::CInstruction(CInstruction { dest, comp, jump })
    }
}
