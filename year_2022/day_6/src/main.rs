// --- Day 6: Tuning Trouble ---
// https://adventofcode.com/2022/day/6

use std::{
    fs,
    io::{Read, Result},
};

fn main() -> Result<()> {
    let mut input_file: fs::File = fs::File::open("./src/input")?;
    let mut data = String::new();
    input_file.read_to_string(&mut data)?;

    println!("Part 1 : {}", puzzle_logic(4, data.clone()));
    println!("Part 2 : {}", puzzle_logic(14, data));

    Ok(())
}

fn puzzle_logic(win_size: usize, data: String) -> i32 {
    let mut num_p2: i32 = 0;
    for window in data.chars().collect::<Vec<char>>().windows(win_size) {
        num_p2 += 1;
        let mut repeat: bool = false;
        let mut x = 1;
        for ch in window {
            for y in x..win_size {
                if ch.eq(&window[y]) {
                    repeat = true;
                }
            }
            x += 1;
        }
        if !repeat {
            num_p2 = num_p2 + win_size as i32 - 1;
            break;
        }
    }
    num_p2
}
