# Description
This is a simulator for the LC-4 architecture that is written entirely in Rust, it reads binary programs produced by the assembler and simulates their execution as it would happen in the LC-4 architecture and allows the user to input characters, the LC-4 architecture is not currently published anywhere, but it will be, and will be linked here at that time. 

# Usage
The simulator is a CLI tool that takes input binary program files in the format that the LC-4 architecture specifies and the LC-4 assembler outputs, it can take multiple files and place them in the correct areas of memory  (examples of programs can be found in this repo and the assembler repo), running the simulator with the `--help` flag gives usage information

```
Simulator for the LC-4 architecture.

Usage: lc-4_simulator [input]

Arguments:
  [input]  assembly input file [default: ./examples/out.bin]

Options:
  -h, --help     Print help
  -V, --version  Print version
```
