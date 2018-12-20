use chrono::{DateTime, TimeZone, Utc};
use std::fs::File;
use std::io;
use std::io::prelude::*;

#[derive(Debug)]
enum EntryType {
    NewShift(u32),
    Falls,
    Wakes,
}

#[derive(Debug)]
struct Entry {
    date: DateTime<Utc>,
    entry: EntryType,
}

fn parse_line(line: String) -> Entry {
    const FORMAT: &'static str = "%Y-%m-%d %H:%M";
    let date = Utc
        .datetime_from_str(&line[1..17], FORMAT)
        .expect("invalid date");

    let etype = match &line[19..24] {
        "Guard" => {
            let id_str = line
                .chars()
                .skip(26)
                .take_while(|&c| c != ' ')
                .collect::<String>();
            let id = id_str.parse::<u32>().expect("invalid id");
            EntryType::NewShift(id)
        }
        "falls" => EntryType::Falls,
        "wakes" => EntryType::Wakes,
        _ => unreachable!(),
    };
    Entry { date, entry: etype }
}

fn main() -> io::Result<()> {
    let f = File::open("input")?;
    let f = io::BufReader::new(f);

    let mut entries = f
        .lines()
        .map(|l| l.expect("invalid line"))
        .map(|l| parse_line(l))
        .collect::<Vec<Entry>>();
    entries.sort_by_key(|k| k.date);



    entries.iter().for_each(|e| println!("{:?}", e));

    Ok(())
}
