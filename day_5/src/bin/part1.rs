use anyhow::Result;
use std::collections::HashMap;
use std::time::Instant;

use day_5::*;

fn is_correct_order(rules: &[(u32, u32)], updates: &[u32]) -> bool {
    let map: HashMap<u32, usize> = updates
        .iter()
        .enumerate()
        .map(|(idx, &page)| (page, idx))
        .collect();

    rules.iter().all(|(x, y)| match (map.get(x), map.get(y)) {
        (Some(x), Some(y)) => x < y,
        _ => true,
    })
}
fn main() -> Result<()> {
    let mut result = 0;
    let rules = read_rules(RULES)?;
    let updates = read_updates(UPDATES)?;

    let now = Instant::now();
    for update in updates {
        if is_correct_order(&rules, &update) {
            result += update[update.len() / 2];
        }
    }
    println!("Part 1: {} - {:.2?}", result, now.elapsed());
    Ok(())
}
