struct LfsrOutput {
    output_bit: u16,
    state: u16,
}


fn main() {
    println!("Hello, world!");
    let state: u16 = 0xACE1;
    let mut flag: bool = true;
    let mut state_new: u16 = lfsr_clock(state).state;
    let mut period = 1;
    while flag {
        state_new = lfsr_clock(state_new).state;
        flag = state != state_new;
        period = period + 1;
    }
    println!("period: {}", period);
}

// our lfsr shifts right with every clock
fn lfsr_clock(state: u16) -> LfsrOutput {
    // xor bits 16 15 13 4 0 which gives us a maximal lfsr
    // https://en.wikipedia.org/wiki/Linear-feedback_shift_register#Example_polynomials_for_maximal_LFSRs
    let bit: u16 = (state ^ (state >> 1) ^ (state >> 3) ^ (state >> 12)) & 0b1;

    let lfsr_state: u16 = (state >> 1) | (bit << 15);
    LfsrOutput {
        output_bit: (bit),
        state: (lfsr_state),
    }
}
