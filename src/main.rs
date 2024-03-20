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

    fn match_opcode(instruction: u16) -> Operation {
        match instruction >> 11 & 0b11111 {
            0b00001 => parse_add(instruction),
            0b00010 => parse_and(instruction),
            0b00011 => parse_xor(instruction),
            0b00100 => Operation::BR,
            0b00101 => {
                if instruction >> 7 & 0b111 == 0b111 {
                    Operation::RET
                } else {
                    Operation::JUMP
                }
            }
            0b00110 => {
                if check_10(instruction) {
                    Operation::JSR
                } else {
                    Operation::JSRR
                }
            }
            0b01000 => {
                if check_10(instruction) {
                    Operation::LDa
                } else {
                    Operation::LD
                }
            }
            0b01001 => Operation::ST,
            0b00111 => {
                if check_10(instruction) {
                    Operation::STR16
                } else {
                    Operation::STR
                }
            }
            0b01010 => Operation::NOT,
            0b01100 => parse_trap(instruction),
            0b01101 => Operation::RTI,
            _ => panic!("invalid instruction"),
        }
    }

    fn check_10(instruction: u16) -> bool {
        instruction >> 10 & 0b1 == 1
    }

    fn parse_trap(instruction: u16) -> Operation {
        let instruction = instruction & 0b11111111;
        match instruction {
            0x20 => Operation::GETC,
            0x21 => Operation::OUT,
            0x22 => Operation::PUTS,
            0x23 => Operation::IN,
            0x24 => Operation::PUTSP,
            0x25 => Operation::HALT,
            0x26 => Operation::LSD,
            0x27 => Operation::LPN,
            0x28 => Operation::CLRP,
            _ => Operation::TRAP,
        }
    }

    fn parse_add(instruction: u16) -> Operation {
        if instruction >> 10 & 0b1 == 0 {
            if instruction >> 3 & 0b1 == 1 {
                Operation::ADDi
            } else {
                Operation::ADD
            }
        } else {
            if instruction >> 3 & 0b1 == 1 {
                Operation::ADDi16
            } else {
                Operation::ADDa
            }
        }
    }
    fn parse_and(instruction: u16) -> Operation {
        if instruction >> 10 & 0b1 == 0 {
            if instruction >> 3 & 0b1 == 1 {
                Operation::ANDi
            } else {
                Operation::AND
            }
        } else {
            if instruction >> 3 & 0b1 == 1 {
                Operation::ANDi16
            } else {
                Operation::ANDa
            }
        }
    }
    fn parse_xor(instruction: u16) -> Operation {
        if instruction >> 10 & 0b1 == 0 {
            if instruction >> 3 & 0b1 == 1 {
                Operation::XORi
            } else {
                Operation::XOR
            }
        } else {
            if instruction >> 3 & 0b1 == 1 {
                Operation::XORi16
            } else {
                Operation::XORa
            }
        }
    }

    pub fn tokenize(encoded_instruction: u16, second_operand: Option<u16>) {
        let operation = match_opcode(encoded_instruction);
    }

    pub struct Instruction {
        op: Operation,
    }
}
