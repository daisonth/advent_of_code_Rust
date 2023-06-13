// --- Day 9: Rope Bridge ---
// https://adventofcode.com/2022/day/9

// PART 1

pub mod part2;

use std::{
    fs::File,
    io::{Read, Result},
};

type Point<T> = (T, T);

fn main() -> Result<()> {
    let mut input_file: File = File::open("./src/input")?;
    let mut data: String = String::new();
    input_file.read_to_string(&mut data)?;

    let s: Point<i32> = (0, 0);
    let mut t: Point<i32> = (0, 0);
    let mut h: Point<i32> = (0, 0);

    let mut position: Vec<Point<i32>> = Vec::new();
    position.push(s);

    for line in data.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect::<Vec<&str>>();
        let direction: &str = parts[0];
        let num: i32 = parts[1].parse::<i32>().unwrap();

        for _ in 0..num {
            match direction {
                "R" => h.1 += 1,
                "L" => h.1 -= 1,
                "D" => h.0 += 1,
                "U" => h.0 -= 1,
                _ => {}
            }
            t = get_next_tail_position(t, h);
            position.push(t);
        }
    }

    position.sort();
    position.dedup();

    println!("Part 1 : Number of positions : {}", position.len());

    part2::part_2(data);

    Ok(())
}

fn get_next_tail_position(mut t: Point<i32>, h: Point<i32>) -> (i32, i32) {
    let dx: i32 = h.0 - t.0;
    let dy: i32 = h.1 - t.1;

    if dx.abs() <= 1 && dy.abs() <= 1 {
        return t;
    } else if dx > 1 {
        t.0 += 1;
        if dy > 0 {
            t.1 += 1;
        }
        if dy < 0 {
            t.1 -= 1;
        }
    } else if dx < -1 {
        t.0 -= 1;
        if dy > 0 {
            t.1 += 1;
        }
        if dy < 0 {
            t.1 -= 1;
        }
    } else if dy > 1 {
        t.1 += 1;
        if dx > 0 {
            t.0 += 1;
        }
        if dx < 0 {
            t.0 -= 1;
        }
    } else if dy < -1 {
        t.1 -= 1;
        if dx > 0 {
            t.0 += 1;
        }
        if dx < 0 {
            t.0 -= 1;
        }
    }

    return t;
}
