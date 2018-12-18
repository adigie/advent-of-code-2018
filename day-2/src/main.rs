use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::prelude::*;

fn check_id(id: &String) -> (bool, bool) {
    let mut m = HashMap::new();
    id.chars().for_each(|c| *m.entry(c).or_insert(0) += 1);
    let two = m.values().any(|&v| v == 2);
    let three = m.values().any(|&v| v == 3);
    (two, three)
}

fn compare_ids(first: &String, second: &String) -> bool {
    let sum = first
        .chars()
        .zip(second.chars())
        .filter(|(a, b)| a != b)
        .count();
    sum == 1
}

fn get_common(first: &String, second: &String) -> String {
    first
        .chars()
        .zip(second.chars())
        .filter_map(|(a, b)| return if a == b { Some(a) } else { None })
        .collect()
}

fn main() -> io::Result<()> {
    let f = File::open("input")?;
    let f = io::BufReader::new(f);

    let mut two = 0;
    let mut three = 0;

    let lines = f
        .lines()
        .map(|l| l.expect("invalid line"))
        .collect::<Vec<String>>();

    for line in lines.as_slice() {
        match check_id(&line) {
            (true, true) => {
                two += 1;
                three += 1;
            }
            (true, false) => two += 1,
            (false, true) => three += 1,
            _ => (),
        }
    }

    println!("checksum: {}", two * three);

    for i in 1..lines.len() {
        for j in 0..(i - 1) {
            if compare_ids(&lines[i], &lines[j]) {
                println!("{}", get_common(&lines[i], &lines[j]));
                return Ok(());
            }
        }
    }

    Ok(())
}
