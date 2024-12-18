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

    #[inline]
    /// Absolute distance between rows
    pub fn vertical_distance(&self, other: &Self) -> isize {
        self.row.max(other.row) - self.row.min(other.row)
    }

    #[inline]
    /// Absolute distance between cols
    pub fn horizontal_distance(&self, other: &Self) -> isize {
        self.col.max(other.col) - self.col.min(other.col)
    }

    /// Measures the horizontal and vertical distance between two points
    pub fn distance(&self, other: &Self) -> Self {
        Self {
            col: self.horizontal_distance(other),
            row: self.vertical_distance(other),
        }
    }

    /// True if the points touch horizontally or vertically.
    /// if `check_diag` then checks to see if they touch on the corners
    pub fn touches(&self, other: &Point, check_diag: bool) -> bool {
        let distance = self.distance(other);
        match check_diag {
            true => distance.row == 1 && distance.col == 1,
            false => {
                distance.row == 0 && distance.col == 1 || distance.row == 1 && distance.col == 0
            }
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
