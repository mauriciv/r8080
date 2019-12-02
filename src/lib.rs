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

pub fn run(config: Config) -> Result<(String), Box<dyn Error>> {
    let mut content: Vec<u8> = Vec::new();

    let mut file_handler = File::open(config.filename)?;

    file_handler.read_to_end(&mut content)?;

    let opcodes = opcodes::init();

    let mut byte_number = 0;
    let mut current_opcode: &opcodes::Opcode;
    match get_opcode(&opcodes, content[0]) {
        Some(index) => current_opcode = &opcodes[index],
        None => panic!("First byte is not an opcode"),
    }
    let mut program = String::new();
    let mut program_counter = 0;
    let mut byte_buffer = String::new();
    let mut line = String::new();
    for &byte in &content {

        if byte_number == 0 {
            if let Some(index) = get_opcode(&opcodes, byte) {
                write!(&mut line, "{:#06x}  ", program_counter).expect("Unable to write to program_counter");
                current_opcode = &opcodes[index];
                line.push_str(current_opcode.mnemonic);
                byte_number += 1;
                program_counter += 1;
                if byte_number == current_opcode.size {
                    line.push_str("\n");
                    program.push_str(&line);
                    line = String::from("");
                    byte_number = 0;
                    continue;
                }
                continue;
            }
        }

        program_counter += 1;

        if byte_number == 1 {
            if current_opcode.size == 2 {
                write!(&mut line, " {:02x}", byte).expect("Unable to write");
                byte_buffer = String::from("");
            } else {
                write!(&mut byte_buffer, "{:02x}", byte).expect("Unable to write to byte_buffer");
            }
        } else if byte_number == 2 {
            write!(&mut line, " {:02x}", byte).expect("Unable to write");
            line.push_str(&byte_buffer);
            byte_buffer = String::from("");
        }

        if byte_number == current_opcode.size - 1{
            line.push_str("\n");
            program.push_str(&line);
            line = String::from("");
            byte_number = 0;
            continue;
        }
        byte_number += 1;
    }

    Ok(program)
}

fn get_opcode(opcodes: &Vec<opcodes::Opcode>, byte: u8) -> std::option::Option<usize> {
    opcodes.iter().position(|opcode| opcode.hex == byte)
}

// fn is_instruction(byte: u8) {

// }
#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;
    use std::io::prelude::Write;
    use std::fs::File;

    #[test]
    fn it_disassembles_the_byte_0x16() {
        create_test_fixture("test.h", &vec![0x16, 0xA]);
        let args: Vec<String> = vec![String::from(""), String::from("test.h")];
        let config = Config::new(&args).expect("Error");

        let asm = super::run(config).expect("Could not run test");

        assert_eq!("0x0000  MVI D, 0a\n", asm);
    }

    #[test]
    fn it_disassembles_the_byte_0x01() {
        create_test_fixture("test.h", &vec![0x01, 0xAA]);
        let args: Vec<String> = vec![String::from(""), String::from("test.h")];
        let config = Config::new(&args).expect("Error");

        let asm = super::run(config).expect("Could not run test");

        assert_eq!("0x0000  MVI D, 0a0a\n", asm);
    }

    fn create_test_fixture(name: &str, bytes: &Vec<u8>) {
        let path_str = "./".to_owned() + name;
        let path = Path::new(&path_str);
        let display = path.display();

        // Open a file in write-only mode, returns `io::Result<File>`
        let mut file = match File::create(&path) {
            Err(why) => panic!("couldn't create {}: {}", display, why.description()),
            Ok(file) => file,
        };

        if let Err(why) = file.write_all(&bytes) {
            panic!("couldn't write to {}: {}", display, why.description());
        }
    }
}
