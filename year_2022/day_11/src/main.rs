// --- Day 11: Monkey in the Middle ---
// https://adventofcode.com/2022/day/11

use std::{
    fs::File,
    io::{Read, Result},
};

struct Monkey<'a> {
    items: &'a Vec<i32>,
    inspection_count: i32,
    operation: String,
    divisible: i32,
    if_true: i32,
    if_false: i32,
}

// impl Monkey {
//     fn get_item(&mut self) -> i32 {
//         self.items.pop().unwrap()
//     }
// }

fn main() -> Result<()> {
    let mut input_file = File::open("./src/input")?;
    let mut data = String::new();
    input_file.read_to_string(&mut data)?;

    // let mut monkeys: Vec<Monkey> = get_each_monkey_data(data);
    let mut monkeys: Vec<Monkey> = Vec::new();

    for para in data.split("\n\n").collect::<Vec<&str>>().iter() {
        monkeys.insert(0, get_each_monkey_data(para));
    }

    for monkey in monkeys.iter() {
        monkey.items.pop().unwrap();
    }

    Ok(())
}

fn get_each_monkey_data<'a>(para: &str) -> Monkey {
    let mut items: Vec<i32> = Vec::new();
    let inspection_count = 0;

    for l in para.lines().nth(1).unwrap().split_whitespace() {
        if l.chars().nth(0).unwrap().is_numeric() {
            items.insert(0, l.split(',').nth(0).unwrap().parse::<i32>().unwrap());
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

    Monkey {
        items,
        inspection_count,
        operation,
        divisible,
        if_true,
        if_false,
    }
}
