#![allow(dead_code)]
mod machine;
mod prng;
mod reader;
mod tokenizer;
use machine::Machine;
use reader::read_input_files;
use std::path::PathBuf;

fn main() {
    let file = PathBuf::from("./examples/out.bin");

    let mut files: Vec<PathBuf> = Vec::new();
    files.push(file);
    let out = read_input_files(files);

    for i in 0x3000..0x3011 {
        println!("0x{:04x}: {:016b}", i, out[i]);
    }
    println!();

    let mut lc4 = Machine::new(Some(out));

    lc4.run_machine().unwrap();
    println!("{:016b}", -12i16);
}

mod executer {}
