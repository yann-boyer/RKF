use std::fs::File;
use std::io::{self, Read, Write};
use std::collections::HashMap;

use crate::instructions::Instructions;

const RAM_SIZE: usize = 65536;
const RAM_END: usize = RAM_SIZE - 1;

pub struct Interpreter {
    ram: [u8; RAM_SIZE],
    program: Vec<Instructions>,
    jump_map: HashMap<u16, u16>,
    ptr: u32,
    pc: u16
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {
            ram: [0; RAM_SIZE],
            program: Vec::new(),
            jump_map: HashMap::new(),
            ptr: 0,
            pc: 0
        }
    }

    pub fn load_program(&mut self, program_path: &str) {
        let mut program_buffer = String::new();

        match File::open(program_path) {
            Ok(mut program_file) => {
                match program_file.read_to_string(&mut program_buffer) {
                    Ok(_) => (),
                    Err(why) => {
                        println!("Error : Unable to read the given file !");
                        println!("Why -> {:?}", why);
                    }
                }
            },
            Err(why) => {
                println!("Error : Unable to open the given file !");
                println!("Why -> {:?}", why);
            }
        }

        for symbol in program_buffer.chars() {
            match symbol {
                '>' => self.program.push(Instructions::IncrementPointer),
                '<' => self.program.push(Instructions::DecrementPointer),
                '+' => self.program.push(Instructions::IncrementByte),
                '-' => self.program.push(Instructions::DecrementByte),
                '.' => self.program.push(Instructions::WriteByte),
                ',' => self.program.push(Instructions::ReadByte),
                '[' => self.program.push(Instructions::JumpForward),
                ']' => self.program.push(Instructions::JumpBackward),
                _ => (),
            }
        }
    }

    fn precompute_jumps(&mut self) {
        let mut stack: Vec<u16> = Vec::new();

        let mut pc_: u16 = 0;
        let mut target;

        while pc_ as usize != self.program.len() {
            let instr = self.program[pc_ as usize];

            match instr {
                Instructions::JumpForward => stack.push(pc_),
                Instructions::JumpBackward => {
                    target = stack.pop().unwrap();
                    self.jump_map.insert(target, pc_);
                    self.jump_map.insert(pc_, target);
                },
                _ => ()
            }

            pc_ += 1;
        }
    }

    pub fn execute_program(&mut self) {
        self.precompute_jumps();

        while self.pc as usize != self.program.len() {
            let instr = self.program[self.pc as usize];

            match instr {
                Instructions::IncrementPointer => {
                    if self.ptr > RAM_END as u32 {self.ptr = 0;}
                    self.ptr += 1;
                },
                Instructions::DecrementPointer => {
                    if self.ptr == 0 {self.ptr = RAM_END as u32 - 1;}
                    self.ptr -= 1;
                },
                Instructions::IncrementByte => {
                    let prev_val = self.ram[self.ptr as usize];
                    self.ram[self.ptr as usize] = prev_val.wrapping_add(1);
                },
                Instructions::DecrementByte => {
                    let prev_val = self.ram[self.ptr as usize];
                    self.ram[self.ptr as usize] = prev_val.wrapping_sub(1);
                },
                Instructions::WriteByte => {
                    print!("{}", self.ram[self.ptr as usize] as char);
                    io::stdout().flush().unwrap();
                },
                Instructions::ReadByte => {
                    let mut input: [u8; 1] = [0; 1];
                    io::stdin().read_exact(&mut input).expect("Error : Unable to read stdin.");
                    self.ram[self.ptr as usize] = input[0];
                },
                Instructions::JumpForward => {
                    if self.ram[self.ptr as usize] == 0 {
                        self.pc = *self.jump_map.get(&self.pc).unwrap();
                    }
                },
                Instructions::JumpBackward => {
                    if self.ram[self.ptr as usize] != 0 {
                        self.pc = *self.jump_map.get(&self.pc).unwrap();
                    }
                }
            }

            self.pc += 1;
        }
    }
}
