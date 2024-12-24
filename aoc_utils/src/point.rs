use std::fmt::Display;
use std::ops::{Add, Sub};

static NEIGHBORS_HV: [Point; 4] = [
    Point { row: -1, col: 0 },
    Point { row: 0, col: -1 },
    Point { row: 0, col: 1 },
    Point { row: 1, col: 0 },
];

static NEIGHBORS: [Point; 8] = [
    Point { row: -1, col: -1 },
    Point { row: -1, col: 0 },
    Point { row: -1, col: 1 },
    Point { row: 0, col: -1 },
    Point { row: 0, col: 1 },
    Point { row: 1, col: -1 },
    Point { row: 1, col: 0 },
    Point { row: 1, col: 1 },
];

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
        if self == other {
            return false;
        }
        match check_diag {
            true => distance.row == 1 && distance.col == 1,
            false => {
                distance.row == 0 && distance.col == 1 || distance.row == 1 && distance.col == 0
            }
        }
    }

    pub fn count_neighbors(&self, neighbors: &[Self]) -> usize {
        neighbors
            .iter()
            .fold(0, |acc, p| match self.touches(p, false) {
                true => acc + 1,
                false => acc,
            })
    }

    pub fn neighbors_hv(&self) -> Vec<Point> {
        NEIGHBORS_HV.iter().map(|dir| *self + *dir).collect()
    }

    pub fn neighbors(&self) -> Vec<Point> {
        NEIGHBORS.iter().map(|dir| *self + *dir).collect()
    }

    pub fn north(&self) -> Point {
        *self + Self { row: -1, col: 0 }
    }
    pub fn east(&self) -> Point {
        *self + Self { row: 0, col: 1 }
    }
    pub fn south(&self) -> Point {
        *self + Self { row: 1, col: 0 }
    }
    pub fn west(&self) -> Point {
        *self + Self { row: 0, col: -1 }
    }
    pub fn north_west(&self) -> Point {
        *self + Self { row: -1, col: -1 }
    }
    pub fn north_east(&self) -> Point {
        *self + Self { row: -1, col: 1 }
    }
    pub fn south_east(&self) -> Point {
        *self + Self { row: 1, col: 1 }
    }
    pub fn south_west(&self) -> Point {
        *self + Self { row: 1, col: -1 }
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
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            row: self.row + rhs.row,
            col: self.col + rhs.col,
        }
    }
}

impl Add<&Point> for Point {
    type Output = Self;
    fn add(self, rhs: &Point) -> Point {
        Self {
            row: self.row + rhs.row,
            col: self.col + rhs.col,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_neighbors() {
        let point = Point { row: 1, col: 1 };
        let neighbors = vec![
            Point::from((0, 0)), // -1,-1
            Point::from((0, 1)), // -1,0
            Point::from((0, 2)), // -1, +1
            Point::from((1, 0)), // 0, -1
            Point::from((1, 2)), // 0, +1
            Point::from((2, 0)), // +1, -1
            Point::from((2, 1)), // +1, 0
            Point::from((2, 2)), // +1, +1
        ];

        assert_eq!(neighbors, point.neighbors());
    }
    #[test]
    fn test_neighbors_hv() {
        let point = Point { row: 1, col: 1 };
        let neighbors = vec![
            Point::from((0, 1)),
            Point::from((1, 0)),
            Point::from((1, 2)),
            Point::from((2, 1)),
        ];

        assert_eq!(neighbors, point.neighbors_hv());
    }
}
