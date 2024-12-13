use std::fmt::Display;
use std::ops::{Add, Sub};

#[derive(Debug, PartialEq, Clone, Copy, Hash, Eq, Default)]
pub struct Point {
    pub row: isize,
    pub col: isize,
}

impl Point {
    pub fn mul(&self, factor: usize) -> Self {
        Self {
            row: self.row * factor as isize,
            col: self.col * factor as isize,
        }
    }
}

impl From<(usize, usize)> for Point {
    fn from(value: (usize, usize)) -> Self {
        Self {
            row: value.0 as isize,
            col: value.1 as isize,
        }
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{},{}]", self.row, self.col)
    }
}

impl Add for Point {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            row: self.row + other.row,
            col: self.col + other.col,
        }
    }
}

impl Sub for Point {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            row: self.row - other.row,
            col: self.col - other.col,
        }
    }
}
