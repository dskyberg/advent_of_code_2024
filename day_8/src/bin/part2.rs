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
            antinodes.insert(*a1);
            for a2 in locs[i + 1..].iter() {
                antinodes.insert(*a2);
                let distance = *a1 - *a2;
                let mut factor = 0;
                loop {
                    factor += 1;
                    let p = *a1 + distance.mul(factor);
                    if board.valid_point(&p) {
                        antinodes.insert(p);
                    } else {
                        break;
                    }
                }
                factor = 0;
                loop {
                    factor += 1;
                    let p = *a2 - distance.mul(factor);
                    if board.valid_point(&p) {
                        antinodes.insert(p);
                    } else {
                        break;
                    }
                }
            }
        }
    }
    println!("Part2: {} - {:.2?}", antinodes.len(), now.elapsed());
    Ok(())
}
