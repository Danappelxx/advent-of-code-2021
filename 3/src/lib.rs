use std::io::{BufRead, stdin};

pub fn get_input() -> Result<Vec<String>, std::io::Error> {
    stdin().lock()
        .lines()
        .collect::<Result<Vec<String>, _>>()
}

pub fn combine(bits: &Vec<u8>) -> u64 {
    let mut res = 0;
    for (i, b) in bits.iter().rev().enumerate() {
        res += 2_usize.pow(i as u32) * *b as usize
    }
    return res as u64;
}
