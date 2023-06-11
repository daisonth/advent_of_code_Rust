// --- Day 8: Treetop Tree House ---
// https://adventofcode.com/2022/day/8

use std::{
    fs::File,
    io::{Read, Result},
};

fn main() -> Result<()> {
    let mut input_file = File::open("./src/input")?;
    let mut data = String::new();
    input_file.read_to_string(&mut data)?;

    Ok(())
}
