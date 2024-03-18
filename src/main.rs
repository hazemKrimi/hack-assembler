use std::cell::RefCell;
use std::collections::HashMap;
use std::env;
use std::ffi;
use std::fs;
use std::io::Write;
use std::path;
use std::path::PathBuf;

use regex::Regex;
use types::Address;
use types::Instruction;

mod code;
mod parser;
mod types;

fn remove_whitespace_and_comments(content: String) -> String {
    let re = Regex::new(r"\s*\/\/.*").unwrap();
    
    content
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.starts_with("//") && !line.is_empty())
        .map(|line| re.replace_all(line, ""))
        .map(|line| format!("{}\n", line))
        .collect::<String>()
}

fn first_pass(content: String, table: &RefCell<HashMap<String, (i32, Address)>>) -> String {
    let mut mutable_table = table.borrow_mut();

    content
        .lines()
        .enumerate()
        .filter(|(_, line)| line.starts_with('(') && line.ends_with(')'))
        .collect::<Vec<(usize, &str)>>()
        .iter()
        .enumerate()
        .for_each(|(index, (line_number, instruction))| {
            let symbol = instruction[1..instruction.len() - 1].to_string();

            if index > 0 {
                mutable_table.insert(symbol, ((*line_number - index) as i32, Address::ROM));
            } else {
                mutable_table.insert(symbol, (*line_number as i32, Address::ROM));
            }
        });

    content
        .lines()
        .filter(|line| !line.starts_with('(') && !line.ends_with(')'))
        .map(|line| format!("{}\n", line))
        .collect()
}

fn second_pass(content: String, table: &RefCell<HashMap<String, (i32, Address)>>) -> String {
    content
        .lines()
        .map(|instruction| {
            let mut mutable_table = table.borrow_mut();

            if instruction.is_empty() {
                return instruction.to_string();
            }

            match parser::parse(&instruction.to_string(), &mut mutable_table) {
                Instruction::AInstruction(parsed_instruction) => {
                    match parsed_instruction.decimal.parse::<i32>() {
                        Ok(decimal) => {
                            let translated = code::decimal_to_fifteen_bits_binary(&decimal);

                            String::from(format!("0{}", translated))
                        }
                        Err(_) => panic!("Failed to parse A instruction {}", instruction),
                    }
                }
                Instruction::CInstruction(parsed_instruction) => {
                    String::from("111")
                        + code::translate_comp(
                            &parsed_instruction.comp.unwrap_or_else(|| "".to_string()),
                        )
                        .as_str()
                        + code::translate_dest(
                            &parsed_instruction.dest.unwrap_or_else(|| "".to_string()),
                        )
                        .as_str()
                        + code::translate_jump(
                            &parsed_instruction.jump.unwrap_or_else(|| "".to_string()),
                        )
                        .as_str()
                }
            }
        })
        .filter(|line| !line.is_empty())
        .map(|line| format!("{}\n", line))
        .collect::<String>()
}

fn process(path: &PathBuf) {
    let filepath = path
        .canonicalize()
        .unwrap()
        .to_str()
        .unwrap()
        .replace("asm", "hack");
    let mut file = fs::File::create(filepath).unwrap();
    let content = fs::read_to_string(&path).expect("You must provide a correct filepath!");
    let symbol_table = RefCell::new(HashMap::from([
        (String::from("SP"), (0, Address::ROM)),
        (String::from("LCL"), (1, Address::ROM)),
        (String::from("ARG"), (2, Address::ROM)),
        (String::from("THIS"), (3, Address::ROM)),
        (String::from("THAT"), (4, Address::ROM)),
        (String::from("R0"), (0, Address::RAM)),
        (String::from("R1"), (1, Address::RAM)),
        (String::from("R2"), (2, Address::RAM)),
        (String::from("R3"), (3, Address::RAM)),
        (String::from("R4"), (4, Address::RAM)),
        (String::from("R5"), (5, Address::RAM)),
        (String::from("R6"), (6, Address::RAM)),
        (String::from("R7"), (7, Address::RAM)),
        (String::from("R8"), (8, Address::RAM)),
        (String::from("R9"), (9, Address::RAM)),
        (String::from("R10"), (10, Address::RAM)),
        (String::from("R11"), (11, Address::RAM)),
        (String::from("R12"), (12, Address::RAM)),
        (String::from("R13"), (13, Address::RAM)),
        (String::from("R14"), (14, Address::RAM)),
        (String::from("R15"), (15, Address::RAM)),
        (String::from("SCREEN"), (16384, Address::RAM)),
        (String::from("KBD"), (24576, Address::RAM)),
    ]));
    let without_whitespace_and_comments = remove_whitespace_and_comments(content);
    let ran_through_first_pass = first_pass(without_whitespace_and_comments, &symbol_table);
    let ran_through_second_pass = second_pass(ran_through_first_pass, &symbol_table);

    file.write_all(ran_through_second_pass.as_bytes()).unwrap();
}

fn main() {
    let arg = env::args()
        .nth(1)
        .expect("You must provide the filepath of the program to assemble!");
    let path = path::PathBuf::from(arg);

    match path
        .extension()
        .unwrap_or_else(|| ffi::OsStr::new(""))
        .to_str()
        .unwrap_or("")
    {
        "asm" => process(&path),
        _ => panic!("The file extension must be asm!"),
    }
}
