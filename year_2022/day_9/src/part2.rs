// --- Day 9: Rope Bridge ---
// https://adventofcode.com/2022/day/9

// PART 2

use crate::get_next_tail_position;

type Point<T> = (T, T);

pub fn part_2(data: String) {
    let s: Point<i32> = (0, 0);

    let mut knot: [Point<i32>; 10] = [s; 10];

    let mut position: Vec<Point<i32>> = Vec::new();
    position.push(s);

    for line in data.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect::<Vec<&str>>();
        let direction: &str = parts[0];
        let num: i32 = parts[1].parse::<i32>().unwrap();

        for _ in 0..num {
            match direction {
                "R" => knot[0].1 += 1,
                "L" => knot[0].1 -= 1,
                "D" => knot[0].0 += 1,
                "U" => knot[0].0 -= 1,
                _ => {}
            }

            for i in 1..=9 {
                knot[i] = get_next_tail_position(knot[i], knot[i - 1]);
                if i == 9 {
                    position.push(knot[9]);
                }
            }
        }
    }

    position.sort();
    position.dedup();

    println!("Part 2 : Number of positions : {}", position.len());
}
