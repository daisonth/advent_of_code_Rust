use crate::Monkey;
pub fn get_each_monkey_data(para: &str) -> Monkey {
    // println!("{para}");
    let mut items: Vec<i64> = Vec::new();
    let inspection_count = 0;

    for l in para.lines().nth(1).unwrap().split_whitespace() {
        if l.chars().nth(0).unwrap().is_numeric() {
            items.insert(0, l.split(',').nth(0).unwrap().parse::<i64>().unwrap());
        }
    }

    let operation = para
        .lines()
        .nth(2)
        .unwrap()
        .split("=")
        .nth(1)
        .unwrap()
        .split_whitespace()
        .nth(1)
        .unwrap()
        .chars()
        .nth(0)
        .unwrap();

    // println!("{operation}");

    let operand = para
        .lines()
        .nth(2)
        .unwrap()
        .split("=")
        .nth(1)
        .unwrap()
        .split_whitespace()
        .nth(2)
        .unwrap()
        .to_string();

    // println!("{operand}");

    let divisible = para
        .lines()
        .nth(3)
        .unwrap()
        .split_whitespace()
        .last()
        .unwrap()
        .parse::<i64>()
        .unwrap();
    // println!("{divisible}");

    let if_true = para
        .lines()
        .nth(4)
        .unwrap()
        .split_whitespace()
        .last()
        .unwrap()
        .parse::<i64>()
        .unwrap();

    let if_false = para
        .lines()
        .nth(5)
        .unwrap()
        .split_whitespace()
        .last()
        .unwrap()
        .parse::<i64>()
        .unwrap();

    // println!("{if_true} , {if_false}");

    Monkey {
        items,
        inspection_count,
        operation,
        operand,
        divisible,
        if_true,
        if_false,
    }
}
