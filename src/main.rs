#![allow(dead_code)]
use termion::raw::IntoRawMode;
mod log;
mod machine;
mod prng;
mod reader;
mod tokenizer;
use crate::log::log;
use clap::command;
use machine::Machine;
use reader::read_input_files;
use std::fs::File;
use std::io::{stdin, stdout, Read, Write};
use std::path::PathBuf;
use std::thread::sleep;
use std::time::Duration;

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

    File::create("debug.log").unwrap();

    let out = read_input_files(&files);

    // log the binary that image that was read
    for (i, val) in out.iter().enumerate().filter(|(_i, x)| **x != 0) {
        log(&format!("{}: {:016b}\n", i, val));
    }
    log(&"\n");

    // Switch to raw mode and use an alternate screen
    let mut screen = stdout().into_raw_mode().unwrap();

    // Create a handle for standard input
    let input = stdin().lock();

    write!(screen, "test").unwrap();
    screen.flush().unwrap();

    let input = input.bytes();

    write!(
        screen,
        "{}{}{}LC-4 simulation.{}{}",
        termion::clear::All,
        termion::cursor::Goto(1, 1),
        termion::style::Bold,
        termion::style::Reset,
        termion::cursor::Goto(1, 2)
    )
    .unwrap();
    screen.flush().unwrap();

    let mut lc4 = Machine::new(Some(out), input, screen);

    let mut screen = stdout().into_raw_mode().unwrap();

    lc4.run_machine()?;
    write!(screen, "{}{}{} Halted execution", '\r', '\n', '\n').unwrap();
    screen.flush().unwrap();

    sleep(Duration::from_secs(2));

    Ok(())
}
