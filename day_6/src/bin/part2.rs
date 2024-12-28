use anyhow::Result;
use std::{collections::HashSet, time::Instant};
use thiserror::Error;

use day_6::*;

#[derive(Debug, Error)]
enum AppError {
    #[error("In a time loop")]
    TimeLoop,
}

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
    visited: HashSet<(Position, Direction)>,
    start_pos: Position,
    last_changed: Option<Position>,
    curr_pos: Position,
    curr_dir: Direction,
}
impl Map {
    fn new(data: &str) -> Self {
        let (matrix, pos) = read_data(data);
        let mut visited = HashSet::new();
        let start_pos = Position::from(pos);
        let curr_pos = start_pos.clone();
        let last_changed = None;
        visited.insert((curr_pos.clone(), Direction::North));

        Map {
            max_row: matrix.len() - 1,
            max_col: matrix[0].len() - 1,
            matrix,
            visited,
            start_pos,
            curr_pos,
            last_changed,
            curr_dir: Direction::default(),
        }
    }

    fn reset(&mut self) {
        self.visited.clear();
        self.curr_pos = self.start_pos.clone();
        self.curr_dir = Direction::default();
        if let Some(pos) = &self.last_changed {
            self.matrix[pos.row][pos.col] = OPEN;
        }
        self.last_changed = None;
    }

    #[inline]
    fn turn(&mut self) {
        self.curr_dir = self.curr_dir.turn();
    }

    #[inline]
    /// Returns true of the current position is an edge.
    fn on_edge(&self) -> bool {
        self.curr_pos.row == 0
            || self.curr_pos.row == self.max_row
            || self.curr_pos.col == 0
            || self.curr_pos.col == self.max_col
    }

    #[inline]
    /// Gets the character at the matrix pos.
    fn get(&self, pos: &Position) -> u8 {
        self.matrix[pos.row][pos.col]
    }
    #[inline]
    /// Sets the matrix position to c.
    fn set(&mut self, pos: &Position, c: u8) {
        self.matrix[pos.row][pos.col] = c;
    }

    /// Adds a block, `'#'`, at the given position.
    fn add_block(&mut self, pos: &Position) -> bool {
        if *pos == self.start_pos {
            return false;
        }
        if self.get(pos) == OPEN {
            self.set(pos, BLOCKED);
            self.last_changed = Some(pos.clone());
            return true;
        }
        false
    }

    /// Returns the next position if moving forward is valid.  Else None.
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

    /// Returns true if there are more steps.
    /// Returns false when an edge has been reached.
    /// Returns an error if it looks like we're in a loop.
    fn step(&mut self) -> Result<bool> {
        // If this fails, it's because we're on an edge, and missed it.
        let Some(pos) = self.try_step() else {
            panic!("Trying to step when already on an edge!");
        };

        if !self.visited.insert((pos.clone(), self.curr_dir.clone())) {
            // Looks like we're in a time loop!
            return Err(AppError::TimeLoop.into());
        }

        self.curr_pos = pos;

        // Is this the end?
        if self.on_edge() {
            return Ok(false);
        }

        // See if we need to turn
        let Some(pos) = self.try_step() else {
            panic!("Trying to step when already on an edge!");
        };

        if self.get(&pos) == BLOCKED {
            self.turn();
            // Recheck if this is now a loop.  So we can bail early.
            if self
                .visited
                .contains(&(self.curr_pos.clone(), self.curr_dir.clone()))
            {
                // oops!  Looks like we enterd a time loop!
                return Err(AppError::TimeLoop.into());
            }
        }
        Ok(true)
    }

    fn is_a_loop(&mut self) -> bool {
        loop {
            let result = self.step();
            if result.is_err() {
                return true;
            }
            if !result.unwrap() {
                return false;
            }
        }
    }
}

fn main() -> Result<()> {
    let mut result = 0;
    let now = Instant::now();
    let mut map = Map::new(TEST_DATA);
    for row in 0..=map.max_row {
        for col in 0..=map.max_col {
            let pos = Position::from((row, col));
            if map.add_block(&pos) {
                if map.is_a_loop() {
                    result += 1;
                }
                map.reset();
            }
        }
    }

    // 1952 is too high
    // 1955 is wrong
    println!("Part1: {} - {:.2?}", result, now.elapsed());
    Ok(())
}
