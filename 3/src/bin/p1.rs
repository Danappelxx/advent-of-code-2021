use std::collections::HashMap;
use aoc_3::get_input;

fn main() -> Result<(), std::io::Error> {
    let lines = get_input()?;

    let mut bits = HashMap::<usize, i64>::new();
    for line in lines {
        for (i, char) in line.chars().enumerate() {
            match char {
                '0' => *bits.entry(i).or_insert(0) -= 1,
                '1' => *bits.entry(i).or_insert(0) += 1,
                _ => panic!("bad input, expected 0 or 1")
            };
        }
    }
    println!("bits: {:#?}", bits);

    let count = bits.keys().max().unwrap();

    let gamma_bits: Vec<u8> = (0..=*count)
        .map(|i| {
            let x = bits[&i];
            assert_ne!(x, 0);
            // 0 if negative, 1 if positive
            let bit = ((x + x.abs()) / (2 * x)) as u8;
            return bit
        })
        .collect();

    let epsilon_bits: Vec<u8> = gamma_bits.iter()
        .map(|b| 1 - b)
        .collect();

    println!("gamma bits: {:?}, res: {}", gamma_bits, combine(&gamma_bits));
    println!("epsilon bits: {:?}, res: {}", epsilon_bits, combine(&epsilon_bits));
    println!("{}", combine(&gamma_bits) * combine(&epsilon_bits));

    Ok(())
}
