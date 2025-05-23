mod bitmap;
use bitmap::Bitmap;

use std::io::stdout;
use std::fs::File;

#[derive(Debug, Clone, Copy)]
enum Player {
    Black,
    White
}

impl Player {
    fn other(&self) -> Self {
        match self {
            Self::Black => Self::White,
            Self::White => Self::Black,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Square {
    Disc(Player),
    Empty,
}

impl From<Player> for char {
    fn from(val: Player) -> Self {
        match val {
            Player::Black => 'X',
            Player::White => 'O',
        }
    }
}

impl From<Square> for char {
    fn from(val: Square) -> Self {
        match val {
            Square::Disc(p) => p.into(),
            Square::Empty => '_',
        }
    }
}

#[derive(Debug, Clone)]
pub struct Board {
    size: u8,
    black: Bitmap,
    white: Bitmap,
    player: Option<Player>,
}

impl Board {
    pub fn new(size: u8) -> Self {
        assert!(size % 2 == 0 && (2..=10).contains(&size));
        Self {
            size,
            black: Bitmap::new(size)
                .set(size / 2, size / 2 - 1)
                .set(size / 2 - 1, size / 2),
            white: Bitmap::new(size)
                .set(size / 2 - 1, size / 2 - 1)
                .set(size / 2, size / 2),
            player: Some(Player::Black),
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
            Some(Player::Black) => (&self.black, &self.white),
            Some(Player::White) => (&self.white, &self.black),
            None => return Bitmap::new(self.size),
        };
        compute_moves(player, opponent)
    }

    fn play(&self, x: u8, y: u8) -> Option<Self> {
        if self.compute_moves().get(x, y) {
            return None;
        }

        let (player, opponent) = match self.player? {
            Player::Black => (&self.black, &self.white),
            Player::White => (&self.white, &self.black),
        };

        let move_mask = Bitmap::new(self.size).set(x, y);
        let mut flipped = Bitmap::new(self.size);
        for shift in [
            Bitmap::shift_north, Bitmap::shift_south, Bitmap::shift_east, Bitmap::shift_west,
            Bitmap::shift_ne, Bitmap::shift_se, Bitmap::shift_sw, Bitmap::shift_nw
        ] {
          let mut prev_line = Bitmap::new(self.size);
          let mut line = shift(&move_mask);

          while line != prev_line && line.subset(opponent) {
            prev_line = line;
            line = prev_line.union(&shift(&prev_line));
          }
          if line.intersection(player).not_empty() {
            flipped = flipped.union(&line);
          }
        }

        let (player, opponent) = (
            player.union(&move_mask).union(&flipped),
            opponent.setminus(&flipped)
        );

        let new_player = if compute_moves(&opponent, &player).not_empty() {
            Some(self.player?.other())
        } else if compute_moves(&player, &opponent).not_empty() {
            Some(self.player?)
        } else {
            None
        };

        let (new_black, new_white) = match self.player? {
            Player::Black => (player, opponent),
            Player::White => (opponent, player),
        };

        Some(Self {
            size: self.size,
            black: new_black,
            white: new_white,
            player: new_player,
        })
    }

    fn print(&self) {
        let handle = stdout().lock();
        for y in 0..self.size {
            for x in 0..self.size {
                print!("{}", char::from(self.get(x, y)));
            }
            println!();
        }
        drop(handle);
    }

    pub fn pretty_print(&self) {
        let handle = stdout().lock();
        if let Some(player) = self.player {
            println!("'{}' player's turn.", char::from(player));
        } else {
            println!("Game ended.");
        }
        println!();

        let moves = self.compute_moves();
        print!("  ");
        for x in 0..self.size {
            print!(" {}", (b'A' + x) as char);
        }
        println!();
        for y in 0..self.size {
            print!("{:2}", y + 1);
            for x in 0..self.size {
                if moves.get(x, y) {
                    print!(" *");
                } else {
                    print!(" {}", char::from(self.get(x, y)));
                }
            }
            println!();
        }

        let (black, white) = self.score();
        println!("Score: '{}' = {}, '{}' = {}",
                 char::from(Player::Black), black,
                 char::from(Player::White), white);
        drop(handle);
    }
}

impl From<File> for Board {
    fn from(val: File) -> Self {
        todo!()
    }
}

fn compute_moves(player: &Bitmap, opponent: &Bitmap) -> Bitmap {
    assert_eq!(player.size, opponent.size);

    let empty = player.union(opponent).not();
    let mut moves = Bitmap::new(player.size);

    for shift in [
        Bitmap::shift_north, Bitmap::shift_south, Bitmap::shift_east, Bitmap::shift_west,
        Bitmap::shift_ne, Bitmap::shift_se, Bitmap::shift_sw, Bitmap::shift_nw
    ] {
        let mut candidates = shift(player).intersection(opponent);

        while !candidates.is_empty() {
            moves = shift(&candidates).intersection(&empty).union(&moves);
            candidates = shift(&candidates).intersection(opponent);
        }
    }

    moves
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
