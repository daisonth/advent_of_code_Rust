// --- Day 6: Tuning Trouble ---
// https://adventofcode.com/2022/day/6

use std::{
    fs,
    io::{Read, Result},
};

fn main() -> Result<()> {
    let mut input_file = fs::File::open("./src/input")?;
    let mut data = String::new();
    input_file.read_to_string(&mut data)?;

    let iter = data.chars().collect::<Vec<char>>();
    let sliding_window = iter.windows(4);

    let mut num: i32 = 0;

    for window in sliding_window {
        num += 1;
        let mut repeat = false;
        let mut x = 1;
        for ch in window {
            (x..=3).for_each(|y| {
                if ch.eq(&window[y]) {
                    repeat = true;
                }
            });
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
