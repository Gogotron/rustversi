use std::ops::{BitAnd, BitOr};

#[derive(Debug, Clone, PartialEq)]
pub struct Bitmap {
    size: u8,
    bm: u128
}

impl Bitmap {
    pub fn new(size: u8) -> Self {
        if size > 10 {
            panic!("Bitmap too big")
        }
        Self { size, bm: 0 }
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

    fn shift_north(&self) -> Self {
        Self {
            size: self.size,
            bm: self.bm >> (self.size + 1)
        }
    }

    fn shift_south(&self) -> Self {
        Self {
            size: self.size,
            bm: self.bm << (self.size + 1)
        }
    }

    fn shift_east(&self) -> Self {
        Self {
            size: self.size,
            bm: self.bm << 1
        }
    }

    fn shift_west(&self) -> Self {
        Self {
            size: self.size,
            bm: self.bm >> 1
        }
    }

    fn shift_ne(&self) -> Self {
        Self {
            size: self.size,
            bm: self.bm >> self.size
        }
    }

    fn shift_se(&self) -> Self {
        Self {
            size: self.size,
            bm: self.bm << (self.size + 2)
        }
    }

    fn shift_sw(&self) -> Self {
        Self {
            size: self.size,
            bm: self.bm << self.size
        }
    }

    fn shift_nw(&self) -> Self {
        Self {
            size: self.size,
            bm: self.bm >> (self.size + 2)
        }
    }

    fn intersection(self: &Self, other: &Self) -> Self {
        assert_eq!(self.size, other.size);
        Self {
            size: self.size,
            bm: self.bm & other.bm
        }
    }

    fn union(self: &Self, other: &Self) -> Self {
        assert_eq!(self.size, other.size);
        Self {
            size: self.size,
            bm: self.bm | other.bm
        }
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
}
