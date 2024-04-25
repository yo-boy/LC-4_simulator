#[derive(PartialEq)]
pub struct LFSR {
    // holds the state of the 16 bit lfsr
    state: u16,
}

impl LFSR {
    // create new LFSR
    pub fn new() -> LFSR {
        LFSR { state: (0) }
    }
    // set seed of LFSR
    pub fn set_seed(&mut self, seed: u16) {
        self.state = seed;
    }
    //clock LFSR and return a u16 containing only 1 bit at position 0 that corresponds to the output bit
    pub fn clock(&mut self) -> u16 {
        let out = self.state & 0b1;
        // https://en.wikipedia.org/wiki/Linear-feedback_shift_register#Example_polynomials_for_maximal_LFSRs
        // this is a maximal LFSR that results in the largest possible period
        let bit: u16 =
            (self.state ^ (self.state >> 1) ^ (self.state >> 3) ^ (self.state >> 12)) & 0b1;
        self.state = (self.state >> 1) | (bit << 15);
        out
    }
}

#[derive(PartialEq)]
pub struct ASG {
    // three LFSRs that represent a physical ASG
    clock: LFSR,
    first: LFSR,
    second: LFSR,
}

impl ASG {
    // create a new ASG
    pub fn new() -> ASG {
        ASG {
            clock: LFSR::new(),
            first: LFSR::new(),
            second: LFSR::new(),
        }
    }
    // set the seed for all three LFSRs
    pub fn set_seed(&mut self, clock: u16, first: u16, second: u16) {
        self.clock.set_seed(clock);
        self.first.set_seed(first);
        self.second.set_seed(second);
    }
    // clock the ASG returning the output bit at position 0 in a u16
    pub fn clock(&mut self) -> u16 {
        if self.clock.clock() == 1 {
            self.first.clock()
        } else {
            self.second.clock()
        }
    }
    // clock the ASG 16 times and return the 16 output bits in a u16
    pub fn clock_16(&mut self) -> u16 {
        let mut out = 0u16;
        for _ in 0..15 {
            out |= self.clock();
            out <<= 1;
        }
        out
    }
}
