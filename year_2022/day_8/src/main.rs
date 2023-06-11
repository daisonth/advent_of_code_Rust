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
    for j in 1..len - 1 {
        for k in 1..wid - 1 {
            g += 1;
            let num = matrix[j][k];
            let mut t: bool = true;
            let mut b: bool = true;
            let mut r: bool = true;
            let mut l: bool = true;

            //check to top
            for x in 0..j {
                if matrix[x][k] >= num {
                    t = false;
                }
            }

            //check to bottom
            for x in j + 1..len {
                if matrix[x][k] >= num {
                    b = false;
                }
            }

            //check to left
            for x in 0..k {
                if matrix[j][x] >= num {
                    l = false;
                }
            }

            //check to right
            for x in k + 1..wid {
                if matrix[j][x] >= num {
                    r = false;
                }
                // println!("{} at {}", num, matrix[j][x]);
            }

            if r || l || t || b {
                count += 1;
                // println!("{num} at [{j}][{k}]");
            }
        }
    }
    println!("count : {count}");
    println!("loops : {g}");

    // for ar in matrix.iter() {
    //     println!("{:?}", ar);
    // }
    Ok(())
}
