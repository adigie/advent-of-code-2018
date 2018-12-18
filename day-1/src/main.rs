use std::fs::File;
use std::io;
use std::io::prelude::*;

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

fn main() -> Result<()> {
    let f = File::open("input")?;
    let f = io::BufReader::new(f);

    let mut frequency: i32 = 0;

    for line in f.lines() {
        match line {
            Ok(line) => frequency += parse_line(line)?,
            Err(e) => return Err(e.into()),
        }
    }

    println!("frequency: {}", frequency);

    Ok(())
}
