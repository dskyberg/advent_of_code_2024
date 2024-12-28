use anyhow::Result;
use std::time::Instant;

use aoc_utils::*;
use day_14::*;

/// Determine which quadrant this position is in
fn quad(pos: &Point, max: &Point) -> Option<usize> {
    let middle_row = max.y / 2;
    let middle_col = max.x / 2;
    // If the robot is in the exact middle of the hall don't count
    if pos.y == middle_row || pos.x == middle_col {
        return None;
    }

    if pos.y > middle_row {
        if pos.x > middle_col { Some(3) } else { Some(2) }
    } else if pos.x > middle_col {
        Some(1)
    } else {
        Some(0)
    }
}

fn main() -> Result<()> {
    let now = Instant::now();
    let mut lobby = Lobby::new(DATA, 103, 101)?;

    for _ in 0..100 {
        lobby.second();
    }

    // Calc quadrants
    let mut quads: [usize; 4] = [0, 0, 0, 0];

    lobby.robots.iter().for_each(|robot| {
        if let Some(q) = quad(&robot.pos, &lobby.max) {
            quads[q] += 1;
        }
    });

    let result: usize = quads.iter().product();
    println!("Part1: {} - {:.2?}", result, now.elapsed());

    Ok(())
}
