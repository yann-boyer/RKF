mod interpreter;
mod instructions;

use std::env;

use crate::interpreter::Interpreter;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        println!("Usage : ./rkf <bf-program>");
        std::process::exit(1);
    }

    let program_path = &args[1];

    let mut interpreter = Interpreter::new();
    interpreter.load_program(program_path);
    interpreter.execute_program();
}
