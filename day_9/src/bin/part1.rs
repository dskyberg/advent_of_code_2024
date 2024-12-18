use anyhow::Result;
use std::time::Instant;

use day_9::*;

fn sort_blocks(blocks: &[Option<usize>]) -> Vec<Option<usize>> {
    let mut result = Vec::new();
    let mut front = 0;
    let mut back = blocks.len() - 1;

    while front <= back {
        match blocks[front] {
            Some(block) => {
                result.push(Some(block));
            }
            None => {
                while blocks[back].is_none() {
                    back -= 1;
                }
                result.push(blocks[back]);
                back -= 1;
            }
        }
        front += 1;
    }

    result
}

fn main() -> Result<()> {
    let now = Instant::now();
    let disk_map = read_data(DATA);
    let blocks = decode(&disk_map);
    let blocks = sort_blocks(&blocks);
    let result = calc_check_sum(&blocks);
    println!("Part1: {} - {:.2?}", result, now.elapsed());
    Ok(())
}
