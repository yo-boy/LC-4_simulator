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
impl Operation {
    fn is_double(&self) -> bool {
        match self {
            Operation::ADDi16 => true,
            Operation::ADDa => true,
            Operation::ANDi16 => true,
            Operation::ANDa => true,
            Operation::XORi16 => true,
            Operation::XORa => true,
            Operation::BR => true,
            Operation::JSR => true,
            Operation::LDa => true,
            Operation::ST => true,
            Operation::STR16 => true,
            _ => false,
        }
    }
}
pub enum Operand {
    BR(Flags),
    Address(u16),
    Imm16(i16),
    Imm7(ux::i7),
    Imm3(ux::i3),
    TrapVect(u8),
    Register(ux::u3),
}
pub struct Instruction {
    operation: Operation,
    dr: Option<Operand>,
    operand1: Option<Operand>,
    operand2: Option<Operand>,
}
pub struct Flags {
    n: bool,
    z: bool,
    p: bool,
}

pub fn tokenize(encoded_instruction: u16, second_operand: Option<u16>) -> Instruction {
    let operation = match_opcode(encoded_instruction);
    if operation.is_double() {
        parse_double(
            operation,
            encoded_instruction,
            second_operand.expect("32 bit operand parse error"),
        )
    } else {
        parse_single(operation, encoded_instruction)
    }
}

// extract operation from instruction

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
            if check_instruction_double(instruction) {
                Operation::JSR
            } else {
                Operation::JSRR
            }
        }
        0b01000 => {
            if check_instruction_double(instruction) {
                Operation::LDa
            } else {
                Operation::LD
            }
        }
        0b01001 => Operation::ST,
        0b00111 => {
            if check_instruction_double(instruction) {
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

fn check_instruction_double(instruction: u16) -> bool {
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

// match parsed operations and create instructions

// parse single length insntructions
fn parse_single(operation: Operation, instruction: u16) -> Instruction {
    match operation {
        Operation::ADD => parse_def(operation, instruction),
        Operation::ADDi => parse_i(operation, instruction),
        Operation::AND => parse_def(operation, instruction),
        Operation::ANDi => parse_i(operation, instruction),
        Operation::XOR => parse_def(operation, instruction),
        Operation::XORi => parse_i(operation, instruction),
        Operation::JUMP => parse_single_reg(operation, instruction),
        Operation::RET => operation_to_instruction(operation),
        Operation::JSRR => parse_single_reg(operation, instruction),
        Operation::LD => Instruction {
            operation,
            dr: Some(get_dr(instruction)),
            operand1: Some(get_imm7(instruction)),
            operand2: None,
        },
        Operation::STR => Instruction {
            operation,
            dr: Some(get_dr(instruction)),
            operand1: Some(get_imm7(instruction)),
            operand2: None,
        },
        Operation::NOT => Instruction {
            operation,
            dr: Some(get_dr(instruction)),
            operand1: Some(get_sr(instruction)),
            operand2: None,
        },
        Operation::TRAP => panic!("unexpected TRAP instruction not supported"),
        Operation::RTI => operation_to_instruction(operation),
        Operation::LSD => operation_to_instruction(operation),
        Operation::LPN => operation_to_instruction(operation),
        Operation::CLRP => operation_to_instruction(operation),
        Operation::HALT => operation_to_instruction(operation),
        Operation::PUTS => operation_to_instruction(operation),
        Operation::GETC => operation_to_instruction(operation),
        Operation::OUT => operation_to_instruction(operation),
        Operation::IN => operation_to_instruction(operation),
        Operation::PUTSP => operation_to_instruction(operation),
        _ => panic!("long instruction in parse_single"),
    }
}
//helpers for single length instruction parsing
fn parse_single_reg(operation: Operation, instruction: u16) -> Instruction {
    Instruction {
        operation,
        dr: Some(get_dr(instruction)),
        operand1: None,
        operand2: None,
    }
}
fn parse_def(operation: Operation, instruction: u16) -> Instruction {
    Instruction {
        operation,
        dr: Some(get_dr(instruction)),
        operand1: Some(get_sr(instruction)),
        operand2: Some(get_sr2(instruction)),
    }
}
fn parse_i(operation: Operation, instruction: u16) -> Instruction {
    Instruction {
        operation,
        dr: Some(get_dr(instruction)),
        operand1: Some(get_sr(instruction)),
        operand2: Some(get_imm3(instruction)),
    }
}
fn operation_to_instruction(operation: Operation) -> Instruction {
    Instruction {
        operation,
        dr: None,
        operand1: None,
        operand2: None,
    }
}

// parse double length intsructions
fn parse_double(operation: Operation, instruction: u16, operand: u16) -> Instruction {
    match operation {
        Operation::ADDi16 => parse_i16(operation, instruction, operand),
        Operation::ADDa => parse_a(operation, instruction, operand),
        Operation::ANDi16 => parse_i16(operation, instruction, operand),
        Operation::ANDa => parse_a(operation, instruction, operand),
        Operation::XORi16 => parse_i16(operation, instruction, operand),
        Operation::XORa => parse_a(operation, instruction, operand),
        Operation::BR => parse_br(instruction, operand),
        Operation::JSR => Instruction {
            operation,
            dr: None,
            operand1: None,
            operand2: Some(get_addr(operand)),
        },
        Operation::LDa => Instruction {
            operation,
            dr: Some(get_dr(instruction)),
            operand1: None,
            operand2: Some(get_addr(operand)),
        },
        Operation::ST => Instruction {
            operation,
            dr: Some(get_dr(instruction)),
            operand1: None,
            operand2: Some(get_addr(operand)),
        },
        Operation::STR16 => Instruction {
            operation,
            dr: Some(get_dr(instruction)),
            operand1: None,
            operand2: Some(get_imm16(operand)),
        },
        _ => panic!("short instruction in parse_double"),
    }
}
// helpers for double length instruction parsing
fn parse_br(instruction: u16, operand: u16) -> Instruction {
    let instruction = instruction >> 7 & 0b111;
    let flag = Operand::BR(Flags {
        n: instruction >> 2 == 1,
        z: instruction >> 1 & 0b1 == 1,
        p: instruction & 0b1 == 1,
    });
    Instruction {
        operation: Operation::BR,
        dr: None,
        operand1: Some(flag),
        operand2: Some(Operand::Address(operand)),
    }
}
fn parse_a(operation: Operation, instruction: u16, operand: u16) -> Instruction {
    Instruction {
        operation,
        dr: Some(get_dr(instruction)),
        operand1: Some(get_sr(instruction)),
        operand2: Some(get_addr(operand)),
    }
}
fn parse_i16(operation: Operation, instruction: u16, operand: u16) -> Instruction {
    Instruction {
        operation,
        dr: Some(get_dr(instruction)),
        operand1: Some(get_sr(instruction)),
        operand2: Some(get_imm16(operand)),
    }
}

// parse operands
fn get_addr(address: u16) -> Operand {
    Operand::Address(address)
}
fn get_dr(instruction: u16) -> Operand {
    let out: ux::u3 = ux::u3::new((instruction >> 7 & 0b111u16) as u8);
    Operand::Register(out)
}
fn get_sr(instruction: u16) -> Operand {
    let out: ux::u3 = ux::u3::new((instruction >> 4 & 0b111u16) as u8);
    Operand::Register(out)
}
fn get_sr2(instruction: u16) -> Operand {
    let out: ux::u3 = ux::u3::new((instruction & 0b111u16) as u8);
    Operand::Register(out)
}
fn get_imm3(instruction: u16) -> Operand {
    Operand::Imm3(if instruction >> 2 & 0b1 == 1 {
        let num = instruction | 0b1111_1111_1111_1100;
        ux::i3::new(num as i8)
    } else {
        let num = instruction & 0b0000_0000_0000_0011;
        ux::i3::new(num as i8)
    })
}
fn get_imm7(instruction: u16) -> Operand {
    Operand::Imm7(if instruction >> 6 & 0b1 == 1 {
        let num = instruction | 0b1111_1111_1100_0000;
        ux::i7::new(num as i8)
    } else {
        let num = instruction & 0b0000_0000_0011_1111;
        ux::i7::new(num as i8)
    })
}
fn get_imm16(operand: u16) -> Operand {
    Operand::Imm16(operand as i16)
}
