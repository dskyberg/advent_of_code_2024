use anyhow::Result;
use aoc_utils::*;
use day_12::*;
use std::collections::{HashMap, HashSet};
use std::time::Instant;

#[inline]
fn is_corner(
    plant: Option<char>,
    first_adjacent: Option<char>,
    second_adjacent: Option<char>,
    diagonal: Option<char>,
) -> bool {
    (first_adjacent != plant && second_adjacent != plant)
        || (first_adjacent == plant && second_adjacent == plant && diagonal != plant)
}

#[derive(Debug)]
struct Garden {
    pub plots: HashMap<Point, char>,
    pub regions: Vec<Region>,
}

impl Garden {
    fn cost(&self) -> usize {
        self.regions
            .iter()
            .map(|region| {
                // The number of sides is equivelent to the number of corners
                let sides = region
                    .region
                    .iter()
                    .map(|plot| self.corners(plot))
                    .sum::<usize>();
                region.area() * sides
            })
            .sum::<usize>()
    }

    fn get(&self, plot: &Point) -> Option<char> {
        self.plots.get(plot).cloned()
    }

    // Find the number of corners that this plot contributes to the shape of the
    // region.
    fn corners(&self, plot: &Point) -> usize {
        let mut count = 0;
        let this = self.get(plot);

        let n = self.get(&plot.north());
        let ne = self.get(&plot.north_east());
        let e = self.get(&plot.east());
        let se = self.get(&plot.south_east());
        let s = self.get(&plot.south());
        let sw = self.get(&plot.south_west());
        let w = self.get(&plot.west());
        let nw = self.get(&plot.north_west());
        count += is_corner(this, n, e, ne) as usize;
        count += is_corner(this, s, e, se) as usize;
        count += is_corner(this, s, w, sw) as usize;
        count += is_corner(this, n, w, nw) as usize;

        count
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
