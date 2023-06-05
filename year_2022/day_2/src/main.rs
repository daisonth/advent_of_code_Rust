// --- Day 2: Rock Paper Scissors ---
// https://adventofcode.com/2022/day/2

/*
 A and X is Rock
 B and Y is Paper
 C and Z is Scissors

 win : 6
 draw: 3
 loose: 0

 Rock : 1
 Paper : 2
 Scissors : 3
*/

use std::fs;
use std::io::Read;

fn main() -> std::io::Result<()> {
    let mut input_file = fs::File::open("./src/input")?;
    let mut file_content = String::new();
    input_file.read_to_string(&mut file_content)?;

    let mut total_score_p1 = 0;
    let mut total_score_p2 = 0;

    for line in file_content.lines() {
        let ar = [line.chars().nth(0).unwrap(), line.chars().nth(2).unwrap()];

        // PART 1
        match ar[1] {
            'X' => total_score_p1 += 1,
            'Y' => total_score_p1 += 2,
            'Z' => total_score_p1 += 3,
            _ => (),
        }

        match ar {
            ['A', 'Y'] | ['B', 'Z'] | ['C', 'X'] => total_score_p1 += 6, // win
            ['A', 'X'] | ['B', 'Y'] | ['C', 'Z'] => total_score_p1 += 3, // draw
            ['A', 'Z'] | ['C', 'Y'] | ['B', 'A'] => total_score_p1 += 0, // loose
            _ => (),
        }

        // PART 2
        match ar[1] {
            'X' => match ar[0] {
                'A' => total_score_p2 += 3,
                'B' => total_score_p2 += 1,
                'C' => total_score_p2 += 2,
                _ => (),
            },
            'Y' => match ar[0] {
                'A' => total_score_p2 += 1 + 3,
                'B' => total_score_p2 += 2 + 3,
                'C' => total_score_p2 += 3 + 3,
                _ => (),
            },
            'Z' => match ar[0] {
                'A' => total_score_p2 += 2 + 6,
                'B' => total_score_p2 += 3 + 6,
                'C' => total_score_p2 += 1 + 6,
                _ => (),
            },
            _ => (),
        }
    }

    println!("Part 1 | Totoal score : {total_score_p1}");
    println!("Part 2 | Totoal score : {total_score_p2}");

    Ok(())
}
