use anyhow::Result;
use std::time::Instant;

use day_14::*;

fn find_cluster(lobby: &mut Lobby) -> usize {
    let mut hline_start = None;
    let mut vline_start = None;
    let mut result = 0;

    loop {
        result += 1;
        for robot in lobby.robots.iter_mut() {
            robot.pos += robot.velocity;
            robot.pos = robot.pos.rem_euclid(lobby.max);
        }

        // Check for unusually many 'busy' horizontal lines
        if hline_start.is_none() {
            let mut hline_counts: Vec<usize> = vec![0; lobby.max.y as usize];
            for robot in lobby.robots.iter() {
                hline_counts[robot.pos.y as usize] += 1;
            }

            if hline_counts.iter().filter(|v| **v > 20).count() > 3 {
                hline_start = Some(result);
            }
        }
        // Check for unusually many 'busy' vertical lines
        if vline_start.is_none() {
            let mut vline_counts: Vec<usize> = vec![0; lobby.max.x as usize];
            for robot in lobby.robots.iter() {
                vline_counts[robot.pos.x as usize] += 1;
            }

            if vline_counts.iter().filter(|v| **v > 20).count() > 3 {
                vline_start = Some(result);
            }
        }
        // If we have both, we have an answer
        // I'm still not sure why the cycles can be off by ±1
        if hline_start.is_some() && vline_start.is_some() {
            // Solve using the Chinese remainder theorem
            let h_offset = hline_start.unwrap() % lobby.max.y as usize;
            let v_offset = vline_start.unwrap() % lobby.max.x as usize;

            let mut h_timer = h_offset;
            let mut v_timer = v_offset;

            loop {
                if h_timer == v_timer {
                    return h_timer;
                }

                if h_timer < v_timer {
                    h_timer += lobby.max.y as usize;
                } else {
                    v_timer += lobby.max.x as usize;
                }
            }
        }
        if result > 10_000 {
            panic!("Reached 10,000!");
        }
    }
}

fn main() -> Result<()> {
    let now = Instant::now();
    let mut lobby = Lobby::new(DATA, 103, 101)?;

    let result = find_cluster(&mut lobby);

    println!("Part2: {} - {:.2?}", result, now.elapsed());
    Ok(())
}
