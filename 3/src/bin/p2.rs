use aoc_3::{combine, get_input};

fn main() -> Result<(), std::io::Error> {
    let input = get_input()?;

    let bit_length = input[0].len();

    let mut vals = Vec::<u64>::new();
    for keep_least_common in 0..=1 {
        let mut lines = input.iter()
            .map(|l: &String| {
                l.chars().map(|c: char| {
                    match c {
                        '0' => 0u8,
                        '1' => 1u8,
                        _ => panic!("expected 0 or 1")
                    }
                }).collect::<Vec<u8>>()
            }).collect::<Vec<Vec<u8>>>();
        for i in 0..bit_length {
            let mut x = 0i64;
            for line in lines.iter() {
                // sub 1 if 0, add 1 if 1
                match line[i] {
                    0 => x -= 1,
                    1 => x += 1,
                    _ => panic!("expected 0 or 1")
                }
            }
            let bit: u8;
            if x == 0 {
                bit = 1 - keep_least_common;
            } else {
                if keep_least_common == 0 {
                    bit = ((x + x.abs()) / (2 * x)) as u8;
                } else {
                    // flip the most common bit
                    bit = 1 - ((x + x.abs()) / (2 * x)) as u8;
                }
            }
            lines = lines.iter()
                .filter(|l| l[i] == bit)
                .cloned()
                .collect();

            if lines.len() == 1 {
                vals.push(combine(&lines[0]));
                break;
            }
        }
    }

    println!("{}", vals[0] * vals[1]);

    Ok(())
}
