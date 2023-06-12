// --- Day 9: Rope Bridge ---
// https://adventofcode.com/2022/day/9

// PART 2

use std::cmp;

type Point<T> = (T, T);

pub fn part_2(data: String) {
    let s: Point<i32> = (0, 0);

    let mut knot: [Point<i32>; 10] = [s; 10];
    let mut position: Vec<Point<i32>> = Vec::new();
    position.push(s);

    for line in data.lines() {
        let parts = line.split_whitespace().collect::<Vec<&str>>();
        let direction = parts[0];
        let num = parts[1].parse::<i32>().unwrap();

        for _ in 0..num {
            for i in knot.iter() {
                print!("{:?} ", i);
            }

            println!("");
            let mut head: Point<i32> = knot[0];
            match direction {
                "R" => head.1 += 1,
                "L" => head.1 -= 1,
                "D" => head.0 += 1,
                "U" => head.0 -= 1,
                _ => {}
            }
            for i in (1..=9).rev() {
                if h_is_not_near(knot[i - 1], knot[i]) {
                    knot[i] = knot[i - 1];
                } else {
                    break;
                }
                knot[0] = head;
                position.push(knot[9]);
            }
        }
    }

    position.sort();
    position.dedup();

    // for i in position.iter() {
    //     println!("{:?}", i)
    // }

    println!("Part 2 : Number of positions : {}", position.len());
}

fn h_is_not_near(h: Point<i32>, t: Point<i32>) -> bool {
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
        1 => {
            return false;
        }
        _ => return true,
    }
}
