/// Solve using systems of linear equations.
///
use anyhow::Result;
use regex::Regex;
use std::time::Instant;

use aoc_utils::*;
use day_13::*;

fn parse_button(line: &str) -> (String, Point) {
    let re = Regex::new(r"Button (?<button>[A-B]): X\+(?<x>\d+), Y\+(?<y>\d+)").unwrap();
    let caps = re
        .captures(line)
        .unwrap_or_else(|| panic!("Failed to parse {}", line));
    let button = caps["button"].to_owned();
    let x = caps["x"].parse::<isize>().unwrap();
    let y = caps["y"].parse::<isize>().unwrap();

    (button, Point { y, x })
}

fn parse_prize(line: &str) -> Point {
    let re = Regex::new(r"Prize: X=(?<x>\d+), Y=(?<y>\d+)").unwrap();
    let caps = re.captures(line).unwrap();
    let x = caps["x"].parse::<isize>().unwrap();
    let y = caps["y"].parse::<isize>().unwrap();
    Point { y, x }
}

#[derive(Debug)]
struct Machine {
    pub a: Point,
    pub b: Point,
    pub prize: Point,
}

impl Machine {
    fn solve(&self) -> Option<isize> {
        solve(self.a, self.b, self.prize)
    }
}

impl std::fmt::Display for Machine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Button A: X+{}, Y+{}", self.a.x, self.a.y)?;
        writeln!(f, "Button B: X+{}, Y+{}", self.b.x, self.b.y)?;
        writeln!(f, "Prize: X={}, Y={}", self.prize.x, self.prize.y)
    }
}

impl From<&str> for Machine {
    fn from(value: &str) -> Self {
        let mut lines = value.trim().lines();
        let a = parse_button(lines.next().unwrap());
        let b = parse_button(lines.next().unwrap());
        let prize = parse_prize(lines.next().unwrap());
        Self {
            a: a.1,
            b: b.1,
            prize: Point {
                y: prize.y + 10_000_000_000_000,
                x: prize.x + 10_000_000_000_000,
            },
        }
    }
}

fn main() -> Result<()> {
    let now = Instant::now();
    let machines: Vec<Machine> = DATA.split("\n\n").map(Machine::from).collect();
    let result: isize = machines.iter().flat_map(|m| m.solve()).sum();
    println!("Part2: {} - {:.2?}", result, now.elapsed());
    Ok(())
}
