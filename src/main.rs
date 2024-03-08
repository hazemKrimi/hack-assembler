use std::env;
use std::ffi;
use std::fs;
use std::io::Write;
use std::path;
use std::process;

use regex::Regex;
use types::Instruction;

mod types;
mod parser;
mod code;

fn process(instruction: String) -> String {
    match parser::parse(&instruction) {
        Some(parsed) => match parsed {
            Instruction::AInstruction(parsed_instruction) => {
                let translated = code::decimal_to_binary(&parsed_instruction.decimal.parse::<i32>().unwrap());

                String::from(format!("0{}", translated))
            }
            Instruction::CInstruction(parsed_instruction) => {
                println!(
                    "C: {}, {}, {}",
                    parsed_instruction.dest,
                    parsed_instruction.comp,
                    parsed_instruction.jump.unwrap_or_else(|| "".to_string())
                );

                instruction
            }
        },
        None => {
            println!("Unexpected error!");
            process::exit(1)
        }
    }
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
        "asm" => {
            let filename = path.file_stem().unwrap().to_str().unwrap();
            let mut file = fs::File::create(format!("{}.hack", filename)).unwrap();
            let content = fs::read_to_string(&path).expect("You must provide a correct filepath!");
            let re = Regex::new(r"\s*\/\/.*").unwrap();
            let processed: String = content
                .lines()
                .filter(|line| !line.starts_with("//") && !line.is_empty())
                .map(|line| re.replace_all(line, ""))
                .map(|line| process(line.to_string()))
                .map(|line| format!("{}\n", line))
                .collect();

            file.write_all(processed.as_bytes()).unwrap();
        }
        _ => {
            println!("The file extension must be asm!");
            process::exit(1)
        }
    }
}