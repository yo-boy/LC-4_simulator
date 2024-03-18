#![allow(dead_code)]
#![allow(unused_imports)]
mod prng;
mod reader;
use prng::{ASG, LFSR};
use reader::read_input_files;
use std::path::PathBuf;
use ux::u3;

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
    r0: u16,
    r1: u16,
    r2: u16,
    r3: u16,
    r4: u16,
    r6: u16,
    r7: u16,
    usp: u16,
    ssp: u16,
    psr: PSR,
}

impl Machine {
    pub fn new() -> Machine {
        Machine {
            asg: ASG::new(),
            memory: [0b0u16; 65536],
            pc: 0,
            r0: 0,
            r1: 0,
            r2: 0,
            r3: 0,
            r4: 0,
            r6: 0,
            r7: 0,
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
    fn simulate_instruction(&mut self) {}
}

fn main() {
    let file = PathBuf::from("./examples/out.bin");

    let mut files: Vec<PathBuf> = Vec::new();
    files.push(file);
    let out = read_input_files(files);

    for i in 0x3000..0x3011 {
        println!("0x{:04x}: {:016b}", i, out[i]);
    }
}

mod tokenizer {
    pub enum Operation {
        ADD,
        ADDi,
        ADDi16,
        ADDa,
        AND,
        ANDi,
        ANDi16,
        ANDa,
        XOR,
        XORi,
        XORi16,
        XORa,
        BR,
        JUMP,
        RET,
        JSR,
        JSRR,
        LD,
        LDa,
        ST,
        STR,
        STR16,
        NOT,
        TRAP,
        RTI,
        LSD,
        LPN,
        CLRP,
        HALT,
        PUTS,
        GETC,
        OUT,
        IN,
        PUTSP,
    }

    pub(crate) fn match_opcode_short(instruction: u16) -> Operation {
        match instruction >> 11 & 0b11111 {
            _ => todo!(),
        }
    }

    pub(crate) fn match_opcode_long(instruction: u16) -> Operation {
        match instruction >> 11 & 0b11111 {
            _ => todo!(),
        }
    }

    pub(crate) fn tokenize(encoded_instruction: u16, second_operand: Option<u16>) {
        if encoded_instruction >> 10 & 0b1 == 0 {
        } else {
        }
    }
}
