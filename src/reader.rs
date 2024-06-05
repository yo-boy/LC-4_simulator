use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::path::PathBuf;

fn read_all_u16_values_from_file(path: &PathBuf) -> std::io::Result<Vec<u16>> {
    // Open the file in read-only mode
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);

    // Read all bytes from the file
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer)?;

    // Check that the number of bytes is even (u16 is 2 bytes)
    if buffer.len() % 2 != 0 {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "File size is not a multiple of 2 (u16 size)",
        ));
    }

    // Convert the bytes into u16 values
    let mut result = Vec::new();
    let mut index = 0;
    while index < buffer.len() {
        let value_bytes: [u8; 2] = [buffer[index], buffer[index + 1]];
        let value = u16::from_be_bytes(value_bytes);
        result.push(value);
        index += 2;
    }

    Ok(result)
}

// panic if PC writes into priveleged memory
fn check_pc(pc: u16) {
    if !(0x3000..=0xFDFF).contains(&pc) {
        panic!("bad binary images, PC: {}", pc)
    }
}

pub fn read_input_files(paths: &Vec<PathBuf>) -> [u16; 65536] {
    let mut out = [0u16; 65536];
    for path in paths {
        let values: Vec<u16> = read_all_u16_values_from_file(path).expect("error reading file");
        let mut pc = values[0];
        check_pc(pc);
        for value in values[1..].to_vec() {
            out[pc as usize] = value;
            pc += 1;
            check_pc(pc);
        }
    }
    out
}
