use std::{collections::VecDeque, io::Read};

#[derive(Debug)]
pub struct BrainFuckVM {
    pc: usize,            // Program counter
    cell_pc: usize,       // Keeping Track of the current cell
    instructions: String, // Program instructions
    registers: [u8; 10],  // Program Registers
    // loop_start_stack: Vec<usize>,
    // loop_end_queue: VecDeque<usize>,
    #[cfg(debug_assertions)]
    buffered_output: String,
}

#[derive(Debug)]
enum OpCode {
    IncrementPointer,
    DecrementPointer,
    Increment,
    Decrement,
    ReadCell,
    WriteCell,
    LoopStart,
    LoopEnd,
}

impl BrainFuckVM {
    pub fn new(content: String) -> Self {
        BrainFuckVM {
            pc: 0,
            cell_pc: 0,
            instructions: content,
            registers: [0; 10],
            // loop_start_stack: Vec::new(),
            // loop_end_queue: VecDeque::new(),
            #[cfg(debug_assertions)]
            buffered_output: String::from(""),
        }
    }

    fn incr_pc(&mut self) {
        self.pc += 1;
    }

    fn decr_pc(&mut self) {
        self.pc -= 1;
    }

    pub fn exec(&mut self) {
        loop {
            if let Some(c) = self.instructions.chars().nth(self.pc) {
                #[cfg(debug_assertions)]
                println!("c: {:?}", c);

                match c {
                    '+' => {
                        self.registers[self.cell_pc] += 1;
                        self.incr_pc();
                    }
                    '-' => {
                        self.registers[self.cell_pc] -= 1;
                        self.incr_pc();
                    }
                    '>' => {
                        self.cell_pc += 1;
                        self.incr_pc();
                    }
                    '<' => {
                        self.cell_pc -= 1;
                        self.incr_pc();
                    }
                    '.' => {
                        #[cfg(debug_assertions)]
                        {
                            self.buffered_output += &self.registers[self.cell_pc].to_string()
                        }

                        print!("{:?}", self.registers[self.cell_pc]);
                        self.incr_pc();
                    }
                    ',' => {
                        let mut input_buf = [0];
                        let _ = std::io::stdin().read_exact(&mut input_buf);
                        self.registers[self.cell_pc] = input_buf[0];
                        self.incr_pc();
                    }
                    '[' => {
                        /*
                        self.loop_start_stack.push(self.pc);
                        if self.registers[self.cell_pc] == 0 {
                            if let Some(pc) = self.loop_end_queue.pop_front() {
                                self.pc = pc + 1
                            } else {
                                while self.instructions.chars().nth(self.pc).unwrap() != ']' {
                                    self.incr_pc();
                                }
                                self.incr_pc();
                            }
                        } else {
                            self.incr_pc();
                        }
                        */
                        if self.registers[self.cell_pc] == 0 {
                            while self.instructions.chars().nth(self.pc).unwrap() != ']' {
                                self.incr_pc();
                            }
                        }
                        self.incr_pc();
                    }
                    ']' => {
                        /*
                        self.loop_end_queue.push_back(self.pc);
                        if let Some(pc) = self.loop_start_stack.pop() {
                            self.pc = pc + 1
                        } else {
                            panic!("No matching [ for the current ]")
                        }
                        */

                        while self.instructions.chars().nth(self.pc).unwrap() != '[' {
                            self.decr_pc();
                        }
                    }
                    _ => continue,
                }

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
