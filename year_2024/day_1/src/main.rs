use std::{
    fs,
    io::{Read, Result},
};

fn main() -> Result<()> {
    let mut input_file = fs::File::open("./src/input")?;
    let mut file_content: String = String::new();
    input_file.read_to_string(&mut file_content)?;

    // print!("file content : \n{file_content}");

    let lines = file_content.lines();

    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();

    for v in lines {
        let mut line = v.split("   ");

        let l = line.next().unwrap().parse::<i32>().unwrap();
        let r = line.next().unwrap().parse::<i32>().unwrap();

        left.push(l);
        right.push(r);
    }

    left.sort();
    right.sort();

    let mut sum: i32 = 0;

    for (i, v) in left.iter().enumerate() {
        let diff;
        if v > &right[i] {
            diff = v - &right[i];
        } else {
            diff = &right[i] - v;
        }
        sum += diff;
    }

    let mut sum2: i32 = 0;

    for v in left.iter() {
        let mut count: i32 = 0;
        for i in right.iter() {
            if v == i {
                count += 1;
            }
            if i > v {
                break;
            }
        }
        sum2 += v * count;
    }

    println!("sum : {sum}");
    println!("sum2 : {sum2}");

    Ok(())
}
