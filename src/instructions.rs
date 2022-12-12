use std::io::{stdin, Read};

use crate::um::Um;

/// This function performs the condition move to set the value at register b to register a's value
/// 
/// # Arguments:
/// 'um': current version of the machine (our Um struct)
/// 'ra': the index of register a stored as a u32
/// 'rb': the index of register b stored as a u32
/// 'rc': the index of register c stored as a u32
pub fn cmov(um: &mut Um, ra: u32, rb: u32, rc: u32){
    if um.reg[rc as usize] != 0 {
        um.reg[ra as usize] = um.reg[rb as usize];
    }
}

/// This function loads a value stored in a memeory segment
/// 
/// # Arguments:
/// 'um': current version of the machine (our Um struct)
/// 'ra': the index of register a stored as a u32
/// 'rb': the index of register b stored as a u32
/// 'rc': the index of register c stored as a u32
pub fn load(um: &mut Um, ra: u32, rb: u32, rc: u32){
    if um.mem[um.reg[rb as usize]as usize].is_empty(){
        panic!("Memory not mapped")
    }
    um.reg[ra as usize] = um.mem[um.reg[rb as usize]as usize][um.reg[rc as usize] as usize];
}

/// This function stores the value of register c to memory
/// 
/// # Arguments:
/// 'um': current version of the machine (our Um struct)
/// 'ra': the index of register a stored as a u32
/// 'rb': the index of register b stored as a u32
/// 'rc': the index of register c stored as a u32
pub fn store(um: &mut Um, ra: u32, rb: u32, rc: u32){
    if um.mem[um.reg[ra as usize]as usize].is_empty(){
        panic!("Memory not mapped")
    }
    um.mem[um.reg[ra as usize]as usize][um.reg[rb as usize] as usize] = um.reg[rc as usize];
}

/// This function sets register a to the sum of registers b and c
/// 
/// # Arguments:
/// 'um': current version of the machine (our Um struct)
/// 'ra': the index of register a stored as a u32
/// 'rb': the index of register b stored as a u32
/// 'rc': the index of register c stored as a u32
pub fn add(um: &mut Um, ra: u32, rb: u32, rc: u32){
    um.reg[ra as usize] = ((um.reg[rb as usize] as u64 + um.reg[rc as usize] as u64) % (2 as u64).pow(32)) as u32;
}

/// This function sets register a to the product of registers b and c
/// 
/// # Arguments:
/// 'um': current version of the machine (our Um struct)
/// 'ra': the index of register a stored as a u32
/// 'rb': the index of register b stored as a u32
/// 'rc': the index of register c stored as a u32
pub fn mul(um: &mut Um, ra: u32, rb: u32, rc: u32){
    um.reg[ra as usize] = ((um.reg[rb as usize] as u64 * um.reg[rc as usize] as u64 ) % (2 as u64).pow(32)) as u32;
}

/// This function sets register a to the quotient of registers b and c
/// 
/// # Arguments:
/// 'um': current version of the machine (our Um struct)
/// 'ra': the index of register a stored as a u32
/// 'rb': the index of register b stored as a u32
/// 'rc': the index of register c stored as a u32
pub fn div(um: &mut Um, ra: u32, rb: u32, rc: u32){
    um.reg[ra as usize] = um.reg[rb as usize] / um.reg[rc as usize] ;
}

/// This function sets register a to the nand of registers b and c
/// 
/// # Arguments:
/// 'um': current version of the machine (our Um struct)
/// 'ra': the index of register a stored as a u32
/// 'rb': the index of register b stored as a u32
/// 'rc': the index of register c stored as a u32
pub fn nand(um: &mut Um, ra: u32, rb: u32, rc: u32){
    um.reg[ra as usize] = !(um.reg[rb as usize] & um.reg[rc as usize] );
}

/// This function ends the program
/// 
pub fn halt(){
    std::process::exit(0)
}

/// This function maps a memory segment
/// 
/// # Arguments:
/// 'um': current version of the machine (our Um struct)
/// 'rb': the index of register b stored as a u32
/// 'rc': the index of register c stored as a u32
pub fn mapseg(um: &mut Um, rb: u32, rc: u32){
    if um.empty_keys.is_empty(){
        um.mem.push(vec![0; um.reg[rc as usize] as usize]);
        um.reg[rb as usize] = (um.mem.len() - 1) as u32;
    } else{
        let idx = um.empty_keys.pop().unwrap();
        um.mem[idx as usize] = vec![0; um.reg[rc as usize] as usize];
        um.reg[rb as usize] = idx;
    }
}

/// This function unmaps a memory segment
/// 
/// # Arguments:
/// 'um': current version of the machine (our Um struct)
/// 'rc': the index of register c stored as a u32
pub fn umapseg(um: &mut Um, rc: u32) {
    um.empty_keys.push(um.reg[rc as usize]);
    um.mem[um.reg[rc as usize] as usize].clear()
}

/// This function takes in a charcter as u32 and assigns it to register c
/// 
/// # Arguments:
/// 'um': current version of the machine (our Um struct)
/// 'rc': the index of register c stored as a u32
pub fn input(um: &mut Um, rc: u32){
    let mut buffer: [u8; 1] = [0];

    let c = stdin().read(&mut buffer);

    um.reg[rc as usize]  = match c {
        Ok(0) => u32::MAX,
        Ok(1) => buffer[0] as u32,
        _ => panic!("Invalid input")
    };
    
}

/// This function prints the ascii value of register c
/// 
/// # Arguments:
/// 'um': current version of the machine (our Um struct)
/// 'rc': the index of register c stored as a u32
pub fn out(um: &mut Um, rc: u32){
    if um.reg[rc as usize]  > 255 {
        panic!("Incorrect value");
    }
    print!("{}", char::from_u32(um.reg[rc as usize]).unwrap());
}

/// This function loads a program
/// 
/// # Arguments:
/// 'um': current version of the machine (our Um struct)
/// 'rb': the index of register b stored as a u32
/// 'rc': the index of register c stored as a u32
pub fn lp(um: &mut Um, rb: u32, rc: u32){
    um.mem[0] = um.mem[um.reg[rb as usize] as usize].clone();
    um.program_counter = um.reg[rc as usize] as usize;
}

/// This function set register a to value of 25 bits
/// 
/// # Arguments:
/// 'um': current version of the machine (our Um struct)
/// 'ra': the index of register a stored as a u32
/// 'value': the value of the current instructions first 25 bits
pub fn lv(um: &mut Um, ra: u32, value: u32){
    um.reg[ra as usize] = value;
}