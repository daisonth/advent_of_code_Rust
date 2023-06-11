// --- Day 8: Treetop Tree House ---
// https://adventofcode.com/2022/day/8

use std::{
    fs::File,
    io::{Read, Result},
};

fn main() -> Result<()> {
    let mut input_file = File::open("./src/input")?;
    let mut data = String::new();
    input_file.read_to_string(&mut data)?;

    let mut matrix: Vec<Vec<i32>> = Vec::new();

    for line in data.lines() {
        let ar = line
            .chars()
            .map(|a| a.to_digit(10).unwrap() as i32)
            .collect::<Vec<i32>>();
        matrix.push(ar)
    }

    let len = matrix.len();
    let wid = matrix[1].len();
    println!("length : {len}");
    println!("width : {wid}\n");

    let mut count = (len * 2) + ((wid - 2) * 2);
    let mut g = 0;

    let mut hs = 0;
    for j in 1..len - 1 {
        for k in 1..wid - 1 {
            g += 1;
            let num = matrix[j][k];

            let mut t: bool = true;
            let mut b: bool = true;
            let mut r: bool = true;
            let mut l: bool = true;

            let mut st: i32 = 0;
            let mut sb: i32 = 0;
            let mut sl: i32 = 0;
            let mut sr: i32 = 0;

            //check to top
            for x in (0..j).rev() {
                st += 1;
                if matrix[x][k] >= num {
                    t = false;
                    break;
                }
            }

            //check to bottom
            for x in j + 1..len {
                sb += 1;
                if matrix[x][k] >= num {
                    b = false;
                    break;
                }
            }

            //check to left
            for x in (0..k).rev() {
                sl += 1;
                if matrix[j][x] >= num {
                    l = false;
                    break;
                }
            }

            //check to right
            for x in k + 1..wid {
                sr += 1;
                if matrix[j][x] >= num {
                    r = false;
                    break;
                }
                // println!("{} at {}", num, matrix[j][x]);
            }

            if r || l || t || b {
                count += 1;
                // println!("{num} at [{j}][{k}]");
            }

            let temp_s = st * sb * sl * sr;
            if temp_s > hs {
                hs = temp_s;
            }
        }
    }
    println!("Part 1 : count = {count}");
    println!("Part 2 : Highest Scenic Score = {hs}");
    // println!("loops : {g}");

    // for ar in matrix.iter() {
    //     println!("{:?}", ar);
    // }
    Ok(())
}
