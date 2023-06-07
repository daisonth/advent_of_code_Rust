// --- Day 4: Camp Cleanup ---
// https://adventofcode.com/2022/day/4

use std::fs;
use std::io::{Read, Result};

fn main() -> Result<()> {
    let mut input_file = fs::File::open("./src/input")?;
    let mut data = String::new();
    input_file.read_to_string(&mut data)?;

    let mut count_p1 = 0;
    let mut count_p2 = 0;

    for line in data.lines() {
        let p: Vec<&str> = line.split(',').map(|x| x).collect();
        let a: Vec<i32> = p[0].split('-').map(|x| x.parse::<i32>().unwrap()).collect();
        let b: Vec<i32> = p[1].split('-').map(|x| x.parse::<i32>().unwrap()).collect();

        if a[0] <= b[0] && a[1] >= b[1] || b[0] <= a[0] && b[1] >= a[1] {
            count_p1 += 1;
        }

        if a[0] >= b[0] && a[0] <= b[1] || b[0] >= a[0] && b[0] <= a[1] {
            count_p2 += 1;
        }
    }

    println!("{} pairs fully contain the other", count_p1);
    println!("{} pairs overlap the other", count_p2);

    Ok(())
}
