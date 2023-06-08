// --- Day 5: Supply Stacks ---
// https://adventofcode.com/2022/day/5

use std::fs;
use std::io::{Read, Result};

fn main() -> Result<()> {
    let mut input_file = fs::File::open("./src/input")?;
    let mut data = String::new();
    input_file.read_to_string(&mut data)?;

    let len = data.lines().nth(0).unwrap().len();
    let nos = (len + 1) / 4;
    let mut stack: Vec<Vec<char>> = Vec::new();
    (0..nos).for_each(|_| stack.push(Vec::new()));

    let mut c = 0;

    for line in data.lines() {
        c += 1;
        if line.chars().nth(1).unwrap().is_numeric() {
            c += 1;
            break;
        }

        let mut n = 1;
        for i in stack.iter_mut() {
            let ch = line.chars().nth(n).unwrap();
            if ch != ' ' {
                i.push(ch);
            }
            n += 4
        }
    }

    for i in stack.iter_mut() {
        i.reverse();
        println!("{:?}", i);
    }

    for line in data.lines().skip(c) {
        println!("{line}");

        let num: Vec<i32> = line
            .split(' ')
            .filter_map(|g| g.parse::<i32>().ok())
            .collect();

        let x = num[0] as usize;
        let from = num[1] as usize - 1;
        let to = num[2] as usize - 1;

        // (1..=x).for_each(|_| {
        //     let poped = stack[from].pop().unwrap();
        //     stack[to].push(poped);
        // });

        let indx = stack[from].len() - x;
        let mut poped: Vec<char> = stack[from].drain(indx..).collect();
        stack[to].append(&mut poped);

        for i in stack.iter() {
            println!("{:?}", i);
        }
    }

    let mut top_crates = String::new();
    for i in stack.iter() {
        top_crates.push(*i.last().unwrap());
    }

    println!("Top Crates are : {top_crates}");

    Ok(())
}
