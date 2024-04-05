#![allow(dead_code)]
#![allow(unused_imports)]
mod prng;
mod reader;
mod tokenizer;
use prng::{ASG, LFSR};
use reader::read_input_files;
use std::path::PathBuf;
use tokenizer::{tokenize, Instruction};
use ux::{i3, u3, u7};

use crate::tokenizer::{check_instruction_double, Operation};

struct PSR {
    priority: u3,
    supervisor: bool,
    n: bool,
    p: bool,
    z: bool,
}

struct Machine {
    halt_flag: bool,
    asg: ASG,
    memory: [u16; 65536],
    // needs to be usize to use as an index into arrays
    pc: usize,
    register: [u16; 8],
    usp: u16,
    ssp: u16,
    psr: PSR,
}

impl Machine {
    pub fn new(mem: Option<[u16; 65536]>) -> Machine {
        Machine {
            halt_flag: true,
            asg: ASG::new(),
            memory: (match mem {
                Some(mem) => mem,
                None => [0b0u16; 65536],
            }),
            pc: 0x3000,
            register: [0b0u16; 8],
            usp: 0xFDFF,
            ssp: 0x2FFF,
            psr: PSR {
                priority: u3::new(0),
                supervisor: false,
                n: false,
                p: false,
                z: false,
            },
        }
    }

    // simulate a single instruction using the executor module
    fn simulate_instruction(&mut self) -> Result<(), String> {
        let double: bool = check_instruction_double(self.memory[self.pc]);
        if double {
            match tokenize(self.memory[self.pc], Some(self.memory[self.pc + 1])) {
                Ok(instruction) => match instruction.operation {
                    Operation::ADDi16 => Ok(()),
                    Operation::ADDa => Ok(()),
                    Operation::ANDi16 => Ok(()),
                    Operation::ANDa => Ok(()),
                    Operation::XORi16 => Ok(()),
                    Operation::XORa => Ok(()),
                    Operation::BR => Ok(()),
                    Operation::JSR => Ok(()),
                    Operation::LDa => Ok(()),
                    Operation::ST => Ok(()),
                    Operation::STR16 => Ok(()),
                    _ => {
                        Err("single length instruction in simulate_instruction double".to_string())
                    }
                },
                Err(error) => Err(error),
            }
        } else {
            match tokenize(self.memory[self.pc], None) {
                Ok(instruction) => match instruction.operation {
                    Operation::ADD => Ok(()),
                    Operation::ADDi => Ok(()),
                    Operation::AND => Ok(()),
                    Operation::ANDi => Ok(()),
                    Operation::XOR => Ok(()),
                    Operation::XORi => Ok(()),
                    Operation::JUMP => Ok(()),
                    Operation::RET => Ok(()),
                    Operation::JSRR => Ok(()),
                    Operation::LD => Ok(()),
                    Operation::STR => Ok(()),
                    Operation::NOT => Ok(()),
                    Operation::TRAP => Ok(()),
                    Operation::RTI => Ok(()),
                    Operation::LSD => Ok(()),
                    Operation::LPN => Ok(()),
                    Operation::CLRP => Ok(()),
                    Operation::HALT => Ok(self.halt_flag = false),
                    Operation::PUTS => Ok(()),
                    Operation::GETC => Ok(()),
                    Operation::OUT => Ok(()),
                    Operation::IN => Ok(()),
                    Operation::PUTSP => Ok(()),
                    _ => Err("double instruction in simulate_instruction double".to_string()),
                },
                Err(error) => Err(error),
            }
        }
    }

    // runs the machine until it reaches a halt instruction or exception
    fn run_machine(&mut self) -> Result<(), &str> {
        while self.halt_flag & (self.pc < 0xFE00) {
            if check_instruction_double(self.memory[self.pc]) {
                println!(
                    "exectuing: {:?}",
                    tokenize(self.memory[self.pc], Some(self.memory[self.pc + 1])).unwrap()
                );
                self.simulate_instruction().unwrap();
                self.pretty_print();
                self.pc += 2;
            } else {
                println!(
                    "executing: {:?}",
                    tokenize(self.memory[self.pc], None).unwrap()
                );
                self.simulate_instruction().unwrap();
                self.pretty_print();
                self.pc += 1;
            }
        }
        Ok(())
    }

    // pretty print all info
    fn pretty_print(&mut self) {
        println!("PC: 0x{:04x}", self.pc);
        self.print_registers();
        println!("");
        self.print_pretty_memory();
        println!("");
    }

    // print memory around PC
    fn print_pretty_memory(&mut self) {
        for i in (self.pc)..(self.pc + 2) {
            println!("0x{:04x}: {:016b}", i, self.memory[i]);
        }
    }

    // print registers in a pretty way
    pub fn print_registers(&self) {
        for (i, reg) in self.register.iter().enumerate() {
            print!("R{}: {}\t", i, reg)
        }
    }

    // print all the modified parts of memory in a pretty way
    fn print_modified_memory(&mut self) {
        todo!()
    }
}

fn main() {
    let file = PathBuf::from("./examples/out.bin");

    let mut files: Vec<PathBuf> = Vec::new();
    files.push(file);
    let out = read_input_files(files);

    for i in 0x3000..0x3011 {
        println!("0x{:04x}: {:016b}", i, out[i]);
    }
    println!("");

    let mut lc4 = Machine::new(Some(out));

    lc4.run_machine().unwrap();
}

mod executer {
    use crate::{tokenizer::Instruction, Machine};
}

// note to self, this is important, you need to work on the read execute cycle now, and make sure to handle unexpected data the same way a processor should (exception I assume)
