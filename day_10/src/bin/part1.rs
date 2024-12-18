use anyhow::Result;
use std::collections::HashSet;
use std::time::Instant;
use thiserror::Error;

use aoc_utils::{DAG, Point, data_to_digits};

use day_10::*;

const TRAIL_HEAD_HEIGHT: u8 = 0;
const TRAIL_MAX_HEIGHT: u8 = 9;

#[derive(Debug, Error)]
enum Day10Error {
    #[error("DAG is incomplete")]
    IncompleteDAG,
}

fn walk_path(stop: u8, from: &Point, max: u8, maxes: &mut HashSet<Point>, dag: &DAG) -> Result<()> {
    for node in (*dag).get(&stop).ok_or(Day10Error::IncompleteDAG)? {
        if node.touches(from, false) {
            if stop == max {
                maxes.insert(*node);
            } else {
                walk_path(stop + 1, node, max, maxes, dag)?;
            }
        }
    }
    Ok(())
}

/// Count how many different maxes can be reached from the trail head.
/// This would be safer if we didn't assum the min (trail head) and max
fn count_trails(dag: &DAG) -> Result<usize> {
    let mut result = 0;

    // Get all the zeros
    let trail_heads = (*dag).get(&TRAIL_HEAD_HEIGHT).unwrap();

    for trail_head in trail_heads {
        let mut maxes = HashSet::new();
        walk_path(1, trail_head, TRAIL_MAX_HEIGHT, &mut maxes, dag)?;
        result += maxes.len();
    }
    Ok(result)
}

fn main() -> Result<()> {
    let now = Instant::now();
    let matrix = data_to_digits::<u8>(DATA)?;
    let dag = DAG::from(&matrix);

    let result = count_trails(&dag)?;

    println!("Part1: {} - {:.2?}", result, now.elapsed());
    Ok(())
}
