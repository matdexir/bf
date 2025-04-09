use std::{collections::VecDeque, io::Read};

#[derive(Debug)]
pub struct BrainFuckVM {
    pc: usize,                 // Program counter
    cell_pc: usize,            // Keeping Track of the current cell
    instructions: Vec<OpCode>, // Program instructions
    registers: [u8; 10],       // Program Registers
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
            cell_pc: 0,
            instructions,
            registers: [0; 10],
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
        self.pc += 1;
    }

    fn decr_pc(&mut self) {
        self.pc -= 1;
    }

    pub fn exec(&mut self) {
        loop {
            if let Some(c) = &self.instructions.get(self.pc) {
                #[cfg(debug_assertions)]
                println!("c: {:?}", c);

                match c {
                    OpCode::IncrementCell => {
                        self.registers[self.cell_pc] += 1;
                        self.incr_pc();
                    }
                    OpCode::DecrementCell => {
                        self.registers[self.cell_pc] -= 1;
                        self.incr_pc();
                    }
                    OpCode::Increment => {
                        self.cell_pc += 1;
                        self.incr_pc();
                    }
                    OpCode::Decrement => {
                        self.cell_pc -= 1;
                        self.incr_pc();
                    }
                    OpCode::WriteCell => {
                        #[cfg(debug_assertions)]
                        {
                            self.buffered_output += &self.registers[self.cell_pc].to_string()
                        }

                        print!("{:?}", self.registers[self.cell_pc]);
                        self.incr_pc();
                    }
                    OpCode::ReadCell => {
                        let mut input_buf = [0];
                        let _ = std::io::stdin().read_exact(&mut input_buf);
                        self.registers[self.cell_pc] = input_buf[0];
                        self.incr_pc();
                    }
                    OpCode::LoopStart => {
                        if self.registers[self.cell_pc] == 0 {
                            loop {
                                let op_code = &self.instructions[self.pc];
                                match op_code {
                                    OpCode::LoopEnd => break,
                                    _ => self.incr_pc(),
                                }
                            }
                        }
                        self.incr_pc();
                    }
                    OpCode::LoopEnd => loop {
                        let op_code = &self.instructions[self.pc];
                        match op_code {
                            OpCode::LoopStart => break,
                            _ => self.decr_pc(),
                        }
                    },
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
