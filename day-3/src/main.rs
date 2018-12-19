use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io;
use std::io::prelude::*;

#[derive(Debug, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn transform(self) -> i32 {
        self.x * 1000 + self.y
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Rect {
    p1: Point,
    p2: Point,
}

impl Rect {
    fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
        Rect {
            p1: Point { x, y },
            p2: Point {
                x: x + width - 1,
                y: y + height - 1,
            },
        }
    }

    fn width(&self) -> i32 {
        self.p2.x + 1 - self.p1.x
    }

    fn height(&self) -> i32 {
        self.p2.y + 1 - self.p1.y
    }

    fn iter(&self) -> RectIter {
        let width = self.width();
        let idx_max = width * self.height();
        RectIter {
            point: self.p1.clone(),
            width,
            idx: 0,
            idx_max,
        }
    }

    fn intersection(&self, other: &Rect) -> Option<Rect> {
        if (self.p1.x > other.p2.x) || (other.p1.x > self.p2.x) {
            return None;
        }
        if (self.p1.y > other.p2.y) || (other.p1.y > self.p2.y) {
            return None;
        }

        let p1 = {
            let x = self.p1.x.max(other.p1.x);
            let y = self.p1.y.max(other.p1.y);
            Point { x, y }
        };

        let p2 = {
            let x = self.p2.x.min(other.p2.x);
            let y = self.p2.y.min(other.p2.y);
            Point { x, y }
        };

        Some(Rect { p1, p2 })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_intersection() {
        {
            // same rects
            let r1 = Rect::new(1, 1, 2, 2);
            let r2 = Rect::new(1, 1, 2, 2);
            let r3 = r1.clone();
            assert_eq!(r1.intersection(&r2), Some(r3));
        }
        {
            // non overlaping
            let r1 = Rect::new(1, 1, 2, 2);
            let r2 = Rect::new(5, 5, 2, 2);
            assert_eq!(r1.intersection(&r2), None);
            assert_eq!(r2.intersection(&r1), None);
        }
        {
            // adjacent
            let r1 = Rect::new(1, 1, 2, 2);
            let r2 = Rect::new(3, 1, 2, 2);
            assert_eq!(r1.intersection(&r2), None);
            assert_eq!(r2.intersection(&r1), None);
        }
        {
            // overlapping
            let r1 = Rect::new(1, 1, 5, 7);
            let r2 = Rect::new(3, 2, 4, 3);
            let r3 = Rect::new(3, 2, 3, 3);
            assert_eq!(r1.intersection(&r2), Some(r3.clone()));
            assert_eq!(r2.intersection(&r1), Some(r3.clone()));
        }
    }
}

struct RectIter {
    point: Point,
    width: i32,
    idx: i32,
    idx_max: i32,
}

impl Iterator for RectIter {
    type Item = Point;

    fn next(&mut self) -> Option<Point> {
        if self.idx == self.idx_max {
            None
        } else {
            let x = self.point.x + (self.idx % self.width);
            let y = self.point.y + (self.idx / self.width);
            self.idx += 1;
            Some(Point { x, y })
        }
    }
}

struct Claim {
    id: u32,
    rect: Rect,
}

fn parse_line(line: String) -> Claim {
    let re = regex::Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();
    let cap = re.captures(&line).unwrap();
    let id = cap.get(1).unwrap().as_str().parse::<u32>().unwrap();
    let x = cap.get(2).unwrap().as_str().parse::<i32>().unwrap();
    let y = cap.get(3).unwrap().as_str().parse::<i32>().unwrap();
    let width = cap.get(4).unwrap().as_str().parse::<i32>().unwrap();
    let height = cap.get(5).unwrap().as_str().parse::<i32>().unwrap();
    let rect = Rect::new(x, y, width, height);
    Claim { id, rect }
}

fn main() -> io::Result<()> {
    let f = File::open("input")?;
    let f = io::BufReader::new(f);

    let claims = f
        .lines()
        .map(|l| l.expect("invalid line"))
        .map(|l| parse_line(l))
        .collect::<Vec<Claim>>();

    let mut map = HashMap::new();
    claims
        .iter()
        .map(|r| r.rect.iter())
        .flatten()
        .for_each(|point| *map.entry(point.transform()).or_insert(0) += 1);

    println!("area: {}", map.values().filter(|v| **v > 1).count());

    let mut overlapped = HashSet::new();
    for (i, claim) in claims.iter().enumerate() {
        claims
            .iter()
            .skip(i + 1)
            .filter(|r| r.rect.intersection(&claim.rect).is_some())
            .for_each(|c| {
                overlapped.insert(c.id);
                overlapped.insert(claim.id);
            });
    }

    let not_overlapped = claims
        .iter()
        .filter(|c| !overlapped.contains(&c.id))
        .map(|c| c.id)
        .collect::<Vec<u32>>();

    println!("not overlapped claims: {:?}", not_overlapped);

    Ok(())
}

