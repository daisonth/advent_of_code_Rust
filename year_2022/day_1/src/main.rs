// --- Day 1: Calorie Counting ---
// https://adventofcode.com/2022/day/1

use std::fs;
use std::io::Read;

fn main() -> std::io::Result<()> {
    // open and read the file content
    let mut input_file = fs::File::open("./src/input")?;
    let mut file_content: String = String::new();
    input_file.read_to_string(&mut file_content)?;

    let mut largest: i32 = 0;
    let mut second_largest: i32 = 0;
    let mut third_largest: i32 = 0;

    let iter = file_content.split("\n\n");
    for i in iter {
        let mut sum: i32 = 0;
        i.lines().for_each(|j| sum += j.parse::<i32>().unwrap());
        if sum > largest {
            third_largest = second_largest;
            second_largest = largest;
            largest = sum;
        } else if sum > second_largest {
            third_largest = second_largest;
            second_largest = sum;
        } else if sum > third_largest {
            third_largest = sum;
        }
    }

    println!("Elf carrying most calories have {largest} calories");
    println!("Elf carrying 2nd most calories have {second_largest} calories");
    println!("Elf carrying 3rd most calories have {third_largest} calories");

    println!(
        "Total calories caried by the Top 3 elves is {} calories",
        third_largest + second_largest + largest
    );
    Ok(())
}
