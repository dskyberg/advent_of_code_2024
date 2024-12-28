use crate::{Point, data_to_grid};
use std::fmt::Display;
use std::ops::{Deref, DerefMut};

#[derive(Debug, Clone, PartialEq)]
pub struct Matrix<T: Display + From<char>> {
    pub grid: Vec<Vec<T>>,
    pub width: usize,
    pub height: usize,
}

impl<T: Display + From<char>> Matrix<T> {
    pub fn neighbors(&self, p: &Point) -> Vec<Point> {
        p.neighbors()
            .into_iter()
            .filter(|point| self.valid_point(point))
            .collect()
    }

    #[inline]
    fn valid_point(&self, point: &Point) -> bool {
        point.y >= 0
            && point.y < self.height as isize
            && point.x >= 0
            && point.x < self.width as isize
    }
}

impl<T: From<char> + Display> From<&str> for Matrix<T> {
    fn from(value: &str) -> Self {
        let grid = data_to_grid::<T>(value);
        let height = grid.len();
        let width = grid[0].len();
        Self {
            grid,
            height,
            width,
        }
    }
}

impl<T: From<char> + Display> Display for Matrix<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in &self.grid {
            for x in y {
                write!(f, "{}", x)?;
            }
            writeln!(f)?;
        }
        writeln!(f)
    }
}

impl<T: From<char> + Display> Deref for Matrix<T> {
    type Target = Vec<Vec<T>>;
    fn deref(&self) -> &Self::Target {
        &self.grid
    }
}

impl<T: From<char> + Display> DerefMut for Matrix<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.grid
    }
}
