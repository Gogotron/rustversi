use std::ops::{BitAnd, BitOr};
use std::io::stdout;

#[derive(Debug, Clone, PartialEq)]
pub struct Bitmap {
    pub size: u8,
    bm: u128
}

impl Bitmap {
    pub fn new(size: u8) -> Self {
        Self::empty(size)
    }

    pub fn empty(size: u8) -> Self {
        if size > 10 {
            panic!("Bitmap too big")
        }
        Self { size, bm: 0 }
    }

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

    pub fn not(&self) -> Self {
        Self {
            size: self.size,
            bm: !self.bm & Self::full(self.size).bm
        }
    }

    pub fn is_empty(&self) -> bool {
        self.bm == 0
    }

    pub fn not_empty(&self) -> bool {
        self.bm != 0
    }

    pub fn set(&self, x: u8, y: u8) -> Self {
        assert!(x < self.size && y < self.size);
        Self {
            size: self.size,
            bm: self.bm | ((1 as u128) << (x + y * (self.size + 1)))
        }
    }

    pub fn unset(&self, x: u8, y: u8) -> Self {
        assert!(x < self.size && y < self.size);
        Self {
            size: self.size,
            bm: self.bm & !((1 as u128) << (x + y * (self.size + 1)))
        }
    }

    pub fn get(&self, x: u8, y: u8) -> bool {
        assert!(x < self.size && y < self.size);
        Self::new(self.size).set(x, y).intersection(self).bm != 0
    }

    pub fn popcount(&self) -> u32 {
        self.bm.count_ones()
    }

    pub fn shift_north(&self) -> Self {
        Self {
            size: self.size,
            bm: self.bm >> (self.size + 1)
        }
    }

    pub fn shift_south(&self) -> Self {
        Self {
            size: self.size,
            bm: self.bm << (self.size + 1)
        }
    }

    pub fn shift_east(&self) -> Self {
        Self {
            size: self.size,
            bm: self.bm << 1
        }
    }

    pub fn shift_west(&self) -> Self {
        Self {
            size: self.size,
            bm: self.bm >> 1
        }
    }

    pub fn shift_ne(&self) -> Self {
        Self {
            size: self.size,
            bm: self.bm >> self.size
        }
    }

    pub fn shift_se(&self) -> Self {
        Self {
            size: self.size,
            bm: self.bm << (self.size + 2)
        }
    }

    pub fn shift_sw(&self) -> Self {
        Self {
            size: self.size,
            bm: self.bm << self.size
        }
    }

    pub fn shift_nw(&self) -> Self {
        Self {
            size: self.size,
            bm: self.bm >> (self.size + 2)
        }
    }

    pub fn intersection(self: &Self, other: &Self) -> Self {
        assert_eq!(self.size, other.size);
        Self {
            size: self.size,
            bm: self.bm & other.bm
        }
    }

    pub fn union(self: &Self, other: &Self) -> Self {
        assert_eq!(self.size, other.size);
        Self {
            size: self.size,
            bm: self.bm | other.bm
        }
    }

    pub fn setminus(self: &Self, other: &Self) -> Self {
        assert_eq!(self.size, other.size);
        self.intersection(&other.not())
    }

    pub fn subset(self: &Self, other: &Self) -> bool {
        assert_eq!(self.size, other.size);
        self.union(other) == *self
    }

    pub fn supset(self: &Self, other: &Self) -> bool {
        assert_eq!(self.size, other.size);
        self.intersection(other) == *self
    }

    pub fn print(&self) {
        let handle = stdout().lock();
        for y in 0..self.size {
            for x in 0..self.size {
                print!("{}", if self.get(x, y) { '*' } else { '_' });
            }
            print!("\n");
        }
        drop(handle);
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
}
