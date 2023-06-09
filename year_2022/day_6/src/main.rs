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

    let mut num_p1: i32 = 0;

    for window in data.chars().collect::<Vec<char>>().windows(4) {
        num_p1 += 1;
        let mut repeat: bool = false;
        let mut x = 1;
        for ch in window {
            for y in x..=3 {
                if ch.eq(&window[y]) {
                    repeat = true;
                }
            }
            x += 1;
        }
        if !repeat {
            num_p1 += 3;
            break;
        }
    }

    let mut num_p2: i32 = 0;

    for window in data.chars().collect::<Vec<char>>().windows(14) {
        num_p2 += 1;
        let mut repeat: bool = false;
        let mut x = 1;
        for ch in window {
            for y in x..=13 {
                if ch.eq(&window[y]) {
                    repeat = true;
                }
            }
            x += 1;
        }
        if !repeat {
            num_p2 += 13;
            break;
        }
    }

    println!("Part 1 : {num_p1}");
    println!("Part 2 : {num_p2}");

    Ok(())
}
