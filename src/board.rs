mod bitmap;
use bitmap::Bitmap;

use std::io::stdout;

#[derive(Debug, Clone, Copy)]
enum Player {
    Black,
    White
}

#[derive(Debug, Clone, Copy)]
enum Square {
    Disc(Player),
    Empty,
}

impl Into<char> for Player {
    fn into(self) -> char {
        match self {
            Player::Black => 'X',
            Player::White => 'O',
        }
    }
}

impl Into<char> for Square {
    fn into(self) -> char {
        match self {
            Square::Disc(p) => p.into(),
            Square::Empty => '_',
        }
    }
}

impl Square {
    fn to_char(&self) -> char {
        (*self).into()
    }
}

#[derive(Debug, Clone)]
struct Board {
    size: u8,
    black: Bitmap,
    white: Bitmap,
    player: Player,
}

impl Board {
    fn new(size: u8) -> Self {
        assert!(size % 2 == 0 && size >= 2 && size <= 10);
        Self {
            size,
            black: Bitmap::new(size)
                .set(size / 2, size / 2 - 1)
                .set(size / 2 - 1, size / 2),
            white: Bitmap::new(size)
                .set(size / 2 - 1, size / 2 - 1)
                .set(size / 2, size / 2),
            player: Player::Black
        }
    }

    fn get(&self, x: u8, y: u8) -> Square {
        assert!(x < self.size && y < self.size);
        if self.black.get(x, y) {
            Square::Disc(Player::Black)
        } else if self.white.get(x, y) {
            Square::Disc(Player::White)
        } else { Square::Empty }
    }

    fn set(&self, x: u8, y: u8, squ: Square) -> Self {
        assert!(x < self.size && y < self.size);
        let (black, white) = match squ {
            Square::Disc(Player::Black) => (
                self.black.set(x, y),
                self.white.unset(x, y)
            ),
            Square::Disc(Player::White) => (
                self.black.unset(x, y),
                self.white.set(x, y)
            ),
            Square::Empty => (
                self.black.unset(x, y),
                self.white.unset(x, y)
            ),
        };
        Self {
            size: self.size,
            black,
            white,
            player: self.player
        }
    }

    fn score(&self) -> (u32, u32) {
        (self.black.popcount(), self.white.popcount())
    }

    fn compute_moves(&self) -> Bitmap {
        let (player, opponent) = match self.player {
            Player::Black => (&self.black, &self.white),
            Player::White => (&self.white, &self.black),
        };

        let empty = player.union(opponent).not();
        let mut moves = Bitmap::new(self.size);

        for shift in [
            Bitmap::shift_north, Bitmap::shift_south, Bitmap::shift_east, Bitmap::shift_west,
            Bitmap::shift_ne, Bitmap::shift_se, Bitmap::shift_sw, Bitmap::shift_nw
        ] {
            let mut candidates = shift(player).intersection(opponent);

            while !candidates.is_empty() {
                moves = shift(&candidates).intersection(&empty).union(&moves);
                candidates = shift(&candidates).intersection(&opponent);
            }
        }

        moves
    }

    fn print(&self) {
        let handle = stdout().lock();
        for y in 0..self.size {
            for x in 0..self.size {
                print!("{}", self.get(x, y).to_char());
            }
            print!("\n");
        }
        drop(handle);
    }

    fn pretty_print(&self) {
        let handle = stdout().lock();
        let player_char: char = self.player.into();
        println!("'{}' player's turn.", player_char);

        let moves = self.compute_moves();
        print!("  ");
        for x in 0..self.size {
            print!(" {}", ('A' as u8 + x) as char);
        }
        print!("\n");
        for y in 0..self.size {
            print!("{:2}", y + 1);
            for x in 0..self.size {
                if moves.get(x, y) {
                    print!(" *");
                } else {
                    print!(" {}", self.get(x, y).to_char());
                }
            }
            print!("\n");
        }
        drop(handle);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn printing() {
        Board::new(8).print();
        Board::new(8).pretty_print();
    }

    #[test]
    fn score() {
        assert_eq!(Board::new(8).score(), (2, 2));
        assert_eq!(Board::new(8).set(0, 0, Square::Disc(Player::Black)).score(), (3, 2));
    }

    #[test]
    fn compute_moves() {
        let moves = Board::new(8).compute_moves();
        moves.print();
        assert!(moves.get(3, 2) && moves.get(2, 3) && moves.get(5, 4) && moves.get(4, 5));
    }
}
