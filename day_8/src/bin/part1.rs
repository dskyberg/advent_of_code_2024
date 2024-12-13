use anyhow::Result;
use std::collections::HashSet;
use std::time::Instant;

use day_8::*;

fn main() -> Result<()> {
    let board = Board::from(DATA);
    let now = Instant::now();

    let mut antinodes = HashSet::<Point>::new();

    for locs in board.map.values() {
        // Every pair of antennae are in a straight line.  So, we just need to
        // determine if the distance between 2 antennae can be extended onto the board
        for (i, a1) in locs.iter().enumerate() {
            for a2 in locs[i + 1..].iter() {
                let distance = *a1 - *a2;

                let p1 = *a1 + distance;
                if board.valid_point(&p1) {
                    antinodes.insert(p1);
                }

                let p2 = *a2 - distance;
                if board.valid_point(&p2) {
                    antinodes.insert(p2);
                }
            }
        }
    }

    println!("Part1: {} - {:.2?}", antinodes.len(), now.elapsed());
    Ok(())
}
