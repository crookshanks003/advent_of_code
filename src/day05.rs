use anyhow;
use std::{collections::HashMap, fs, ops::RangeInclusive, str::FromStr};

#[derive(Eq, Hash, PartialEq, Copy, Clone)]
struct Position {
    x: u32,
    y: u32,
}

impl FromStr for Position {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.split_once(",").unwrap();
        Ok(Position {
            x: x.parse()?,
            y: y.parse()?,
        })
    }
}

struct Line {
    start: Position,
    end: Position,
}

impl FromStr for Line {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start, end) = s.split_once(" -> ").unwrap();

        Ok(Line {
            start: start.parse()?,
            end: end.parse()?,
        })
    }
}

fn get_range(num1: u32, num2: u32) -> Box<dyn Iterator<Item = u32>> {
    if num1 > num2 {
        Box::new(RangeInclusive::new(num2, num1))
    } else {
        Box::new(RangeInclusive::new(num1, num2).rev())
    }
}

fn get_points_on_line(ln: &Line) -> Vec<Position> {
    let mut result: Vec<Position> = Vec::new();
    let slope: f32 = if ln.start.x == ln.end.x {
        2.0
    } else {
        ((ln.start.y) as f32 - (ln.end.y) as f32).abs()
            / ((ln.start.x) as f32 - (ln.end.x) as f32).abs()
    };
    if slope == 2.0 || slope == 0.0 {
        for x in get_range(ln.start.x, ln.end.x) {
            for y in get_range(ln.start.y, ln.end.y) {
                result.push(Position { x, y });
            }
        }
    } else if slope == 1.0 {
        let rangex: Vec<u32> = get_range(ln.start.x, ln.end.x).collect();

        for (i, y) in get_range(ln.start.y, ln.end.y).enumerate() {
            result.push(Position { x: rangex[i], y });
        }
    }
    result
}

fn get_positions(lines: &Vec<Line>) -> u32 {
    let mut count: HashMap<Position, u32> = HashMap::new();

    for l in lines {
        for pos in get_points_on_line(&l) {
            // println!("{}, {}", pos.x, pos.y);
            count.entry(pos).or_insert(0);
            count.insert(pos, 1 + count[&pos]);
        }
    }
    let mut count_pos = 0;
    for (k, v) in count {
        if v >= 2 {
            println!("{}, {} -- {}", k.x, k.y, v);
            count_pos += 1;
        }
    }
    count_pos
}

pub fn get_result() {
    let inp = fs::read_to_string("src/input/day5.input").expect("Cant read the file");
    let lines: Vec<Line> = inp
        .split("\n")
        .filter(|l| l.len() > 0)
        .map(|l| l.parse().unwrap())
        .collect();

    println!("{}", get_positions(&lines));
}
