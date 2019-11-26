#![allow(unused_variables)]

mod opcodes;

use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::str;
use std::fmt::Write;

pub struct Config {
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        }

        let filename = args[1].clone();

        Ok(Config { filename })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut content: Vec<u8> = Vec::new();

    let mut file_handler = File::open(config.filename)?;

    file_handler.read_to_end(&mut content)?;

    let opcodes = opcodes::init_opcodes();

    let mut byte_number = 1;
    let mut current_opcode: &opcodes::Opcode;
    match opcodes.iter().position(|opcode| opcode.hex == content[0]) {
        Some(index) => current_opcode = &opcodes[index],
        None => panic!("First byte is not an opcode"),
    }
    let mut program = String::new();
    for &byte in content.iter() {

        if byte_number == 1 {
            if let Some(index) = opcodes.iter().position(|opcode| opcode.hex == byte) {
                // println!("{:#04x}    {:#03?}    {}", byte, opcodes[index].hex, opcodes[index].mnemonic);
                current_opcode = &opcodes[index];
                program.push_str(current_opcode.mnemonic);
                byte_number += 1;
                continue;
            }
        }

        // println!("byte_number {}", byte_number);
        // println!("current_opcode.size {}", current_opcode.size);

        if byte_number >= current_opcode.size {
            // println!("byte_number >= current_opcde.size");
            program.push_str("\n");
            byte_number = 1;

            continue;
        }

        write!(&mut program, " {:X}", byte).expect("Unable to write");
        byte_number += 1;
    }
    println!("{}", program);

    Ok(())
}

// fn is_instruction(byte: u8) {

// }
