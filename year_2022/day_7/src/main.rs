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
    let mut top: usize = 0;

    let used_space = calculate_used_space(data.clone());

    let available_space = 70000000 - used_space;
    let delete_space = 30000000 - available_space;

    let mut ddir_space: i32 = used_space;

    let mut sum: i32 = 0;

    for line in data.lines() {
        let words = line.split_whitespace().collect::<Vec<&str>>();

        match words[0] {
            "$" => match words[1] {
                "cd" => match words[2] {
                    ".." => {
                        if top > 0 {
                            stack[top - 1].1 += stack[top].1;
                            top -= 1;
                        }

                        let poped = stack.pop().unwrap();
                        if poped.1 <= 100000 {
                            sum += poped.1;
                        }

                        if poped.1 >= delete_space && poped.1 <= ddir_space {
                            ddir_space = poped.1;
                        }
                    }
                    word => {
                        if word.ne("/") {
                            top += 1
                        }
                        stack.push((word, 0));
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
    }

    let mut len = stack.len();

    while len != 0 {
        if top > 0 {
            stack[top - 1].1 += stack[top].1;
            top -= 1;
        }

        let poped = stack.pop().unwrap();
        if poped.1 <= 100000 {
            sum += poped.1;
        }

        if poped.1 >= delete_space && poped.1 <= ddir_space {
            ddir_space = poped.1;
        }
        len -= 1;
    }

    println!("Part 1: SUM = {sum}");
    println!("Part 2: Size of directory to be deleted  = {ddir_space}");

    Ok(())
}

fn calculate_used_space(data: String) -> i32 {
    let mut used_space = 0;
    for line in data.lines() {
        let words = line.split_whitespace().collect::<Vec<&str>>();
        match words[0] {
            "$" | "dir" => {}
            x => used_space += x.parse::<i32>().unwrap(),
        }
    }
    used_space
}
