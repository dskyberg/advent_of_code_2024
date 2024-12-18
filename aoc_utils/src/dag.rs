use std::collections::HashMap;
use std::ops::{Deref, DerefMut};

use crate::Point;

#[derive(Debug, Clone)]
pub struct DAG(HashMap<u8, Vec<Point>>);

impl From<&Vec<Vec<u8>>> for DAG {
    fn from(input: &Vec<Vec<u8>>) -> Self {
        let mut map = HashMap::new();
        for (row, line) in input.iter().enumerate() {
            for (col, &b) in line.iter().enumerate() {
                map.entry(b).or_insert_with(Vec::new).push(Point {
                    row: row as isize,
                    col: col as isize,
                });
            }
        }

        Self(map)
    }
}

impl Deref for DAG {
    type Target = HashMap<u8, Vec<Point>>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for DAG {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
