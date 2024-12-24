/// Solve using systems of linear equations.
///
use anyhow::Result;
use regex::Regex;
use std::time::Instant;

use aoc_utils::*;
use day_13::*;

fn parse_button(line: &str) -> (String, Point) {
    let re = Regex::new(r"Button (?<button>[A-B]): X\+(?<col>\d+), Y\+(?<row>\d+)").unwrap();
    let caps = re
        .captures(line)
        .unwrap_or_else(|| panic!("Failed to parse {}", line));
    let button = caps["button"].to_owned();
    let col = caps["col"].parse::<isize>().unwrap();
    let row = caps["row"].parse::<isize>().unwrap();

    (button, Point { row, col })
}

fn parse_prize(line: &str) -> Point {
    let re = Regex::new(r"Prize: X=(?<col>\d+), Y=(?<row>\d+)").unwrap();
    let caps = re.captures(line).unwrap();
    let col = caps["col"].parse::<isize>().unwrap();
    let row = caps["row"].parse::<isize>().unwrap();
    Point { row, col }
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
        writeln!(f, "Button A: X+{}, Y+{}", self.a.col, self.a.row)?;
        writeln!(f, "Button B: X+{}, Y+{}", self.b.col, self.b.row)?;
        writeln!(f, "Prize: X={}, Y={}", self.prize.col, self.prize.row)
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
                row: prize.row + 10_000_000_000_000,
                col: prize.col + 10_000_000_000_000,
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
