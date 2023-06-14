// --- Day 10: Cathode-Ray Tube ---
// https://adventofcode.com/2022/day/10

// Part 1

mod part2;

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
    let mut sum: i32 = 0;

    let mut cycle: i32 = 0;

    for line in data.lines() {
        let words = line.split_whitespace().collect::<Vec<&str>>();
        let instruction: &str = words[0];

        let value: i32;
        let cycles: i32;

        if instruction == "addx" {
            value = words[1].parse::<i32>().unwrap();
            cycles = 2;
        } else {
            value = 0;
            cycles = 1;
        };

        for _ in 0..cycles {
            cycle += 1;

            if loc == cycle {
                sum += cycle * reg_x;
                loc += 40;
            }
        }
        reg_x += value;

        if cycle >= 240 {
            break;
        }
    }

    println!("Part 1 : sum = {sum}");
    part2::part_2(data);

    Ok(())
}
