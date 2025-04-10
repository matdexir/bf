use std::{
    collections::VecDeque,
    io::{self, Read, Write},
};

const MEMORY_SIZE: usize = 1000;

#[derive(Debug)]
pub struct BrainFuckVM {
    pc: usize,                 // Program counter
    cc: usize,                 // Keeping Track of the current cell
    instructions: Vec<OpCode>, // Program instructions
    tape: [u8; MEMORY_SIZE],   // Program Registers
    // loop_start_stack: Vec<usize>,
    // loop_end_queue: VecDeque<usize>,
    #[cfg(debug_assertions)]
    buffered_output: String,
}

#[derive(Debug)]
enum OpCode {
    IncrementCell,
    DecrementCell,
    Increment,
    Decrement,
    ReadCell,
    WriteCell,
    LoopStart,
    LoopEnd,
}

impl BrainFuckVM {
    pub fn new(content: String) -> Self {
        let instructions = BrainFuckVM::lex(content);

        BrainFuckVM {
            pc: 0,
            cc: 0,
            instructions,
            tape: [0; MEMORY_SIZE],
            // loop_start_stack: Vec::new(),
            // loop_end_queue: VecDeque::new(),
            #[cfg(debug_assertions)]
            buffered_output: String::from(""),
        }
    }

    fn lex(instructions: String) -> Vec<OpCode> {
        let mut op_codes = Vec::new();
        for c in instructions.chars() {
            match c {
                '+' => op_codes.push(OpCode::IncrementCell),
                '-' => op_codes.push(OpCode::DecrementCell),
                '>' => op_codes.push(OpCode::Increment),
                '<' => op_codes.push(OpCode::Decrement),
                '.' => op_codes.push(OpCode::WriteCell),
                ',' => op_codes.push(OpCode::ReadCell),
                '[' => op_codes.push(OpCode::LoopStart),
                ']' => op_codes.push(OpCode::LoopEnd),

                _ => continue,
            };
        }
        return op_codes;
    }

    fn incr_pc(&mut self) {
        self.pc = self.pc.wrapping_add(1);
    }

    fn decr_pc(&mut self) {
        self.pc = self.pc.wrapping_sub(1);
    }

    pub fn exec(&mut self) {
        while self.pc < self.instructions.len() {
            if let Some(c) = &self.instructions.get(self.pc) {
                #[cfg(debug_assertions)]
                println!("c: {:?}", c);

                match c {
                    OpCode::IncrementCell => {
                        self.tape[self.cc] = self.tape[self.cc].wrapping_add(1);
                    }
                    OpCode::DecrementCell => {
                        self.tape[self.cc] = self.tape[self.cc].wrapping_sub(1);
                    }
                    OpCode::Increment => {
                        // self.cc = self.cc.wrapping_add(1);
                        self.cc += 1
                    }
                    OpCode::Decrement => {
                        // self.cc = self.cc.wrapping_sub(1);
                        self.cc -= 1
                    }
                    OpCode::WriteCell => {
                        #[cfg(debug_assertions)]
                        {
                            self.buffered_output += &(self.tape[self.cc] as char).to_string()
                        }

                        let _ = io::stdout().write_all(&self.tape[self.cc..self.cc + 1]);
                    }
                    OpCode::ReadCell => {
                        let _ = io::stdin().read_exact(&mut self.tape[self.cc..self.cc + 1]);
                    }
                    OpCode::LoopStart => {
                        if self.tape[self.cc] == 0 {
                            let mut level = 1;
                            while level > 0 {
                                self.pc += 1;

                                match self.instructions[self.pc] {
                                    OpCode::LoopStart => level += 1,
                                    OpCode::LoopEnd => level -= 1,
                                    _ => {}
                                }
                            }
                        }
                    }
                    OpCode::LoopEnd => {
                        if self.tape[self.cc] != 0 {
                            let mut level = 1;
                            while level > 0 {
                                self.pc -= 1;
                                match self.instructions[self.pc] {
                                    OpCode::LoopStart => level -= 1,
                                    OpCode::LoopEnd => level += 1,
                                    _ => {}
                                }
                            }
                        }
                    }
                }
                self.incr_pc();

                #[cfg(debug_assertions)]
                self.inspect();
            } else {
                #[cfg(debug_assertions)]
                println!("Execution is done.");

                break;
            }
        }
    }

    pub fn inspect(&self) {
        println!("registers: {:?}", self);
    }
}
