// --- Day 11: Monkey in the Middle ---
// https://adventofcode.com/2022/day/11

mod functions;
mod part1;
mod part2;

use std::{
    fs::File,
    io::{Read, Result},
};

#[derive(Clone)]
pub struct Monkey {
    items: Vec<i64>,
    inspection_count: i64,
    operation: char,
    operand: String,
    divisible: i64,
    if_true: i64,
    if_false: i64,
}

fn main() -> Result<()> {
    let mut input_file = File::open("./src/input")?;
    let mut data = String::new();
    input_file.read_to_string(&mut data)?;

    let mut monkeys: Vec<Monkey> = Vec::new();

    for para in data.split("\n\n").collect::<Vec<&str>>().iter() {
        monkeys.push(functions::get_each_monkey_data(para));
    }

    let part1: i64 = part1::part_1(monkeys.clone());
    let part2: i64 = part2::part_2(monkeys);

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");

    Ok(())
}
