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

        match direction {
            "R" => {
                for _ in 0..num {
                    for i in 0..9 {
                        print!("{i}");
                        if h_is_not_near((knot[i].0, knot[i].1 + 1), knot[i + 1]) {
                            knot[i + 1] = knot[i];
                            if i == 8 {
                                // println!("{:?}", knot[9]);
                                position.push(knot[9]);
                            }
                        }
                        knot[i].1 += 1;
                    }
                    println!("");
                }
            }
            "L" => {
                for _ in 0..num {
                    for i in 0..9 {
                        print!("{i}");
                        if h_is_not_near((knot[i].0, knot[i].1 - 1), knot[i + 1]) {
                            knot[i + 1] = knot[i];
                            if i == 8 {
                                // println!("{:?}", knot[9]);
                                position.push(knot[9]);
                            }
                        }
                        knot[0].1 -= 1;
                    }
                }
            }
            "D" => {
                for _ in 0..num {
                    for i in 0..9 {
                        print!("{i}");
                        if h_is_not_near((knot[i].0 + 1, knot[i].1), knot[i + 1]) {
                            knot[i + 1] = knot[i];
                            if i == 8 {
                                // println!("{:?}", knot[9]);
                                position.push(knot[9]);
                            }
                        }
                        knot[0].0 += 1;
                    }
                }
            }
            "U" => {
                for _ in 0..num {
                    for i in 0..9 {
                        print!("{i}");
                        if h_is_not_near((knot[i].0 - 1, knot[i].1), knot[i + 1]) {
                            knot[i + 1] = knot[i];
                            if i == 8 {
                                // println!("{:?}", knot[9]);
                                position.push(knot[9]);
                            }
                        }
                        knot[0].0 -= 1;
                    }
                }
            }
            _ => {}
        }
    }

    // position.sort();
    // position.dedup();

    // for i in position.iter() {
    //     println!("{:?}", i);
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
