use std::env;
use rum::um::{Um};


fn main() {
    let input = env::args().nth(1);
    let mut rum = Um{reg: [0; 8], mem: Vec::new(), program_counter: 0, empty_keys: vec![]};
    Um::initialize_program(&mut rum, input);
    Um::run_program(&mut rum);
}