/// Solve Part 1 using path finding.
/// This will use a standard dijkstra style alg.  This can't be used for part2,
/// due to the size of the search space.
use anyhow::Result;
use pathfinding::prelude::dijkstra;
use regex::Regex;
use std::time::Instant;

use aoc_utils::*;
use day_13::*;

fn parse_button(line: &str) -> (String, Point) {
    let re = Regex::new(r"Button (?<button>[A-B]): X\+(?<col>\d+), Y\+(?<row>\d+)").unwrap();
    let caps = re
        .captures(line)
        .expect(&format!("Failed to parse {}", line));
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
            prize,
        }
    }
}
fn main() -> Result<()> {
    let now = Instant::now();
    let machines: Vec<Machine> = DATA.split("\n\n").map(Machine::from).collect();

    let result: isize = machines
        .iter()
        .flat_map(|machine| {
            let start_node = Point::from((0, 0));
            let result = dijkstra(
                &start_node,
                |pos| {
                    if pos.col > machine.prize.col || pos.row > machine.prize.row {
                        vec![]
                    } else {
                        vec![(*pos + machine.a, A_COST), (*pos + machine.b, B_COST)]
                    }
                },
                |&p| p == machine.prize,
            );

            result.map(|(_, cost)| cost)
        })
        .sum();

    println!(
        r#"{{"part":1, "answer": "{}", "time": "{:.2?}"}}"#,
        result,
        now.elapsed()
    );
    Ok(())
}
