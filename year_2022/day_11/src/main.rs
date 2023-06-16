// --- Day 11: Monkey in the Middle ---
// https://adventofcode.com/2022/day/11

use std::{
    fs::File,
    io::{Read, Result},
};

struct Monkey {
    items: Vec<i32>,
    inspection_count: i32,
    operation: char,
    operand: String,
    divisible: i32,
    if_true: i32,
    if_false: i32,
}

fn main() -> Result<()> {
    let mut input_file = File::open("./src/input")?;
    let mut data = String::new();
    input_file.read_to_string(&mut data)?;

    let mut monkeys: Vec<Monkey> = Vec::new();

    for para in data.split("\n\n").collect::<Vec<&str>>().iter() {
        monkeys.push(get_each_monkey_data(para));
    }

    for _ in 0..20 {
        // println!("for round: {}\n===============================", r + 1);
        for n in 0..monkeys.len() {
            for _ in 0..monkeys[n].items.len() {
                let item = monkeys[n].items.pop().unwrap();

                let operand;
                match monkeys[n].operand.as_str() {
                    "old" => operand = item,
                    _ => operand = monkeys[n].operand.parse::<i32>().unwrap(),
                }

                let mut worry_level: i32 = 0;
                match monkeys[n].operation {
                    '*' => worry_level = item * operand,
                    '+' => worry_level = item + operand,
                    _ => (),
                }
                // println!("operand : {operand}");

                worry_level /= 3;
                // println!("worry_level : {worry_level}");

                let val = worry_level % monkeys[n].divisible;

                let if_true = monkeys[n].if_true as usize;
                let if_false = monkeys[n].if_false as usize;
                match val == 0 {
                    true => monkeys[if_true].items.insert(0, worry_level),
                    false => monkeys[if_false].items.insert(0, worry_level),
                }
                monkeys[n].inspection_count += 1;
            }
            // println!();
        }

        // println!("for round: {}", r + 1);
        // for n in 0..monkeys.len() {
        //     print!("monkey {n}:");
        //     for m in monkeys[n].items.iter() {
        //         print!(" {m}");
        //     }
        //     println!("");
        // }
        // println!("");
    }

    let mut v: Vec<i32> = Vec::new();
    for m in monkeys.iter() {
        v.push(m.inspection_count);
    }

    v.sort();
    v.reverse();
    println!("Part 1 : level of Monkey business = {}", v[0] * v[1]);

    Ok(())
}

fn get_each_monkey_data(para: &str) -> Monkey {
    // println!("{para}");
    let mut items: Vec<i32> = Vec::new();
    let inspection_count = 0;

    for l in para.lines().nth(1).unwrap().split_whitespace() {
        if l.chars().nth(0).unwrap().is_numeric() {
            items.insert(0, l.split(',').nth(0).unwrap().parse::<i32>().unwrap());
        }
    }

    let operation = para
        .lines()
        .nth(2)
        .unwrap()
        .split("=")
        .nth(1)
        .unwrap()
        .split_whitespace()
        .nth(1)
        .unwrap()
        .chars()
        .nth(0)
        .unwrap();

    // println!("{operation}");

    let operand = para
        .lines()
        .nth(2)
        .unwrap()
        .split("=")
        .nth(1)
        .unwrap()
        .split_whitespace()
        .nth(2)
        .unwrap()
        .to_string();

    // println!("{operand}");

    let divisible = para
        .lines()
        .nth(3)
        .unwrap()
        .split_whitespace()
        .last()
        .unwrap()
        .parse::<i32>()
        .unwrap();
    // println!("{divisible}");

    let if_true = para
        .lines()
        .nth(4)
        .unwrap()
        .split_whitespace()
        .last()
        .unwrap()
        .parse::<i32>()
        .unwrap();

    let if_false = para
        .lines()
        .nth(5)
        .unwrap()
        .split_whitespace()
        .last()
        .unwrap()
        .parse::<i32>()
        .unwrap();

    // println!("{if_true} , {if_false}");

    Monkey {
        items,
        inspection_count,
        operation,
        operand,
        divisible,
        if_true,
        if_false,
    }
}
