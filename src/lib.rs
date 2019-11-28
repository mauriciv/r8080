#![allow(unused_variables)]

mod opcodes;

use std::error::Error;
use std::fmt::Write;
use std::fs::File;
use std::io::Read;
use std::str;

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

    let opcodes = opcodes::init();

    let mut byte_number = 0;
    let mut current_opcode: &opcodes::Opcode;
    // match opcodes.iter().position(|opcode| opcode.hex == content[0]) {
    match get_opcode(&opcodes, content[0]) {
        Some(index) => current_opcode = &opcodes[index],
        None => panic!("First byte is not an opcode"),
    }
    let mut program = String::new();
    let mut program_counter = 0;
    for &byte in &content {
        // println!("Byte is :{:#04x}", byte);
        if byte_number == 0 {
            if let Some(index) = get_opcode(&opcodes, byte) {
                // println!("{:#04x}    {:#03?}    {}", byte, opcodes[index].hex, opcodes[index].mnemonic);
                write!(&mut program, "{:#06X}  ", program_counter)
                    .expect("Unable to write program_counter");
                program_counter += 1;
                current_opcode = &opcodes[index];
                program.push_str(current_opcode.mnemonic);
                byte_number += 1;
                if byte_number == current_opcode.size {
                    // println!("byte_number >= current_opcde.size");
                    program.push_str("\n");
                    byte_number = 0;
                    continue;
                }
                continue;
            }
        }

        write!(&mut program, " {:X}", byte).expect("Unable to write");

        if byte_number == current_opcode.size - 1{
            // println!("byte_number >= current_opcde.size");
            program.push_str("\n");
            byte_number = 0;
            continue;
        }
        byte_number += 1;
    }
    println!("{}", program);

    Ok(())
}

fn get_opcode(opcodes: &Vec<opcodes::Opcode>, byte: u8) -> std::option::Option<usize> {
    opcodes.iter().position(|opcode| opcode.hex == byte)
}

// fn is_instruction(byte: u8) {

// }
