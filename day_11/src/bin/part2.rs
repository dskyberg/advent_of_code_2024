use anyhow::Result;
use std::collections::HashMap;
use std::time::Instant;

use day_11::*;

use aoc_utils::*;

fn get_num_stones(
    memo: &mut HashMap<(usize, usize), usize>,
    stone: usize,
    remaining_blinks: usize,
) -> Result<usize> {
    if remaining_blinks == 0 {
        return Ok(1);
    }
    if let Some(&result) = memo.get(&(stone, remaining_blinks)) {
        return Ok(result);
    }
    if stone == 0 {
        let result = get_num_stones(memo, 1, remaining_blinks - 1)?;
        memo.insert((stone, remaining_blinks), result);
        return Ok(result);
    }
    let val = stone.to_string();
    if val.len() % 2 == 1 {
        let result = get_num_stones(memo, stone * 2024, remaining_blinks - 1)?;
        memo.insert((stone, remaining_blinks), result);
        return Ok(result);
    }
    let half = val.len() / 2;
    let left = val[..half].parse::<usize>()?;
    let right = val[half..].parse::<usize>()?;
    let left_result = get_num_stones(memo, left, remaining_blinks - 1)?;
    let right_result = get_num_stones(memo, right, remaining_blinks - 1)?;
    Ok(left_result + right_result)
}

fn main() -> Result<()> {
    let now = Instant::now();
    let mut result = 0;
    let stones = line_to_numbers::<usize>(DATA, ' ')?;
    let mut memo = HashMap::new();
    for stone in stones {
        result += get_num_stones(&mut memo, stone, 75)?;
    }

    println!("Part1: {} - {:.2?}", result, now.elapsed());
    Ok(())
}
