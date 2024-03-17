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

fn main() {
    let file = PathBuf::from("./examples/out.bin");

    let mut files: Vec<PathBuf> = Vec::new();
    files.push(file);
    let out = read_input_files(files);

    for i in 0x3000..0x3010 {
        println!("0x{:04x}: {:016b}", i, out[i]);
    }
}
