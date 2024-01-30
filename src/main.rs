#[derive(PartialEq)]
struct LFSR {
    state: u16,
}

impl LFSR {
    // create new LFSR
    fn new() -> LFSR {
        LFSR { state: (0) }
    }
    // set seed of LFSR
    fn set_seed(&mut self, seed: u16) {
        self.state = seed;
    }
    //clock LFSR
    fn clock(&mut self) -> u16 {
        let bit: u16 =
            (self.state ^ (self.state >> 1) ^ (self.state >> 3) ^ (self.state >> 12)) & 0b1;
        self.state = (self.state >> 1) | (bit << 15);
        return bit;
    }
}

#[derive(PartialEq)]
struct ASG {
    clock: LFSR,
    first: LFSR,
    second: LFSR,
}

impl ASG {
    fn new() -> ASG {
        ASG {
            clock: (LFSR { state: (0) }),
            first: (LFSR { state: (0) }),
            second: (LFSR { state: (0) }),
        }
    }
    fn set_seed(&mut self, clock: u16, first: u16, second: u16) {
        self.clock.set_seed(clock);
        self.first.set_seed(first);
        self.second.set_seed(second);
    }
    fn clock(&mut self) -> u16 {
        if self.clock.clock() == 1 {
            return self.first.clock();
        } else {
            return self.second.clock();
        }
    }
}

fn main() {
    println!("Hello, world!");
    let state: u16 = 0xACE1;
    let mut just_lfsr: LFSR = LFSR::new();
    let mut lfsr_orig: LFSR = LFSR::new();
    just_lfsr.set_seed(state);
    lfsr_orig.set_seed(state);
    just_lfsr.clock();
    // 128 bits because the ASG overflows lower values
    let mut period: u128 = 1;
    while just_lfsr != lfsr_orig {
        just_lfsr.clock();
        period += 1;
    }
    println!("LSFR period: {}", period);

    let first: u16 = 0xDA0C;
    let second: u16 = 0xFD0B;
    let mut asg: ASG = ASG::new();
    let mut asg_orig: ASG = ASG::new();
    asg.set_seed(state, first, second);
    asg_orig.set_seed(state, first, second);
    period = 1;
    asg.clock();
    while asg != asg_orig {
        asg.clock();
        period += 1;
    }
    println!("ASG period: {}", period)
}
