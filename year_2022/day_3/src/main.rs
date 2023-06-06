// --- Day 3: Rucksack Reorganization ---
// https://adventofcode.com/2022/day/3

use std::fs;
use std::io::Read;

fn part1(data: &String, alphabets: &Vec<char>) -> i32 {
    let mut sum: i32 = 0;

    for line in data.lines() {
        let (s1, s2) = line.split_at(line.len() / 2);
        let mut common = ' ';

        s1.chars().for_each(|x| match s2.contains(x) {
            true => common = x,
            false => (),
        });

        sum += alphabets.iter().position(|&x| x == common).unwrap() as i32 + 1;
    }
    sum
}

fn part2(data: String, alphabets: Vec<char>) -> i32 {
    let mut sum: i32 = 0;
    let mut lines = vec![];
    data.lines().for_each(|x| lines.push(x));

    for l in lines.chunks(3) {
        let mut common = ' ';

        l[0].chars().for_each(|x| {
            if l[1].contains(x) && l[2].contains(x) {
                common = x
            }
        });

        sum += alphabets.iter().position(|&x| x == common).unwrap() as i32 + 1;
    }
    sum
}

fn main() -> std::io::Result<()> {
    let mut input_file = fs::File::open("./src/input")?;
    let mut data = String::new();
    input_file.read_to_string(&mut data)?;

    let alphabets = (b'a'..=b'z')
        .chain(b'A'..=b'Z')
        .map(|a| a as char)
        .collect::<Vec<_>>();

    println!("Part 1 Answer : {}", part1(&data, &alphabets));
    println!("Part 2 Answer : {}", part2(data, alphabets));

    Ok(())
}
