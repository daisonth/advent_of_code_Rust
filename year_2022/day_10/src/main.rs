// --- Day 10: Cathode-Ray Tube ---
// https://adventofcode.com/2022/day/10

use std::{
    fs::File,
    io::{Read, Result},
};

fn main() -> Result<()> {
    let mut input_file: File = File::open("./src/input")?;
    let mut data: String = String::new();
    input_file.read_to_string(&mut data)?;

    let mut reg_x: i32 = 1;
    let mut loc: i32 = 20;
    let mut sum: i32 = 20;

    let mut cycle: i32 = 0;
    for line in data.lines() {
        let words = line.split_whitespace().collect::<Vec<&str>>();
        let instruction: &str = words[0];

        cycle += 1;
        if loc == cycle {
            sum += reg_x * cycle;
        }

        if instruction != "noop" {
            let num: i32 = words[1].parse::<i32>().unwrap();

            cycle += 1;
            reg_x *= num;
        }
    }

    Ok(())
}
