use anyhow::Result;
use aoc_utils::*;
use day_12::*;
use std::collections::{HashMap, HashSet};
use std::time::Instant;

#[derive(Debug)]
struct Garden {
    pub plots: HashMap<Point, char>,
    pub regions: Vec<Region>,
}

impl Garden {
    fn cost(&self) -> usize {
        self.regions
            .iter()
            .fold(0, |acc, r| acc + r.area() * r.perimeter())
    }
}

impl From<&str> for Garden {
    fn from(data: &str) -> Self {
        let plots: HashMap<Point, char> = data
            .lines()
            .enumerate()
            .flat_map(|(row, line)| {
                line.chars()
                    .enumerate()
                    .map(move |(col, plant)| (Point::from((row, col)), plant))
            })
            .collect();

        // Find all the regions of plants in the garden
        let mut regions: Vec<Region> = Vec::new();
        let mut visited = HashSet::<Point>::new();
        for (plot, &plant) in &plots {
            if visited.contains(plot) {
                continue;
            }
            let mut region = HashSet::new();
            bfs_region(&plots, plot, plant, &mut region);
            visited.extend(&region);
            regions.push(Region {
                plant,
                region: region.into_iter().collect(),
            });
        }

        Garden { plots, regions }
    }
}

impl std::fmt::Display for Garden {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let size = (self.plots.len() as f32).sqrt() as usize;
        for row in 0..size {
            for col in 0..size {
                let plot = Point::from((row, col));
                let plant = self.plots[&plot];
                write!(f, "{}", plant)?;
            }
            writeln!(f)?;
        }
        writeln!(f)?;

        for region in &self.regions {
            writeln!(f, "{}", region)?;
        }
        write!(f, "")
    }
}

fn main() -> Result<()> {
    let now = Instant::now();
    let garden = Garden::from(DATA);
    let result = garden.cost();
    println!("Part1: {} - {:.2?}", result, now.elapsed());
    Ok(())
}
