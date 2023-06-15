// --- Day 11: Monkey in the Middle ---
// https://adventofcode.com/2022/day/11

use std::{
    fs::File,
    io::{Read, Result},
};

struct Monkey {
    items: Vec<i32>,
    inspection_count: i32,
    operation: String,
    divisible: i32,
    if_true: i32,
    if_false: i32,
}

fn main() -> Result<()> {
    let mut input_file = File::open("./src/input")?;
    let mut data = String::new();
    input_file.read_to_string(&mut data)?;

    let mut monkeys: Vec<Monkey> = get_each_monkey_data(data);

    Ok(())
}

fn get_each_monkey_data(data: String) -> Vec<Monkey> {
    let mut monkeys: Vec<Monkey> = Vec::new();

    for para in data.split("\n\n").collect::<Vec<&str>>().iter() {
        let mut items: Vec<i32> = Vec::new();
        let inspection_count = 0;

        for l in para.lines().nth(1).unwrap().split_whitespace() {
            if l.chars().nth(0).unwrap().is_numeric() {
                items.push(l.split(',').nth(0).unwrap().parse::<i32>().unwrap());
            }
        }

        let operation = para
            .lines()
            .nth(3)
            .unwrap()
            .split("old")
            .nth(1)
            .unwrap()
            .to_string();

        let divisible = para
            .lines()
            .nth(4)
            .unwrap()
            .split_whitespace()
            .last()
            .unwrap()
            .parse::<i32>()
            .unwrap();

        let if_true = para
            .lines()
            .nth(5)
            .unwrap()
            .split_whitespace()
            .last()
            .unwrap()
            .parse::<i32>()
            .unwrap();

        let if_false = para
            .lines()
            .nth(6)
            .unwrap()
            .split_whitespace()
            .last()
            .unwrap()
            .parse::<i32>()
            .unwrap();

        let monkey: Monkey = Monkey {
            items,
            inspection_count,
            operation,
            divisible,
            if_true,
            if_false,
        };

        monkeys.push(monkey);
    }
    monkeys
}
