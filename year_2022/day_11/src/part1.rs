use crate::Monkey;

pub fn part_1(mut monkeys: Vec<Monkey>) -> i64 {
    for _ in 0..20 {
        for n in 0..monkeys.len() {
            for _ in 0..monkeys[n].items.len() {
                let item: i64 = monkeys[n].items.pop().unwrap();

                let operand: i64;
                match monkeys[n].operand.as_str() {
                    "old" => operand = item,
                    _ => operand = monkeys[n].operand.parse::<i64>().unwrap(),
                }

                let mut worry_level: i64 = 0;
                match monkeys[n].operation {
                    '*' => worry_level = item * operand,
                    '+' => worry_level = item + operand,
                    _ => (),
                }

                worry_level = worry_level / 3;

                let val = worry_level % monkeys[n].divisible;

                let if_true = monkeys[n].if_true as usize;
                let if_false = monkeys[n].if_false as usize;
                match val == 0 {
                    true => monkeys[if_true].items.insert(0, worry_level),
                    false => monkeys[if_false].items.insert(0, worry_level),
                }
                monkeys[n].inspection_count += 1;
            }
        }
    }

    let mut inspect_counts: Vec<i64> = Vec::new();
    for m in monkeys.iter() {
        inspect_counts.push(m.inspection_count);
    }

    inspect_counts.sort();
    inspect_counts.reverse();

    inspect_counts[0] * inspect_counts[1]
}
