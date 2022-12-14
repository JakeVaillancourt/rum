use std::convert::TryInto;

/// This function takes in the name of a file and reads the instructions of the file
/// to a vector of u32
/// 
/// # Arguments:
/// 'input': name of a file
pub fn load_file(input: Option<&str>) -> Vec<u32> {
    let mut raw_reader: Box<dyn std::io::BufRead> = match input {
        None => Box::new(std::io::BufReader::new(std::io::stdin())),
        Some(filename) => Box::new(std::io::BufReader::new(
            std::fs::File::open(filename).unwrap(),
        )),
    };
    let mut buf = Vec::<u8>::new();
    raw_reader.read_to_end(&mut buf).unwrap();
    let instructions: Vec<u32> = buf
        .chunks_exact(4)
        .map(|x| u32::from_be_bytes(x.try_into().unwrap()))
        .collect();
    instructions
}
