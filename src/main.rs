mod prng;
use prng::{ASG, LFSR};

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
    //asg.clock();
    //while asg != asg_orig {
    //    asg.clock();
    //    period += 1;
    //}
    println!("ASG period: {}", period);
    let test: [u16; 65536] = [0u16; 2usize.pow(16)];
    println!("{}", test[0]);
}
