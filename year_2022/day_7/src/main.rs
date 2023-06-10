// --- Day 7: No Space Left On Device ---
// https://adventofcode.com/2022/day/7

use std::{
    fs::File,
    io::{Read, Result},
};

fn main() -> Result<()> {
    let mut input_file = File::open("input")?;
    let mut data = String::new();
    input_file.read_to_string(&mut data)?;

    Ok(())
}
