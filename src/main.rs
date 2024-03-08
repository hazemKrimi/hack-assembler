use std::env;
use std::ffi;
use std::fs;
use std::io::Write;
use std::path;
use std::process;

use regex::Regex;
use types::Instruction;

mod parser;
mod types;

fn process(instruction: String) -> String {
    // let parsed = parser::parse(instruction);

    // match parsed {
    //     Instruction::AInstruction { .. } => {
    //         let a_instruction: Instruction::AInstruction = parsed;

    //         println!("{}", a_instruction.decimal)
    //     },
    //     _ => ()
    // }

    instruction
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
            let without_whitespace: String = content
                .lines()
                .filter(|line| !line.starts_with("//") && !line.is_empty())
                .map(|line| re.replace_all(line, ""))
                .map(|line| process(line.to_string()))
                .map(|line| format!("{}\n", line))
                .collect();

            file.write_all(without_whitespace.as_bytes()).unwrap();
        }
        _ => {
            println!("The file extension must be asm!");
            process::exit(1)
        }
    }
}
