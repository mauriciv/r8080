use std::env;
use std::process;

use r8080::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    match r8080::run(config) {
        Ok(asm) => println!("{}", asm),
        Err(e) => {
                eprintln!("Application error: {}", e);
                process::exit(1)
        },
    }
}
