// --- Day 11: Monkey in the Middle ---
// https://adventofcode.com/2022/day/11

mod functions;
mod part2;

use std::{
    fs::File,
    io::{Read, Result},
};

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

    part2::part_2(monkeys);

    let mut the_mod = 1;
    for monkey in monkeys.iter() {
        the_mod *= monkey.divisible;
    }

    for _ in 1..10001 {
        for n in 0..monkeys.len() {
            for _ in 0..monkeys[n].items.len() {
                let item: i64 = monkeys[n].items.pop().unwrap();

                let operand: i64;
                match monkeys[n].operand.as_str() {
                    "old" => operand = item,
                    _ => operand = monkeys[n].operand.parse::<i64>().unwrap(),
                }

                let mut worry_level: i64 = 0;
                match monkeys[n].operation {
                    '*' => worry_level = item.wrapping_mul(operand),
                    '+' => worry_level = item + operand,
                    _ => (),
                }

                // worry_level = worry_level.wrapping_mul(3);
                worry_level = worry_level % the_mod;

                let val = worry_level % monkeys[n].divisible;

                let if_true = monkeys[n].if_true as usize;
                let if_false = monkeys[n].if_false as usize;
                match val == 0 {
                    true => monkeys[if_true].items.insert(0, worry_level),
                    false => monkeys[if_false].items.insert(0, worry_level),
                }
                monkeys[n].inspection_count += 1;
            }
        }
    }

    let mut inspect_counts: Vec<i64> = Vec::new();
    for m in monkeys.iter() {
        inspect_counts.push(m.inspection_count);
    }

    inspect_counts.sort();
    inspect_counts.reverse();
    println!(
        "Part 1 : level of Monkey business = {}",
        inspect_counts[0] * inspect_counts[1]
    );

    Ok(())
}
