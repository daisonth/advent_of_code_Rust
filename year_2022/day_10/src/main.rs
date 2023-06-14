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
    let mut sum: i32 = 0;

    let mut cycle: i32 = 0;

    for line in data.lines() {
        let words = line.split_whitespace().collect::<Vec<&str>>();
        let instruction: &str = words[0];

        let num: i32;
        let i: i32;

        if instruction == "addx" {
            num = words[1].parse::<i32>().unwrap();
            i = 2;
        } else {
            num = 0;
            i = 1;
        };

        for _ in 0..i {
            cycle += 1;
            if loc == cycle {
                sum += cycle * reg_x;
                // println!("regx: {reg_x} | loc : {cycle} | val : {}", cycle * reg_x);
                loc += 40;
            }
        }
        reg_x += num;

        if cycle >= 220 {
            break;
        }
    }

    println!("Part 1 : sum = {sum}");

    Ok(())
}
