#![allow(dead_code)]
mod log;
mod machine;
mod prng;
mod reader;
mod term;
mod tokenizer;
use crate::log::log;
use clap::command;
use machine::Machine;
use reader::read_input_files;
use std::fs::File;
use std::path::PathBuf;

fn main() -> Result<(), String> {
    let mut files: Vec<PathBuf> = Vec::new();
    let matches = command!()
        .about("Simulator for the LC-4 architecture.")
        .arg(
            clap::Arg::new("input")
                .default_value("./examples/out.bin")
                .value_parser(clap::value_parser!(PathBuf))
                .help("assembly input file"),
        )
        .get_matches();
    files.push(
        matches
            .get_one::<PathBuf>("input")
            .expect("could not parse input file path")
            .to_owned(),
    );

    File::create("lc4-debug.log").unwrap();

    let out = read_input_files(&files);

    for i in 0x3000..0x3011 {
        log(&format!("0x{:04x}: {:016b}\n", i, out[i]));
    }

    let mut lc4 = Machine::new(Some(out));

    lc4.run_machine()
}
