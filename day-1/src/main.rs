use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::collections::HashSet;

#[derive(Debug)]
enum Error {
    Io(std::io::Error),
    Parse(std::num::ParseIntError),
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Error {
        Error::Io(err)
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(err: std::num::ParseIntError) -> Error {
        Error::Parse(err)
    }
}

type Result<T> = std::result::Result<T, Error>;

fn parse_line(line: String) -> Result<i32> {
    match line.parse::<i32>() {
        Ok(val) => Ok(val),
        Err(e) => Err(Error::Parse(e)),
    }
}

fn find_duplicate_frequency(freq_list: &Vec<i32>) -> i32 {
    let mut current_freq : i32 = 0;
    let mut freq_set = HashSet::new();
    freq_set.insert(current_freq);
    loop {
        for f in freq_list.iter() {
            current_freq += f;
            if !freq_set.insert(current_freq) {
                return current_freq;
            }
        }
    }
}

fn main() -> Result<()> {
    let f = File::open("input")?;
    let f = io::BufReader::new(f);

    let mut freq_list = Vec::new();
    for line in f.lines() {
        match line {
            Ok(line) => freq_list.push(parse_line(line)?),
            Err(e) => return Err(e.into()),
        }
    }

    println!("frequency: {}", freq_list.iter().sum::<i32>());
    println!("duplicate: {}", find_duplicate_frequency(&freq_list));

    Ok(())
}
