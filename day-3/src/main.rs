use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;

#[derive(Debug, Copy, Clone)]
struct Rect {
    x: u32,
    y: u32,
    width: u32,
    height: u32,
}

struct RectIter {
    rect: Rect,
    idx: u32,
    idx_max: u32,
}

#[derive(Debug)]
struct Point(u32, u32);

impl Point {
    fn transform(self) -> u32 {
        self.0 * 1000 + self.1
    }
}

impl IntoIterator for Rect {
    type Item = Point;
    type IntoIter = RectIter;

    fn into_iter(self) -> RectIter {
        let idx_max = self.width * self.height;
        RectIter {
            rect: self,
            idx: 0,
            idx_max,
        }
    }
}

impl Iterator for RectIter {
    type Item = Point;

    fn next(&mut self) -> Option<Point> {
        if self.idx == self.idx_max {
            None
        } else {
            let x = self.rect.x + (self.idx % self.rect.width);
            let y = self.rect.y + (self.idx / self.rect.width);
            self.idx += 1;
            Some(Point(x, y))
        }
    }
}

fn parse_line(line: String) -> Rect {
    let re = regex::Regex::new(r"#\d+ @ (\d+),(\d+): (\d+)x(\d+)").unwrap();
    let cap = re.captures(&line).unwrap();
    let x = cap.get(1).unwrap().as_str().parse::<u32>().unwrap();
    let y = cap.get(2).unwrap().as_str().parse::<u32>().unwrap();
    let width = cap.get(3).unwrap().as_str().parse::<u32>().unwrap();
    let height = cap.get(4).unwrap().as_str().parse::<u32>().unwrap();
    Rect {
        x,
        y,
        width,
        height,
    }
}

fn main() -> io::Result<()> {
    let f = File::open("input")?;
    let f = io::BufReader::new(f);

    let rects = f.lines().map(|l| l.expect("invalid line")).map(|l| parse_line(l)).collect::<Vec<Rect>>();

    let mut set = HashMap::new();
    rects.iter().map(|r| r.into_iter()).flatten().for_each(|point| *set.entry(point.transform()).or_insert(0) += 1 );

    println!("{}", set.values().filter(|v| **v > 1).count());

    Ok(())
}

