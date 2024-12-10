use anyhow::Result;
use std::{collections::HashSet, time::Instant};

use day_6::*;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Position {
    col: usize,
    row: usize,
}

impl From<(usize, usize)> for Position {
    fn from((row, col): (usize, usize)) -> Self {
        Position { row, col }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Default)]
enum Direction {
    #[default]
    North,
    South,
    East,
    West,
}

impl Direction {
    pub fn turn(&self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::South => Direction::West,
            Direction::East => Direction::South,
            Direction::West => Direction::North,
        }
    }
}
#[derive(Debug, Clone)]
struct Map {
    max_row: usize,
    max_col: usize,
    matrix: Vec<Vec<u8>>,
    visited: HashSet<Position>,
    curr_pos: Position,
    curr_dir: Direction,
}
impl Map {
    fn new(data: &str) -> Self {
        let (matrix, pos) = read_data(data);
        let mut visited = HashSet::new();
        let start_pos = Position::from(pos);
        let curr_pos = start_pos.clone();
        visited.insert(curr_pos.clone());

        Map {
            max_row: matrix.len() - 1,
            max_col: matrix[0].len() - 1,
            matrix,
            visited,
            curr_pos,
            curr_dir: Direction::default(),
        }
    }

    #[inline]
    fn turn(&mut self) {
        self.curr_dir = self.curr_dir.turn();
    }

    #[inline]
    fn on_edge(&self) -> bool {
        self.curr_pos.row == 0
            || self.curr_pos.row == self.max_row
            || self.curr_pos.col == 0
            || self.curr_pos.col == self.max_col
    }

    fn try_step(&self) -> Option<Position> {
        match (&self.curr_dir, (self.curr_pos.row, self.curr_pos.col)) {
            (Direction::North, (row, col)) => {
                if row > 0 {
                    Some(Position { row: row - 1, col })
                } else {
                    None
                }
            }
            (Direction::South, (row, col)) => {
                if row < self.max_row {
                    Some(Position { col, row: row + 1 })
                } else {
                    None
                }
            }
            (Direction::East, (row, col)) => {
                if col < self.max_col {
                    Some(Position { row, col: col + 1 })
                } else {
                    None
                }
            }
            (Direction::West, (row, col)) => {
                if col > 0 {
                    Some(Position { row, col: col - 1 })
                } else {
                    None
                }
            }
        }
    }

    #[inline]
    fn get(&self, pos: &Position) -> u8 {
        self.matrix[pos.row][pos.col]
    }

    /// Returns true if there are more steps.
    /// Returns false when an edge has been reached.
    fn step(&mut self) -> bool {
        // If this fails, it's because we're on an edge, and missed it.
        let Some(pos) = self.try_step() else {
            panic!("Trying to step when already on an edge!");
        };

        self.visited.insert(pos.clone());
        self.curr_pos = pos;

        // Is this the end?
        if self.on_edge() {
            return false;
        }

        // See if we need to turn
        let Some(pos) = self.try_step() else {
            panic!("Trying to step when already on an edge!");
        };
        if self.get(&pos) == BLOCKED {
            self.turn()
        }
        true
    }
    fn run_simulation(&mut self) -> usize {
        while self.step() {}
        self.visited.len()
    }
}

fn main() -> Result<()> {
    let now = Instant::now();
    let mut map = Map::new(DATA);
    let result = map.run_simulation();
    println!("Part1: {} - {:.2?}", result, now.elapsed());
    Ok(())
}
