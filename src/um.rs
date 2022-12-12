use core::panic;
use crate::umload::load_file;
use crate::instructions::{cmov, load, store, add, mul, div, nand, halt, mapseg, umapseg, out, input, lp, lv};

type Umi = u32;

enum Opcode {
    CMov,
    Load,
    Store,
    Add,
    Mul,
    Div,
    NAND,
    HALT,
    MapSeg,
    UMapSeg,
    Out,
    In,
    LP,
    LV
}

pub struct Field {
    width: u32,
    lsb: u32,
}

static RA: Field = Field { width: 3, lsb: 6 };
static RB: Field = Field { width: 3, lsb: 3 };
static RC: Field = Field { width: 3, lsb: 0 };
static RL: Field = Field { width: 3, lsb: 25 };
static VL: Field = Field { width: 25, lsb: 0 };
static OP: Field = Field { width: 4, lsb: 28 };

fn mask(bits: u32) -> u32 {
    (1 << bits) - 1
}

pub fn get(field: &Field, instruction: Umi) -> u32 {
    (instruction >> field.lsb) & mask(field.width)
}

/// Structure of our Universal Machine
/// 
/// # Fields:
/// 'reg': 8 registers to hold u32
/// 'mem': 2D vector to map memory
/// 'program_counter': u32 counter to keep track of current instruction
/// 'empty_keys': vector of u32 to store empty memory segments
pub struct Um {
    pub reg: [u32; 8],
    pub mem: Vec<Vec<u32>>,
    pub program_counter: usize,
    pub empty_keys: Vec<u32>
}

impl Um {

    /// This funtion initializes our machine
    /// 
    /// # Arguments:
    /// 'input': name of a file
    pub fn initialize_program(&mut self, input: Option<String>) {
        self.reg = [0; 8];
        self.mem = Vec::new();
        self.program_counter = 0;
        self.empty_keys= Vec::new();

        self.mem.insert(0, load_file(input.as_deref()));
    }

    /// This function runs our program
    /// 
    pub fn run_program(&mut self) {
        loop {
            //println!("{}", self.program_counter);
            let inst = self.mem[0][self.program_counter as usize];
            let ra = get(&RA, inst);
            let rb = get(&RB, inst);
            let rc = get(&RC, inst);
            let rl = get(&RL, inst);
            let vl = get(&VL, inst);
            match get(&OP, inst) {
                o => {
                    //println!("{:032b}", inst);
                    if o == Opcode::CMov as u32 {
                        cmov(self, ra, rb, rc);
                        self.program_counter += 1;
                    } else if o == Opcode::Load as u32 {
                        load(self, ra, rb, rc);
                        self.program_counter += 1;
                    } else if o == Opcode::Store as u32 {
                        store(self, ra, rb, rc);
                        self.program_counter += 1;
                    } else if o == Opcode::Add as u32 {
                        add(self, ra, rb, rc);
                        self.program_counter += 1;
                    } else if o == Opcode::Mul as u32 {
                       mul(self, ra, rb, rc);
                       self.program_counter += 1;
                    } else if o == Opcode::Div as u32 {
                        div(self, ra, rb, rc);
                        self.program_counter += 1;
                    } else if o == Opcode::NAND as u32 {
                        nand(self, ra, rb, rc);
                        self.program_counter += 1;
                    } else if o == Opcode::HALT as u32 {
                        halt();
                    } else if o == Opcode::MapSeg as u32 {
                        mapseg(self, rb, rc);
                        self.program_counter += 1;
                    } else if o == Opcode::UMapSeg as u32 {
                        umapseg(self, rc);
                        self.program_counter += 1;
                    } else if o == Opcode::Out as u32 {
                        out(self, rc);
                        self.program_counter += 1;
                    } else if o == Opcode::In as u32 {
                        input(self, rc);
                        self.program_counter += 1;
                    } else if o == Opcode::LP as u32 {
                        lp(self, rb, rc);
                    } else if o == Opcode::LV as u32 {
                        lv(self, rl, vl);
                        self.program_counter += 1;
                    } else {
                        panic!("Invalid instruction");
                    }
                }
            }
        }       
    }
}