// --- Day 10: Cathode-Ray Tube ---
// https://adventofcode.com/2022/day/10

// Part 2

pub fn part_2(data: String) {
    let mut reg_x: i32 = 1;
    let mut cycle: i32 = 0;
    let mut splice: i32 = 1;

    for line in data.lines() {
        let words: Vec<&str> = line.split_whitespace().collect();
        let instruction: &str = words[0];

        let value: i32;
        let cycles: i32;

        if instruction == "addx" {
            value = words[1].parse::<i32>().unwrap();
            cycles = 2;
        } else {
            value = 0;
            cycles = 1;
        };

        for _ in 0..cycles {
            if cycle == splice - 1 || cycle == splice || cycle == splice + 1 {
                print!("#");
            } else {
                print!(".");
            }
            if cycle == 39 {
                print!("\n");
                cycle = 0;
            } else {
                cycle += 1;
            }
        }

        reg_x += value;

        splice = reg_x;
    }
}
