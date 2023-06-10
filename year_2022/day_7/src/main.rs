// --- Day 7: No Space Left On Device ---
// https://adventofcode.com/2022/day/7

use std::{
    fs::File,
    io::{Read, Result},
};

fn main() -> Result<()> {
    let mut input_file = File::open("./src/input")?;
    let mut data = String::new();
    input_file.read_to_string(&mut data)?;

    let mut stack: Vec<(&str, i32)> = Vec::new();
    let mut top = 0;

    let mut sum: i32 = 0;

    for line in data.lines() {
        let words = line.split_whitespace().collect::<Vec<&str>>();

        match words[0] {
            "$" => match words[1] {
                "cd" => match words[2] {
                    ".." => {
                        stack[top - 1].1 += stack[top].1;
                        top -= 1;
                        let poped = stack.pop().unwrap();
                        if poped.1 <= 100000 {
                            sum += poped.1;
                        }
                        // println!("back to {:?}", poped);
                    }
                    word => {
                        if word.ne("/") {
                            top += 1
                        }
                        stack.push((word, 0));
                        // println!("{:?} pushed", stack.last().unwrap());
                    }
                },
                _ => (),
            },
            "dir" => {}
            x => {
                let len = stack.len() - 1;
                stack[len].1 += x.parse::<i32>().unwrap();
            }
        }
        // print_stack(stack.clone());
    }

    for _ in 1..stack.len() {
        stack[top - 1].1 += stack[top].1;
        top -= 1;
        let poped = stack.pop().unwrap();
        if poped.1 <= 100000 {
            sum += poped.1;
        }

        // print_stack(stack.clone());
    }
    println!("SUM = {sum}");

    Ok(())
}
//
// fn print_stack(stack: Vec<(&str, i32)>) {
//     println!("");
//     for i in stack.iter() {
//         println!("{:?}", i);
//     }
// }
