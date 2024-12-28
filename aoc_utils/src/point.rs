use std::fmt::Display;
use std::ops::{Add, AddAssign, Rem, Sub, SubAssign};

static NEIGHBORS_HV: [Point; 4] = [
    Point { y: -1, x: 0 },
    Point { y: 0, x: -1 },
    Point { y: 0, x: 1 },
    Point { y: 1, x: 0 },
];

static NEIGHBORS: [Point; 8] = [
    Point { y: -1, x: -1 },
    Point { y: -1, x: 0 },
    Point { y: -1, x: 1 },
    Point { y: 0, x: -1 },
    Point { y: 0, x: 1 },
    Point { y: 1, x: -1 },
    Point { y: 1, x: 0 },
    Point { y: 1, x: 1 },
];

#[derive(Debug, PartialEq, Clone, Copy, Hash, Eq, Default)]
pub struct Point {
    pub y: isize,
    pub x: isize,
}

impl Point {
    pub fn mul(&self, factor: usize) -> Self {
        Self {
            y: self.y * factor as isize,
            x: self.x * factor as isize,
        }
    }

    #[inline]
    /// Absolute distance between rows
    pub fn vertical_distance(&self, other: &Self) -> isize {
        self.y.max(other.y) - self.y.min(other.y)
    }

    #[inline]
    /// Absolute distance between cols
    pub fn horizontal_distance(&self, other: &Self) -> isize {
        self.x.max(other.x) - self.x.min(other.x)
    }

    /// Measures the horizontal and vertical distance between two points
    pub fn distance(&self, other: &Self) -> Self {
        Self {
            x: self.horizontal_distance(other),
            y: self.vertical_distance(other),
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
            true => distance.y == 1 && distance.x == 1,
            false => distance.y == 0 && distance.x == 1 || distance.y == 1 && distance.x == 0,
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

    pub fn rem_euclid(&mut self, rhs: Self) -> Self {
        Self {
            x: (self.x % rhs.x).abs(),
            y: (self.y % rhs.y).abs(),
        }
    }

    pub fn north(&self) -> Point {
        *self + Self { y: -1, x: 0 }
    }
    pub fn east(&self) -> Point {
        *self + Self { y: 0, x: 1 }
    }
    pub fn south(&self) -> Point {
        *self + Self { y: 1, x: 0 }
    }
    pub fn west(&self) -> Point {
        *self + Self { y: 0, x: -1 }
    }
    pub fn north_west(&self) -> Point {
        *self + Self { y: -1, x: -1 }
    }
    pub fn north_east(&self) -> Point {
        *self + Self { y: -1, x: 1 }
    }
    pub fn south_east(&self) -> Point {
        *self + Self { y: 1, x: 1 }
    }
    pub fn south_west(&self) -> Point {
        *self + Self { y: 1, x: -1 }
    }
}

impl From<(usize, usize)> for Point {
    fn from(value: (usize, usize)) -> Self {
        Self {
            y: value.0 as isize,
            x: value.1 as isize,
        }
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{},{}]", self.y, self.x)
    }
}

impl Add for Point {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            y: self.y + rhs.y,
            x: self.x + rhs.x,
        }
    }
}

impl Add<&Point> for Point {
    type Output = Self;
    fn add(self, rhs: &Point) -> Point {
        Self {
            y: self.y + rhs.y,
            x: self.x + rhs.x,
        }
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        self.y += rhs.y;
        self.x += rhs.x;
    }
}
impl AddAssign<&Point> for Point {
    fn add_assign(&mut self, rhs: &Point) {
        self.y += rhs.y;
        self.x += rhs.x;
    }
}

impl Sub for Point {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self {
            y: self.y - rhs.y,
            x: self.x - rhs.x,
        }
    }
}

impl Sub<&Point> for Point {
    type Output = Self;
    fn sub(self, rhs: &Self) -> Self {
        Self {
            y: self.y - rhs.y,
            x: self.x - rhs.x,
        }
    }
}

impl SubAssign for Point {
    fn sub_assign(&mut self, rhs: Self) {
        self.y -= rhs.y;
        self.x -= rhs.x;
    }
}

impl SubAssign<&Point> for Point {
    fn sub_assign(&mut self, rhs: &Self) {
        self.y -= rhs.y;
        self.x -= rhs.x;
    }
}

impl Rem for Point {
    type Output = Self;
    fn rem(self, rhs: Self) -> Self::Output {
        Self {
            y: self.y % rhs.y,
            x: self.x % rhs.x,
        }
    }
}

impl Rem<&Point> for Point {
    type Output = Self;
    fn rem(self, rhs: &Self) -> Self::Output {
        Self {
            y: self.y % rhs.y,
            x: self.x % rhs.x,
        }
    }
}
