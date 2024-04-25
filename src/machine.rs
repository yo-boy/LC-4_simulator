use crate::prng::ASG;
use crate::tokenizer::{check_instruction_double, tokenize, Instruction, Operand, Operation};
use ux::u3;

struct PSR {
    priority: u3,
    supervisor: bool,
    n: bool,
    p: bool,
    z: bool,
}

pub struct Machine {
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

impl Machine {
    pub fn new(mem: Option<[u16; 65536]>) -> Machine {
        Machine {
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
                    Operation::ADDi16 => Ok(()),
                    Operation::ADDa => Ok(()),
                    Operation::ANDi16 => Ok(()),
                    Operation::ANDa => Ok(()),
                    Operation::XORi16 => Ok(()),
                    Operation::XORa => Ok(()),
                    Operation::BR => Ok(()),
                    Operation::JSR => Ok(()),
                    Operation::LDa => Ok(()),
                    Operation::ST => Ok(()),
                    Operation::STR16 => Ok(()),
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
                    Operation::RTI => Ok(()),
                    Operation::LSD => Ok(()),
                    Operation::LPN => Ok(()),
                    Operation::CLRP => Ok(()),
                    Operation::HALT => Ok(self.halt_flag = false),
                    Operation::PUTS => Ok(()),
                    Operation::GETC => Ok(()),
                    Operation::OUT => Ok(()),
                    Operation::IN => Ok(()),
                    Operation::PUTSP => Ok(()),
                    _ => Err("double instruction in simulate_instruction double".to_string()),
                },
                Err(error) => Err(error),
            }
        }
    }

    fn rti(&mut self) -> Result<(), String> {
        if !self.psr.supervisor {
            return Err("privilege mode exception".to_owned());
        }
        self.ssp; // TODO
        Ok(())
    }

    fn not(&mut self, instruction: Instruction) -> Result<(), String> {
        let dr = instruction_to_dr(&instruction)?;
        let sr = instruction_to_sr1(&instruction)?;
        let value = !self.register[sr];
        Ok(self.register[dr] = value)
    }

    fn str(&mut self, instruction: Instruction) -> Result<(), String> {
        let dest_addr = instruction_to_dr(&instruction)?;
        let value = instruction_to_imm7(&instruction)?;
        self.memory[dest_addr] = value as u16;
        Ok(())
    }

    fn ld(&mut self, instruction: Instruction) -> Result<(), String> {
        let dr = instruction_to_dr(&instruction)?;
        let num = instruction_to_imm7(&instruction)?;
        self.setcc(num);
        Ok(self.register[dr] = num)
    }

    fn jump(&mut self, instruction: Instruction) -> Result<(), String> {
        let reg = instruction_to_dr(&instruction)?;
        self.pc = self.register[reg] as usize;
        Ok(())
    }

    fn ret(&mut self) -> Result<(), String> {
        self.pc = self.register[7] as usize;
        Ok(())
    }

    fn jsrr(&mut self, instruction: Instruction) -> Result<(), String> {
        self.register[7] = self.pc as i16;
        let reg = instruction_to_dr(&instruction)?;
        self.pc = self.register[reg] as usize;
        Ok(())
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
        return match instruction.operation {
            Operation::ADD => {
                let value = self.register[sr1] + self.register[sr2 as usize];
                self.setcc(value);
                Ok(self.register[dr] = value)
            }
            Operation::ADDi => {
                let value = self.register[sr1] + sr2;
                self.setcc(value);
                Ok(self.register[dr] = value)
            }
            Operation::AND => {
                let value = self.register[sr1] & self.register[sr2 as usize];
                self.setcc(value);
                Ok(self.register[dr] = value)
            }
            Operation::ANDi => {
                let value = self.register[sr1] & sr2;
                self.setcc(value);
                Ok(self.register[dr] = value)
            }
            Operation::XOR => {
                let value = self.register[sr1] ^ self.register[sr2 as usize];
                self.setcc(value);
                Ok(self.register[dr] = value)
            }
            Operation::XORi => {
                let value = self.register[sr1] ^ sr2;
                self.setcc(value);
                Ok(self.register[dr] = value)
            }
            _ => Err("".to_owned()),
        };
    }

    // runs the machine until it reaches a halt instruction or exception
    pub fn run_machine(&mut self) -> Result<(), &str> {
        while self.halt_flag & (self.pc < 0xFE00) {
            if check_instruction_double(self.memory[self.pc]) {
                println!(
                    "exectuing: {:?}",
                    tokenize(self.memory[self.pc], Some(self.memory[self.pc + 1])).unwrap()
                );
                self.simulate_instruction().unwrap();
                self.pretty_print();
                self.pc += 2;
            } else {
                println!(
                    "executing: {:?}",
                    tokenize(self.memory[self.pc], None).unwrap()
                );
                self.simulate_instruction().unwrap();
                self.pretty_print();
                self.pc += 1;
            }
        }
        Ok(())
    }

    // pretty print all info
    fn pretty_print(&self) {
        println!("PC: 0x{:04x}", self.pc);
        self.print_registers();
        println!();
        self.print_pretty_memory();
        println!();
    }

    // print memory around PC
    fn print_pretty_memory(&self) {
        for i in (self.pc)..(self.pc + 2) {
            println!("0x{:04x}: {:016b}", i, self.memory[i]);
        }
    }

    // print registers in a pretty way
    pub fn print_registers(&self) {
        for (i, reg) in self.register.iter().enumerate() {
            print!("R{}: {}\t", i, reg)
        }
    }

    // print all the modified parts of memory in a pretty way
    fn print_modified_memory(&self) {
        todo!()
    }
}

fn register_to_index(register: &Operand) -> Result<usize, String> {
    match register {
        Operand::Register(reg) => Ok(u32::from(reg.clone()) as usize),
        _ => Err("dr is not a register".to_owned()),
    }
}

fn instruction_to_dr(instruction: &Instruction) -> Result<usize, String> {
    match &instruction.dr {
        Some(operand) => register_to_index(&operand),
        None => Err("no destination register specified".to_owned()),
    }
}

fn instruction_to_sr1(instruction: &Instruction) -> Result<usize, String> {
    match &instruction.operand1 {
        Some(operand) => register_to_index(&operand),
        None => Err("no destination register specified".to_owned()),
    }
}

fn instruction_to_sr2(instruction: &Instruction) -> Result<usize, String> {
    match &instruction.operand2 {
        Some(operand) => register_to_index(&operand),
        None => Err("no destination register specified".to_owned()),
    }
}

fn instruction_to_imm3(instruction: &Instruction) -> Result<i16, String> {
    match &instruction.operand2 {
        Some(number) => match number {
            Operand::Imm3(number) => Ok(i16::from(number.clone())),
            _ => Err("incorrect operand 2, not imm3".to_owned()),
        },
        None => Err("no imm3".to_owned()),
    }
}

fn instruction_to_imm7(instruction: &Instruction) -> Result<i16, String> {
    match &instruction.operand1 {
        Some(number) => match number {
            Operand::Imm7(number) => Ok(i16::from(number.clone())),
            _ => Err("incorrect operand 1, not imm7".to_owned()),
        },
        None => Err("no imm7".to_owned()),
    }
}
