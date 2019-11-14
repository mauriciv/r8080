use std::error::Error;
use std::fs::File;
use std::io::Read;

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

    for byte in content.iter() {
        // println!("{:#04x}", byte);
        if *byte == 0x0u8 {
            println!("NOP Found");
        }
    }

    Ok(())
}

// fn is_instruction(byte: u8) {

// }