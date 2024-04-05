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
    asg: ASG,
    memory: [u16; 65536],
    pc: u16,
    register: [u16; 8],
    usp: u16,
    ssp: u16,
    psr: PSR,
}

impl Machine {
    pub fn new(mem: Option<[u16; 65536]>) -> Machine {
        Machine {
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
    pub fn print_registers(&self) {
        for (i, reg) in self.register.iter().enumerate() {
            print!("R{}: {}\t", i, reg)
        }
    }
    fn simulate_instruction(&mut self) -> Result<(), &str> {
        Ok(())
    }
    // print all the modified parts of memory in a pretty way
    fn print_modified_memory(&mut self) {
        todo!()
    }

    fn add(&mut self, inst: Instruction) {}
}

fn main() {
    let file = PathBuf::from("./examples/out.bin");

    let mut files: Vec<PathBuf> = Vec::new();
    files.push(file);
    let out = read_input_files(files);

    for i in 0x3000..0x3011 {
        println!("0x{:04x}: {:016b}", i, out[i]);
    }

    let lc4 = Machine::new(Some(out));

    println!("{:?}", tokenize(out[0x3001], None));

    let mut halt_flag: bool = true;
    let mut i = 0x3000;
    while (i < 0x3011) & (halt_flag) {
        if check_instruction_double(out[i]) {
            println!("{:?}", tokenize(out[i], Some(out[i + 1])));
            i += 2;
        } else {
            println!("{:?}", tokenize(out[i], None));
            if tokenize(out[i], None).operation == Operation::HALT {
                halt_flag = false;
            }
            i += 1;
        }
    }
}

// note to self, this is important, you need to work on the read execute cycle now, and make sure to handle unexpected data the same way a processor should (exception I assume)
fn tokenize_helper() {
    todo!()
}
