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

    // part2::part_2(data);

    let f: i32 = 5;
    let g: i32 = -3;
    println!("result : {}", g - f);
    println!("result : {}", f.abs() - g.abs());
    println!("result : {}", f.abs_diff(g));
    println!("result : {}", g.abs_diff(f));

    Ok(())
}

// fn get_next_tail_position(t: Point<i32>, h: Point<i32>) -> Point<i32> {
//     let new: Point<i32> = (0, 0);
//     // let mut dx = h.0 - t.0;
//     // let mut dy = h.0 - t.0;
//
//     new
// }

fn get_next_tail_position(t: Point<i32>, h: Point<i32>) -> Point<i32> {
    let dx = h.0 - t.0;
    let dy = h.1 - t.1;

    if !dx.abs() <= 1 && !dy.abs() <= 1 {
        return t;
    }

    if dx > 1 {
        t.0 += 1;
        if dy >= 1 {
            t.1 += 1;
        }
        if dy <= -1 {}
    }

    return t;
}

// fn h_is_not_near(t: Point<i32>, h: Point<i32>) -> bool {
//     if t == h {
//         return false;
//     }
//
//     let difference;
//
//     if t.0 == h.0 {
//         difference = h.1.abs_diff(t.1);
//     } else if t.1 == h.1 {
//         difference = h.0.abs_diff(t.0);
//     } else {
//         let d1 = h.0.abs_diff(t.0);
//         let d2 = h.1.abs_diff(t.1);
//         difference = d1 * d2;
//     }
//
//     match difference {
//         1 => return false,
//         _ => return true,
//     }
// }
