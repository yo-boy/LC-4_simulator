use crate::log::log;
use crate::prng::ASG;
use crate::tokenizer::{check_instruction_double, tokenize, Instruction, Operand, Operation};
use std::io::StdinLock;
use std::io::Write;

use ux::u3;

struct TerminalHandles<'a, W: Write> {
    pub input: std::io::Bytes<StdinLock<'a>>,
    pub output: W,
}

struct PSR {
    priority: u3,
    supervisor: bool,
    n: bool,
    p: bool,
    z: bool,
}

struct Position {
    x: u16,
    y: u16,
}

pub struct Machine<'a, W: Write> {
    cursor: Position,
    // implementing a buffer would be more consistent with the hardware
    //input_buffer: Vec<u8>,
    term: TerminalHandles<'a, W>,
    halt_flag: bool,
    asg: ASG,
    memory: [u16; 65536],
    // needs to be usize to use as an index into arrays
    pc: usize,
    register: [i16; 8],
    usp: u16,
    ssp: u16,
    psr: PSR,
}

impl<'a, W: Write> Machine<'a, W> {
    pub fn new(
        mem: Option<[u16; 65536]>,
        input: std::io::Bytes<StdinLock<'a>>,
        output: W,
    ) -> Machine<W> {
        Machine {
            cursor: Position { x: 1, y: 1 },
            //input_buffer: Vec::new(),
            term: TerminalHandles { input, output },
            halt_flag: true,
            asg: ASG::new(),
            memory: mem.unwrap_or([0b0u16; 65536]),
            pc: 0x3000,
            register: [0b0i16; 8],
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

    // set the result registers
    fn setcc(&mut self, value: i16) {
        if value > 0 {
            self.psr.p = true;
            self.psr.n = false;
            self.psr.z = false;
        } else if value == 0 {
            self.psr.p = false;
            self.psr.n = false;
            self.psr.z = true;
        } else {
            self.psr.p = false;
            self.psr.n = true;
            self.psr.z = false;
        }
    }

    // simulate a single instruction using the executor module
    fn simulate_instruction(&mut self) -> Result<(), String> {
        let double: bool = check_instruction_double(self.memory[self.pc]);
        if double {
            match tokenize(self.memory[self.pc], Some(self.memory[self.pc + 1])) {
                Ok(instruction) => match instruction.operation {
                    Operation::ADDi16 => self.execute_double_def(instruction),
                    Operation::ADDa => self.execute_double_def(instruction),
                    Operation::ANDi16 => self.execute_double_def(instruction),
                    Operation::ANDa => self.execute_double_def(instruction),
                    Operation::XORi16 => self.execute_double_def(instruction),
                    Operation::XORa => self.execute_double_def(instruction),
                    Operation::BR => self.br(&instruction),
                    Operation::JSR => self.jsr(&instruction),
                    Operation::LDa => self.lda(instruction),
                    Operation::ST => self.st(instruction),
                    Operation::STR16 => self.str16(instruction),
                    _ => {
                        Err("single length instruction in simulate_instruction double".to_string())
                    }
                },
                Err(error) => Err(error),
            }
        } else {
            match tokenize(self.memory[self.pc], None) {
                Ok(instruction) => match instruction.operation {
                    Operation::ADD => self.execute_def(instruction),
                    Operation::ADDi => self.execute_def(instruction),
                    Operation::AND => self.execute_def(instruction),
                    Operation::ANDi => self.execute_def(instruction),
                    Operation::XOR => self.execute_def(instruction),
                    Operation::XORi => self.execute_def(instruction),
                    Operation::JUMP => self.jump(instruction),
                    Operation::RET => self.ret(),
                    Operation::JSRR => self.jsrr(instruction),
                    Operation::LD => self.ld(instruction),
                    Operation::STR => self.str(instruction),
                    Operation::NOT => self.not(instruction),
                    Operation::TRAP => Ok(()), //this will not happen
                    Operation::RTI => self.rti(),
                    Operation::LSD => self.lsd(),
                    Operation::LPN => self.lpn(),
                    Operation::CLRP => self.clrp(),
                    Operation::HALT => {
                        self.halt_flag = false;
                        Ok(())
                    }
                    Operation::PUTS => self.puts(),
                    Operation::GETC => self.getc(),
                    Operation::OUT => self.out(),
                    Operation::IN => self.in_trap(),
                    Operation::PUTSP => self.putsp(),
                    _ => Err("double instruction in simulate_instruction double".to_string()),
                },
                Err(error) => Err(error),
            }
        }
    }

    fn puts(&mut self) -> Result<(), String> {
        let mut addr = self.register[0] as usize;
        let mut out = self.memory[addr] as u8 as char;
        while out != 0x0000 as char {
            match write!(self.term.output, "{}", out) {
                Ok(()) => Ok(()),
                Err(_) => Err("couldn't write to terminal".to_owned()),
            }?;
            match self.term.output.flush() {
                Ok(()) => Ok(()),
                Err(_) => Err("couldn't write to terminal".to_owned()),
            }?;
            addr += 1;
            out = self.memory[addr] as u8 as char;
        }
        Ok(())
    }
    fn in_trap(&mut self) -> Result<(), String> {
        // get cursor position
        //let (_x, y) = self.term.output.cursor_pos().unwrap();
        //let y = y + 1;
        // go to next line and print input prompt for user
        match write!(self.term.output, "{}{}input: ", '\n', '\r') {
            Ok(_) => Ok(()),
            Err(_) => Err("couldn't write to terminal".to_owned()),
        }?;
        // flush to terminal
        match self.term.output.flush() {
            Ok(()) => Ok(()),
            Err(_) => Err("couldn't write to terminal".to_owned()),
        }?;
        // block and read input
        let key = match self.term.input.next() {
            Some(key) => match key {
                Ok(key) => Ok(key),
                Err(_) => Err("couldn't read input".to_owned()),
            }?,
            None => b'\0',
        };
        // echo key and place cursor on next line
        match write!(self.term.output, "{}{}{}", key as char, '\n', '\r') {
            Ok(_) => Ok(()),
            Err(_) => Err("couldn't write to terminal".to_owned()),
        }?;
        match self.term.output.flush() {
            Ok(()) => Ok(()),
            Err(_) => Err("couldn't write to terminal".to_owned()),
        }?;
        self.register[0] = key as i16;
        Ok(())
    }
    fn getc(&mut self) -> Result<(), String> {
        let key = match self.term.input.next() {
            Some(key) => match key {
                Ok(key) => Ok(key),
                Err(_) => Err("couldn't read input".to_owned()),
            }?,
            None => b'\0',
        };
        self.register[0] = key as i16;
        Ok(())
    }
    fn out(&mut self) -> Result<(), String> {
        let out = self.register[0].to_be_bytes()[1] as char;
        match write!(self.term.output, "{}", out) {
            Ok(()) => Ok(()),
            Err(_) => Err("couldn't write to terminal".to_owned()),
        }?;
        match self.term.output.flush() {
            Ok(()) => Ok(()),
            Err(_) => Err("couldn't write to terminal".to_owned()),
        }?;
        Ok(())
    }

    fn putsp(&mut self) -> Result<(), String> {
        let mut addr = self.register[0] as usize;
        let mut out = self.memory[addr].to_be_bytes();
        while out[0] != 0x00 {
            match write!(self.term.output, "{}{}", out[0] as char, out[1] as char) {
                Ok(()) => Ok(()),
                Err(_) => Err("couldn't write to terminal".to_owned()),
            }?;
            addr += 1;
            out = self.memory[addr].to_be_bytes();
        }
        match self.term.output.flush() {
            Ok(()) => Ok(()),
            Err(_) => Err("couldn't write to terminal".to_owned()),
        }?;
        Ok(())
    }

    fn str16(&mut self, instruction: Instruction) -> Result<(), String> {
        let addr = self.register[instruction_to_dr(&instruction)?] as usize;
        let value = instruction_to_imm16(&instruction)?;
        self.memory[addr] = value as u16;
        Ok(())
    }

    fn st(&mut self, instruction: Instruction) -> Result<(), String> {
        let sr = instruction_to_dr(&instruction)?;
        let addr = instruction_to_addr(&instruction)? as usize;
        self.acv_exception(addr)?;
        self.memory[addr] = self.register[sr] as u16;
        Ok(())
    }

    fn lda(&mut self, instruction: Instruction) -> Result<(), String> {
        let dr = instruction_to_dr(&instruction)?;
        let addr = instruction_to_addr(&instruction)? as usize;
        let value = self.memory[addr] as i16;
        self.setcc(value);
        self.register[dr] = value;
        Ok(())
    }

    fn jsr(&mut self, instruction: &Instruction) -> Result<(), String> {
        let addr = instruction_to_addr(instruction)? as usize;
        self.register[7] = self.pc as i16 + 2;
        self.pc = addr - 2;
        Ok(())
    }

    // found out how, the pc advances by 2 after BR :skull:
    fn br(&mut self, instruction: &Instruction) -> Result<(), String> {
        let addr = instruction_to_addr(instruction)?;
        match &instruction.operand1 {
            Some(br) => match br {
                Operand::BR(flag) => {
                    if (flag.n & self.psr.n) | (flag.z & self.psr.z) | (flag.p & self.psr.p) {
                        self.pc = addr as usize - 2;
                    };
                    Ok(())
                }
                _ => Err("br came with something other than flags".to_owned()),
            },
            None => Err("BR does not have flag".to_owned()),
        }
    }

    fn clrp(&mut self) -> Result<(), String> {
        self.asg.set_seed(0, 0, 0);
        Ok(())
    }

    fn lpn(&mut self) -> Result<(), String> {
        self.register[0] = self.asg.clock_16() as i16;
        Ok(())
    }

    fn lsd(&mut self) -> Result<(), String> {
        //        let mut addr = self.register[0] as usize;
        let clock = self.register[0] as u16;
        // addr += 1;
        let first = self.register[1] as u16;
        // addr += 1;
        let second = self.register[2] as u16;
        self.asg.set_seed(clock, first, second);
        Ok(())
    }

    fn rti(&mut self) -> Result<(), String> {
        if !self.psr.supervisor {
            return Err("privilege mode exception".to_owned());
        }
        self.pc = self.memory[self.register[6] as usize] as usize - 1;
        self.ssp; // TODO
                  // somewhat uneeded because trap instructions and interrupts don't exist
        Ok(())
    }

    fn not(&mut self, instruction: Instruction) -> Result<(), String> {
        let dr = instruction_to_dr(&instruction)?;
        let sr = instruction_to_sr1(&instruction)?;
        let value = !self.register[sr];
        self.register[dr] = value;
        Ok(())
    }

    fn str(&mut self, instruction: Instruction) -> Result<(), String> {
        let dest_addr = instruction_to_dr(&instruction)?;
        self.acv_exception(dest_addr)?;
        let value = instruction_to_imm7(&instruction)?;
        self.memory[dest_addr] = value as u16;
        Ok(())
    }

    fn acv_exception(&mut self, addr: usize) -> Result<(), String> {
        if address_privileged(addr) & !self.psr.supervisor {
            return Err("ACV exception, privileged memory in non supervisor".to_owned());
        }
        Ok(())
    }

    fn ld(&mut self, instruction: Instruction) -> Result<(), String> {
        let dr = instruction_to_dr(&instruction)?;
        let num = instruction_to_imm7(&instruction)?;
        self.setcc(num);
        self.register[dr] = num;
        Ok(())
    }

    fn jump(&mut self, instruction: Instruction) -> Result<(), String> {
        let reg = instruction_to_dr(&instruction)?;
        self.pc = self.register[reg] as usize - 1;
        Ok(())
    }

    fn ret(&mut self) -> Result<(), String> {
        self.pc = self.register[7] as usize - 1;
        Ok(())
    }

    fn jsrr(&mut self, instruction: Instruction) -> Result<(), String> {
        self.register[7] = self.pc as i16;
        let reg = instruction_to_dr(&instruction)?;
        self.pc = self.register[reg] as usize - 1;
        Ok(())
    }

    fn execute_double_def(&mut self, instruction: Instruction) -> Result<(), String> {
        let dr: usize = instruction_to_dr(&instruction)?;
        let sr1: usize = instruction_to_sr1(&instruction)?;
        let sr2 = match &instruction.operand2 {
            Some(op) => match op {
                Operand::Imm16(_) => Ok(instruction_to_imm16(&instruction)?),
                Operand::Address(_) => Ok(instruction_to_addr(&instruction)?),
                _ => Err("unexpected second operand in double instruction".to_owned()),
            },
            None => Err("no second operand double instruction".to_owned()),
        }?;

        match instruction.operation {
            Operation::ADDi16 => {
                let value = self.register[sr1] + sr2;
                self.setcc(value);
                self.register[dr] = value;
                Ok(())
            }
            Operation::ADDa => {
                let value = self.register[sr1] + self.memory[sr2 as usize] as i16;
                self.setcc(value);
                self.register[dr] = value;
                Ok(())
            }
            Operation::ANDi16 => {
                let value = self.register[sr1] & sr2;
                self.setcc(value);
                self.register[dr] = value;
                Ok(())
            }
            Operation::ANDa => {
                let value = self.register[sr1] & self.memory[sr2 as usize] as i16;
                self.setcc(value);
                self.register[dr] = value;
                Ok(())
            }
            Operation::XORi16 => {
                let value = self.register[sr1] ^ sr2;
                self.setcc(value);
                self.register[dr] = value;
                Ok(())
            }
            Operation::XORa => {
                let value = self.register[sr1] ^ self.memory[sr2 as usize] as i16;
                self.setcc(value);
                self.register[dr] = value;
                Ok(())
            }
            _ => Err("".to_owned()),
        }
    }

    fn execute_def(&mut self, instruction: Instruction) -> Result<(), String> {
        // this is for intructions in the 'default' configuration (dr, sr1, sr2|imm3)
        let dr: usize = instruction_to_dr(&instruction)?;
        let sr1: usize = instruction_to_sr1(&instruction)?;
        let sr2 = match &instruction.operand2 {
            Some(op) => match op {
                Operand::Imm3(_) => Ok(instruction_to_imm3(&instruction)?),
                Operand::Register(_) => Ok(instruction_to_sr2(&instruction)? as i16),
                _ => Err("unexpected second operand".to_owned()),
            },
            None => Err("no second operand".to_owned()),
        }?;
        // this may work, needs testing to make sure it does with negative numbers, probably doesn't work
        // never mind, this should work fine now that registers are i16
        match instruction.operation {
            Operation::ADD => {
                let value = self.register[sr1] + self.register[sr2 as usize];
                self.setcc(value);
                self.register[dr] = value;
                Ok(())
            }
            Operation::ADDi => {
                let value = self.register[sr1] + sr2;
                self.setcc(value);
                self.register[dr] = value;
                Ok(())
            }
            Operation::AND => {
                let value = self.register[sr1] & self.register[sr2 as usize];
                self.setcc(value);
                self.register[dr] = value;
                Ok(())
            }
            Operation::ANDi => {
                let value = self.register[sr1] & sr2;
                self.setcc(value);
                self.register[dr] = value;
                Ok(())
            }
            Operation::XOR => {
                let value = self.register[sr1] ^ self.register[sr2 as usize];
                self.setcc(value);
                self.register[dr] = value;
                Ok(())
            }
            Operation::XORi => {
                let value = self.register[sr1] ^ sr2;
                self.setcc(value);
                self.register[dr] = value;
                Ok(())
            }
            _ => Err("".to_owned()),
        }
    }

    // runs the machine until it reaches a halt instruction or exception
    pub fn run_machine(&mut self) -> Result<(), String> {
        //let mut key;
        while self.halt_flag & (self.pc < 0xFE00)
        // for if multi-threaded input handling is implemented
        // & (self.input_buffer.last() != Some(&('\x1B' as u8)))
        {
            if self.memory[self.pc] == 0 {
                self.halt_flag = false;
            } else {
                let mut out = String::new();
                if check_instruction_double(self.memory[self.pc]) {
                    out += &format!(
                        "{:016b}\n{:016b}\nexectuing: {:?}\n",
                        self.memory[self.pc],
                        self.memory[self.pc + 1],
                        tokenize(self.memory[self.pc], Some(self.memory[self.pc + 1]))?
                    );
                    self.simulate_instruction()?;
                    out += &self.pretty_print();
                    self.pc += 2;
                } else {
                    out += &format!(
                        "{:016b}\nexecuting: {:?}\n",
                        self.memory[self.pc],
                        tokenize(self.memory[self.pc], None)?
                    );
                    self.simulate_instruction()?;
                    out += &self.pretty_print();
                    self.pc += 1;
                }
                log(&out);
            }
            // this can work, but it needs to be in a seperate thread, later, for now let's not give an exit.
            // key = match self.term.input.next() {
            //     Some(key) => key.unwrap(),
            //     None => '\0' as u8,
            // };
            // if key != '\0' as u8 {
            //     self.input_buffer.push(key)
            // }
        }
        Ok(())
    }

    // fn cleanup(&mut self) {
    //     //self.term.output.into_main_screen();
    // }

    // pretty print all info
    fn pretty_print(&self) -> String {
        let mut out = String::new();
        out += &format!("PC: {} ", self.pc);
        //println!("PC: 0x{:04x}", self.pc);
        out += &self.print_registers();
        out += "\n";
        out += &self.print_pretty_memory();
        out += "\n";
        out
    }

    // print memory around PC
    fn print_pretty_memory(&self) -> String {
        let mut out: String = String::new();
        for i in (self.pc)..(self.pc + 2) {
            out += &format!("0x{:04x}: {:016b}\n", i, self.memory[i]);
        }
        out
    }

    // print registers in a pretty way
    pub fn print_registers(&self) -> String {
        let mut out: String = String::new();
        for (i, reg) in self.register.iter().enumerate() {
            out += &format!("R{}: {}\t ", i, reg);
        }
        out
    }

    // print all the modified parts of memory in a pretty way
    //fn print_modified_memory(&self) {
    //    todo!()
    //}
}

fn register_to_index(register: &Operand) -> Result<usize, String> {
    match register {
        Operand::Register(reg) => Ok(u32::from(*reg) as usize),
        _ => Err("dr is not a register".to_owned()),
    }
}

fn instruction_to_dr(instruction: &Instruction) -> Result<usize, String> {
    match &instruction.dr {
        Some(operand) => register_to_index(operand),
        None => Err("no destination register specified".to_owned()),
    }
}

fn instruction_to_addr(instruction: &Instruction) -> Result<i16, String> {
    match &instruction.operand2 {
        Some(addr) => match addr {
            Operand::Address(addr) => Ok(*addr as i16),
            Operand::Imm16(num) => Ok(*num as i16),
            _ => Err("incorrect operand2, not address".to_owned()),
        },
        None => Err("no address specified".to_owned()),
    }
}

fn instruction_to_sr1(instruction: &Instruction) -> Result<usize, String> {
    match &instruction.operand1 {
        Some(operand) => register_to_index(operand),
        None => Err("no destination register specified".to_owned()),
    }
}

fn instruction_to_sr2(instruction: &Instruction) -> Result<usize, String> {
    match &instruction.operand2 {
        Some(operand) => register_to_index(operand),
        None => Err("no destination register specified".to_owned()),
    }
}

fn instruction_to_imm3(instruction: &Instruction) -> Result<i16, String> {
    match &instruction.operand2 {
        Some(number) => match number {
            Operand::Imm3(number) => Ok(i16::from(*number)),
            _ => Err("incorrect operand 2, not imm3".to_owned()),
        },
        None => Err("no imm3".to_owned()),
    }
}

fn instruction_to_imm7(instruction: &Instruction) -> Result<i16, String> {
    match &instruction.operand1 {
        Some(number) => match number {
            Operand::Imm7(number) => Ok(i16::from(*number)),
            _ => Err("incorrect operand 1, not imm7".to_owned()),
        },
        None => Err("no imm7".to_owned()),
    }
}

fn instruction_to_imm16(instruction: &Instruction) -> Result<i16, String> {
    match &instruction.operand2 {
        Some(number) => match number {
            Operand::Imm16(number) => Ok(*number),
            _ => Err("incorrect operand 2, not imm16".to_owned()),
        },
        None => Err("no imm16".to_owned()),
    }
}

fn address_privileged(addr: usize) -> bool {
    !(0x3000..=0xFDFF).contains(&addr)
}
