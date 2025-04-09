use brainf::BrainFuckVM;
use std::{env, fs, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <brainfuck-file>", args[0]);
        process::exit(1);
    }
    let filename = &args[1];

    match fs::read_to_string(filename) {
        Ok(contents) => {
            let mut vm = BrainFuckVM::new(contents);
            vm.exec();
        }
        Err(error) => {
            eprintln!("Error reading file '{}': {}", filename, error);
            process::exit(1);
        }
    }
}
