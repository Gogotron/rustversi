use std::{
    io::stdout,
    ops::{BitAnd, BitOr},
};

/// A structure representing a square grid of boolean cells.
#[derive(Debug, Clone, PartialEq)]
pub struct Bitmap {
    pub size: u8,
    bm: u128
}

impl Bitmap {
    /// An alias for `empty`.
    pub fn new(size: u8) -> Self {
        Self::empty(size)
    }

    /// Create an empty `Bitmap` of size at most 10.
    pub fn empty(size: u8) -> Self {
        if size > 10 {
            panic!("Bitmap too big")
        }
        Self { size, bm: 0 }
    }

    /// Create a full `Bitmap` of size at most 10.
    pub fn full(size: u8) -> Self {
        if size > 10 {
            panic!("Bitmap too big")
        }
        if size % 2 != 0 {
            panic!("Bitmap should be evenly sized");
        }
        Self {
            size,
            bm: [0, 0, 0x1b, 0, 0x7bdef, 0, 0x1fbf7efdfbf, 0, 0x7fbfdfeff7fbfdfeff, 0, 0x1ffbff7feffdffbff7feffdffbff][size as usize]
        }
    }

    /// Negate the `Bitmap`.
    pub fn not(&self) -> Self {
        Self {
            size: self.size,
            bm: !self.bm & Self::full(self.size).bm
        }
    }

    /// Return whether the `Bitmap` is empty.
    pub fn is_empty(&self) -> bool {
        self.bm == 0
    }

    /// Return whether the `Bitmap` is not empty.
    pub fn not_empty(&self) -> bool {
        self.bm != 0
    }

    /// Set the given cell of the `Bitmap`.
    pub fn set(&self, x: u8, y: u8) -> Self {
        assert!(x < self.size && y < self.size);
        Self {
            size: self.size,
            bm: self.bm | (1_u128 << (x + y * (self.size + 1)))
        }
    }

    /// Unset the given cell of the `Bitmap`.
    pub fn unset(&self, x: u8, y: u8) -> Self {
        assert!(x < self.size && y < self.size);
        Self {
            size: self.size,
            bm: self.bm & !(1_u128 << (x + y * (self.size + 1)))
        }
    }

    /// Get the value of the given cell of the `Bitmap`.
    pub fn get(&self, x: u8, y: u8) -> bool {
        assert!(x < self.size && y < self.size);
        Self::new(self.size).set(x, y).intersection(self).bm != 0
    }

    /// Return the number of cells st in the `Bitmap`.
    pub fn popcount(&self) -> u32 {
        self.bm.count_ones()
    }

    /// Shift the cells of the `Bitmap` north.
    pub fn shift_north(&self) -> Self {
        Self {
            size: self.size,
            bm: self.bm >> (self.size + 1)
        }
    }

    /// Shift the cells of the `Bitmap` south.
    pub fn shift_south(&self) -> Self {
        Self {
            size: self.size,
            bm: self.bm << (self.size + 1)
        }
    }

    /// Shift the cells of the `Bitmap` east.
    pub fn shift_east(&self) -> Self {
        Self {
            size: self.size,
            bm: self.bm << 1
        }
    }

    /// Shift the cells of the `Bitmap` west.
    pub fn shift_west(&self) -> Self {
        Self {
            size: self.size,
            bm: self.bm >> 1
        }
    }

    /// Shift the cells of the `Bitmap` north-east.
    pub fn shift_ne(&self) -> Self {
        Self {
            size: self.size,
            bm: self.bm >> self.size
        }
    }

    /// Shift the cells of the `Bitmap` south-east.
    pub fn shift_se(&self) -> Self {
        Self {
            size: self.size,
            bm: self.bm << (self.size + 2)
        }
    }

    /// Shift the cells of the `Bitmap` south-west.
    pub fn shift_sw(&self) -> Self {
        Self {
            size: self.size,
            bm: self.bm << self.size
        }
    }

    /// Shift the cells of the `Bitmap` north-west.
    pub fn shift_nw(&self) -> Self {
        Self {
            size: self.size,
            bm: self.bm >> (self.size + 2)
        }
    }

    /// Compute the interstion with another `Bitmap`.
    pub fn intersection(&self, other: &Self) -> Self {
        assert_eq!(self.size, other.size);
        Self {
            size: self.size,
            bm: self.bm & other.bm
        }
    }

    /// Compute the union with another `Bitmap`.
    pub fn union(&self, other: &Self) -> Self {
        assert_eq!(self.size, other.size);
        Self {
            size: self.size,
            bm: self.bm | other.bm
        }
    }

    /// Compute the set difference with another `Bitmap`.
    pub fn setminus(&self, other: &Self) -> Self {
        assert_eq!(self.size, other.size);
        self.intersection(&other.not())
    }

    /// Return whether the `Bitmap` is a subset of another.
    pub fn subset_of(&self, other: &Self) -> bool {
        assert_eq!(self.size, other.size);
        self.union(other) == *other
    }

    /// Return whether the `Bitmap` is a superset of another.
    #[allow(dead_code)]
    pub fn superset_of(&self, other: &Self) -> bool {
        assert_eq!(self.size, other.size);
        self.intersection(other) == *other
    }

    /// Print a representation of the `Bitmap`, for debugging purposes.
    #[allow(dead_code)]
    pub fn print(&self) {
        let handle = stdout().lock();
        for y in 0..self.size {
            for x in 0..self.size {
                print!("{}", if self.get(x, y) { '*' } else { '_' });
            }
            println!();
        }
        drop(handle);
    }

    /// Return the first cell that is set, if any are set.
    fn lowest(&self) -> Option<(u8, u8)> {
        if self.is_empty() {
            None
        } else {
            let i: u8 = self.bm.trailing_zeros().try_into().expect("u8 shouldn't have more than 255 zeros");
            Some((i % (self.size + 1), i / (self.size + 1)))
        }
    }
}

impl Iterator for Bitmap {
    type Item = (u8, u8);

    fn next(&mut self) -> Option<Self::Item> {
        self.lowest().map(|(x, y)| {
            *self = self.unset(x, y);
            (x, y)
        })
    }
}

impl From<Vec<Vec<bool>>> for Bitmap {
    fn from(val: Vec<Vec<bool>>) -> Self {
        let mut bm = Bitmap::empty(val.len().try_into().expect("bitmaps must be at most of size 10"));
        for (y, r) in val.into_iter().enumerate() {
            let y = y.try_into().expect("bitmaps must be at most of size 10");
            for (x, b) in r.into_iter().enumerate() {
                if b {
                    bm = bm.set(x.try_into().expect("bitmaps must be at most of size 10"), y);
                }
            }
        }
        bm
    }
}

impl BitAnd for Bitmap {
    type Output = Self;

    fn bitand(self, other: Self) -> Self {
        self.intersection(&other)
    }
}

impl BitOr for Bitmap {
    type Output = Self;

    fn bitor(self, other: Self) -> Self {
        self.union(&other)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shifting() {
        for s in (4..=10).filter(|&s| s % 2 == 0) {
            let bitmap = Bitmap::new(s).set(1, 1);
            assert_eq!(bitmap.shift_north(), Bitmap::new(s).set(1, 0));
            assert_eq!(bitmap.shift_south(), Bitmap::new(s).set(1, 2));
            assert_eq!(bitmap.shift_east(), Bitmap::new(s).set(2, 1));
            assert_eq!(bitmap.shift_west(), Bitmap::new(s).set(0, 1));
            assert_eq!(bitmap.shift_ne(), Bitmap::new(s).set(2, 0));
            assert_eq!(bitmap.shift_se(), Bitmap::new(s).set(2, 2));
            assert_eq!(bitmap.shift_sw(), Bitmap::new(s).set(0, 2));
            assert_eq!(bitmap.shift_nw(), Bitmap::new(s).set(0, 0));
        }
    }

    #[test]
    fn weight() {
        let mut bitmap = Bitmap::new(10);
        assert_eq!(bitmap.popcount(), 0);
        bitmap = bitmap.set(1, 0);
        assert_eq!(bitmap.popcount(), 1);
        bitmap = bitmap.set(2, 0);
        assert_eq!(bitmap.popcount(), 2);
        bitmap = bitmap.set(0, 1);
        bitmap = bitmap.set(0, 1);
        bitmap = bitmap.set(0, 1);
        assert_eq!(bitmap.popcount(), 3);
        bitmap = bitmap.set(0, 2);
        assert_eq!(bitmap.popcount(), 4);
        bitmap = bitmap.unset(0, 2);
        assert_eq!(bitmap.popcount(), 3);
        bitmap = bitmap.unset(2, 0);
        bitmap = bitmap.unset(2, 0);
        bitmap = bitmap.unset(2, 0);
        assert_eq!(bitmap.popcount(), 2);
    }

    #[test]
    fn full() {
        for s in (2..=10).filter(|&s| s % 2 == 0) {
            let full = Bitmap::full(s);
            for x in 0..s {
                for y in 0..s {
                    assert!(full.get(x, y), "{} {} {}", s, x, y);
                }
            }
        }
    }

    #[test]
    fn not() {
        for s in (2..=10).filter(|&s| s % 2 == 0) {
            let not_full = Bitmap::full(s).not();
            for x in 0..s {
                for y in 0..s {
                    assert!(!not_full.get(x, y), "{} {} {}", s, x, y);
                }
            }
        }
    }

    #[test]
    fn is_empty() {
        for s in (2..=10).filter(|&s| s % 2 == 0) {
            assert!(Bitmap::new(s).is_empty());
            assert!(Bitmap::full(s).not().is_empty());
        }
    }

    #[test]
    fn iterator() {
        let mut bitmap = Bitmap::new(3);
        bitmap = bitmap.set(2, 0);
        bitmap = bitmap.set(0, 1);
        bitmap = bitmap.set(0, 2);
        bitmap = bitmap.set(1, 0);
        bitmap = bitmap.set(1, 1);
        assert_eq!(
            bitmap.collect::<Vec<(u8, u8)>>(),
            vec!((1, 0), (2, 0), (0, 1), (1, 1), (0, 2))
        );
    }
}
