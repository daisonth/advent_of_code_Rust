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
    // println!("length : {len}");
    // println!("width : {wid}\n");

    let mut count = (len * 2) + ((wid - 2) * 2);
    let mut highest_sc = 0;

    for j in 1..len - 1 {
        for k in 1..wid - 1 {
            let num = matrix[j][k];

            let mut visible_top: bool = true;
            let mut visible_bottom: bool = true;
            let mut visible_right: bool = true;
            let mut visible_left: bool = true;

            let mut top_sc: i32 = 0;
            let mut botton_sc: i32 = 0;
            let mut left_sc: i32 = 0;
            let mut right_sc: i32 = 0;

            //check to top
            for x in (0..j).rev() {
                top_sc += 1;
                if matrix[x][k] >= num {
                    visible_top = false;
                    break;
                }
            }

            //check to bottom
            for x in j + 1..len {
                botton_sc += 1;
                if matrix[x][k] >= num {
                    visible_bottom = false;
                    break;
                }
            }

            //check to left
            for x in (0..k).rev() {
                left_sc += 1;
                if matrix[j][x] >= num {
                    visible_left = false;
                    break;
                }
            }

            //check to right
            for x in k + 1..wid {
                right_sc += 1;
                if matrix[j][x] >= num {
                    visible_right = false;
                    break;
                }
            }

            if visible_right || visible_left || visible_bottom || visible_top {
                count += 1;
            }

            let temp_s = top_sc * botton_sc * left_sc * right_sc;
            if temp_s > highest_sc {
                highest_sc = temp_s;
            }
        }
    }
    println!("Part 1 : count = {count}");
    println!("Part 2 : Highest Scenic Score = {highest_sc}");
    Ok(())
}
