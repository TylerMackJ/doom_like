use std::ops::{Index, IndexMut};
use std::fmt;

pub struct Map<T: std::clone::Clone + std::fmt::Debug> {
    map: Vec<T>,
    pub width: usize,
    pub height: usize,
}

impl<T: std::clone::Clone + std::fmt::Debug> Map<T> {
    pub fn new(width: usize, height: usize, fill: T) -> Map<T> {
        Map {
            map: vec![fill; width * height],
            width: width,
            height: height,
        }
    }

    pub fn size(&self) -> usize {
        self.width * self.height
    }
}

impl<T: std::clone::Clone + std::fmt::Debug> Index<(usize, usize)> for Map<T> {
    type Output = T;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        &self.map[x + (y * self.width)]
    }
}

impl<T: std::clone::Clone + std::fmt::Debug> Index<(f64, f64)> for Map<T> {
    type Output = T;

    fn index(&self, (x, y): (f64, f64)) -> &Self::Output {
        &self.map[x as usize + (y as usize * self.width)]
    }
}

impl<T: std::clone::Clone + std::fmt::Debug> IndexMut<(usize, usize)> for Map<T> {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        &mut self.map[x + (y * self.width)]
    }
}

impl<T: std::clone::Clone + std::fmt::Debug> IndexMut<(f64, f64)> for Map<T> {
    fn index_mut(&mut self, (x, y): (f64, f64)) -> &mut Self::Output {
        &mut self.map[x as usize + (y as usize * self.width)]
    }
}

impl<T: std::clone::Clone + std::fmt::Debug> fmt::Debug for Map<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut out: String = String::new();
        for x in 0..self.width {
            for y in 0..self.height {
                out = format!("{}{:?}", out, self[(x, y)]);
            }
            out = format!("{}\n", out);
        }
        write!(f, "{}", out)
    }
}