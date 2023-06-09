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

    let mut num: i32 = 0;

    for window in data.chars().collect::<Vec<char>>().windows(4) {
        num += 1;
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
            num += 3;
            break;
        }
    }

    println!("Number of characters need to be processed : {num}");
    Ok(())
}
