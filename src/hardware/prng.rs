#[derive(PartialEq)]
pub struct LFSR {
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
    //clock LFSR
    pub fn clock(&mut self) -> u16 {
        // https://en.wikipedia.org/wiki/Linear-feedback_shift_register#Example_polynomials_for_maximal_LFSRs
        // this is a maximal LFSR that results in the largest possible period
        let bit: u16 =
            (self.state ^ (self.state >> 1) ^ (self.state >> 3) ^ (self.state >> 12)) & 0b1;
        self.state = (self.state >> 1) | (bit << 15);
        return bit;
    }
}

#[derive(PartialEq)]
pub struct ASG {
    clock: LFSR,
    first: LFSR,
    second: LFSR,
}

impl ASG {
    pub fn new() -> ASG {
        ASG {
            clock: LFSR::new(),
            first: LFSR::new(),
            second: LFSR::new(),
        }
    }
    pub fn set_seed(&mut self, clock: u16, first: u16, second: u16) {
        self.clock.set_seed(clock);
        self.first.set_seed(first);
        self.second.set_seed(second);
    }
    pub fn clock(&mut self) -> u16 {
        if self.clock.clock() == 1 {
            return self.first.clock();
        } else {
            return self.second.clock();
        }
    }
}
