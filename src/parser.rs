use std::collections::HashMap;

use crate::types::{AInstruction, Address, CInstruction, Instruction};
use regex::Regex;

pub fn parse(instruction: &String, table: &mut HashMap<String, (i32, Address)>) -> Instruction {
    let mut cloned = instruction.clone();

    if cloned.starts_with('@') {
        cloned.remove(0);

        if cloned.parse::<i32>().is_ok() {
            return Instruction::AInstruction(AInstruction { decimal: cloned });
        } else {
            if table.contains_key(&cloned) {
                return Instruction::AInstruction(AInstruction {
                    decimal: table.get(&cloned).copied().unwrap().0.to_string(),
                });
            } else {
                let mut temp_table = table.clone();

                temp_table.remove_entry(&String::from("SCREEN"));
                temp_table.remove_entry(&String::from("KBD"));

                let address = temp_table
                    .iter()
                    .filter(|(_, (_, address))| matches!(address, Address::RAM))
                    .fold(
                        0,
                        |acc, (_, (addr, _))| if *addr > acc { *addr } else { acc },
                    )
                    + 1;

                table.insert(cloned, (address, Address::RAM));
                return Instruction::AInstruction(AInstruction {
                    decimal: address.to_string(),
                });
            }
        }
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
