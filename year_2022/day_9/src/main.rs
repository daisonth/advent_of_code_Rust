// --- Day 9: Rope Bridge ---
// https://adventofcode.com/2022/day/9

// PART 1

pub mod part2;

use std::{
    cmp,
    fs::File,
    io::{Read, Result},
};

type Point<T> = (T, T);

fn main() -> Result<()> {
    let mut input_file = File::open("./src/input")?;
    let mut data = String::new();
    input_file.read_to_string(&mut data)?;

    let s: Point<i32> = (0, 0);
    let mut t: Point<i32> = (0, 0);
    let mut h: Point<i32> = (0, 0);

    let mut position: Vec<Point<i32>> = Vec::new();
    position.push(s);

    for line in data.lines() {
        let parts = line.split_whitespace().collect::<Vec<&str>>();
        let direction = parts[0];
        let num = parts[1].parse::<i32>().unwrap();

        for _ in 0..num {
            let head = h;
            match direction {
                "R" => h.1 += 1,
                "L" => h.1 -= 1,
                "D" => h.0 += 1,
                "U" => h.0 -= 1,
                _ => {}
            }
            if h_is_not_near(h, t) {
                t = head;
                position.push(t);
            }
        }
    }

    position.sort();
    position.dedup();
    println!("Part 1 : Number of positions : {}", position.len());

    part2::part_2(data);

    Ok(())
}

fn h_is_not_near(t: Point<i32>, h: Point<i32>) -> bool {
    if t == h {
        return false;
    }

    let mut max;
    let mut min;
    let difference;

    if t.0 == h.0 {
        max = cmp::max(h.1, t.1);
        min = cmp::min(h.1, t.1);
        difference = max - min;
    } else if t.1 == h.1 {
        max = cmp::max(h.0, t.0);
        min = cmp::min(h.0, t.0);
        difference = max - min;
    } else {
        max = cmp::max(h.0, t.0);
        min = cmp::min(h.0, t.0);
        let d1 = max - min;
        max = cmp::max(h.1, t.1);
        min = cmp::min(h.1, t.1);
        let d2 = max - min;
        difference = d1 * d2;
    }

    match difference {
        1 => return false,
        _ => return true,
    }
}
