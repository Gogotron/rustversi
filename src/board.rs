#![allow(unused)]

mod bitmap;
use bitmap::Bitmap;

use std::fs::File;
use std::io::{stdout, BufReader, Read};


#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Player {
    Black,
    White
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Square {
    Disc(Player),
    Empty,
}

impl Player {
    pub fn other(&self) -> Self {
        match self {
            Self::Black => Self::White,
            Self::White => Self::Black,
        }
    }
}

pub struct Move { x: u8, y: u8, }

#[derive(Debug, PartialEq)]
pub enum ParsingError {
    IOError,
    Generic,
    EmptyFile,
    InvalidCharacter,
    BadSize,
    InconsistentSize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Board {
    size: u8,
    black: Bitmap,
    white: Bitmap,
    moves: Bitmap,
    pub player: Option<Player>,
}

impl Board {
    pub fn new(size: u8) -> Self {
        assert!(size % 2 == 0 && (4..=10).contains(&size));

        let black = Bitmap::new(size)
            .set(size / 2, size / 2 - 1)
            .set(size / 2 - 1, size / 2);
        let white = Bitmap::new(size)
            .set(size / 2 - 1, size / 2 - 1)
            .set(size / 2, size / 2);
        let moves = compute_moves(&black, &white);

        Self {
            size,
            black,
            white,
            moves,
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

        let moves = match self.player {
            Some(Player::Black) => compute_moves(&black, &white),
            Some(Player::White) => compute_moves(&white, &black),
            None => Bitmap::empty(self.size),
        };

        Self {
            size: self.size,
            black,
            white,
            moves,
            player: self.player
        }
    }

    pub fn score(&self) -> (u32, u32) {
        (self.black.popcount(), self.white.popcount())
    }

    fn compute_moves(&self) -> Bitmap {
        let (player, opponent) = match self.player {
            Some(Player::Black) => (&self.black, &self.white),
            Some(Player::White) => (&self.white, &self.black),
            None => return Bitmap::empty(self.size),
        };
        compute_moves(player, opponent)
    }

    pub fn moves(&self) -> Vec<Move> {
        self.moves.clone().map(|(x, y)| Move { x, y }).collect()
    }

    pub fn play(&self, m: Move) -> Option<Self> {
        let (x, y) = (m.x, m.y);
        if !self.moves.get(x, y) {
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

        let mut moves = compute_moves(&opponent, &player);
        let new_player = if moves.not_empty() {
            Some(self.player?.other())
        } else {
            moves = compute_moves(&player, &opponent);
            if compute_moves(&player, &opponent).not_empty() {
                Some(self.player?)
            } else {
                moves = Bitmap::empty(self.size);
                None
            }
        };

        let (new_black, new_white) = match self.player? {
            Player::Black => (player, opponent),
            Player::White => (opponent, player),
        };

        Some(Self {
            size: self.size,
            black: new_black,
            white: new_white,
            moves,
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

        print!("  ");
        for x in 0..self.size {
            print!(" {}", (b'A' + x) as char);
        }
        println!();
        for y in 0..self.size {
            print!("{:2}", y + 1);
            for x in 0..self.size {
                if self.moves.get(x, y) {
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

impl From<Player> for String {
    fn from(p: Player) -> Self {
        match p {
            Player::Black => "black",
            Player::White => "white",
        }.into()
    }
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

impl From<Square> for Option<Player> {
    fn from(val: Square) -> Self {
        match val {
            Square::Disc(p) => Some(p),
            Square::Empty => None,
        }
    }
}

impl TryFrom<char> for Square {
    type Error = ();
    fn try_from(val: char) -> Result<Self, Self::Error> {
        match val {
            'X' => Ok(Square::Disc(Player::Black)),
            'O' => Ok(Square::Disc(Player::White)),
            '_' => Ok(Square::Empty),
            _ => Err(()),
        }
    }
}

impl From<()> for ParsingError {
    fn from(_val: ()) -> Self {
        Self::Generic
    }
}

impl From<std::io::Error> for ParsingError {
    fn from(_val: std::io::Error) -> Self {
        Self::IOError
    }
}

impl TryFrom<File> for Board {
    type Error = ParsingError;

    fn try_from(file: File) -> Result<Self, Self::Error> {
        let mut chars = BufReader::new(file)
            .bytes()
            .filter(|r| r.is_ok())
            .map(|c| c.expect("Should be Ok.") as char);

        let player: Option<Player> = match next_ignore_chars(&mut chars) {
            Some(c) => Square::try_from(c)?.into(),
            None => return Err(Self::Error::EmptyFile),
        };

        let mut first_row: Vec<Square> = vec!();
        while let Some(c) = next_ignore_chars(&mut chars) {
            match c {
                '\n' => if first_row.is_empty() { } else { break },
                'X' | 'O' | '_' => first_row.push(c.try_into().expect("Should be valid character")),
                _ => return Err(Self::Error::InvalidCharacter),
            }
        }
        let size = first_row.len();
        if !(size % 2 == 0 && (4..=10).contains(&size)) {
            return Err(Self::Error::BadSize)
        }

        let mut grid: Vec<Vec<Square>> = vec!();
        grid.push(first_row);
        let mut row: Vec<Square> = vec!();
        while let Some(c) = next_ignore_chars(&mut chars) {
            match c {
                '\n' => match row.len() {
                    l if l == size => {
                        grid.push(row);
                        row = vec!();
                    }
                    0 => (),
                    _ => return Err(Self::Error::InconsistentSize),
                },
                'X' | 'O' | '_' => {
                    if row.len() < size && grid.len() < size {
                        row.push(c.try_into().expect("Should be valid character"))
                    } else { return Err(Self::Error::InconsistentSize) }
                },
                _ => return Err(Self::Error::InvalidCharacter),
            }
        }

        let size = size.try_into().expect("already checked");
        let black = grid.iter()
            .map(|r| r.iter().map(|s| *s == Square::Disc(Player::Black)).collect())
            .collect::<Vec<Vec<bool>>>()
            .into();
        let white = grid.iter()
            .map(|r| r.iter().map(|s| *s == Square::Disc(Player::White)).collect())
            .collect::<Vec<Vec<bool>>>()
            .into();
        let moves = match player {
            Some(Player::Black) => compute_moves(&black, &white),
            Some(Player::White) => compute_moves(&white, &black),
            None => Bitmap::empty(size),
        };

        Ok(Self {
            player,
            size,
            black,
            white,
            moves,
        })
    }
}

fn next_ignore_chars<T: Iterator<Item = char>>(iter: &mut T) -> Option<char> {
    match iter.find(|c| !c.is_ascii_whitespace() || *c =='\n') {
        Some('#') => iter.find(|c| *c == '\n'),
        c => c,
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
    use std::io::{SeekFrom, Seek, Write};

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

    #[test]
    fn ignore_chars() {
        let mut iter = "BEFORE COMMENT #IN COMMENT\nAFTER COMMENT".chars();
        let mut no_comment: Vec<char> = vec!();
        while let Some(c) = super::next_ignore_chars(&mut iter) {
            no_comment.push(c);
        }
        println!("{}", no_comment.into_iter().collect::<String>());
    }

    #[test]
    fn file_conversion() {
        let mut file: File = tempfile::tempfile().unwrap();
        write!(file, "X\n____\n_OX_\n_XO_\n____\n").unwrap();
        file.seek(SeekFrom::Start(0)).unwrap();
        assert_eq!(Board::try_from(file), Ok(Board::new(4)));

        let mut file: File = tempfile::tempfile().unwrap();
        write!(file, "X\n______\n______\n__OX__\n__XO__\n______\n______\n").unwrap();
        file.seek(SeekFrom::Start(0)).unwrap();
        assert_eq!(Board::try_from(file), Ok(Board::new(6)));

        let mut file: File = tempfile::tempfile().unwrap();
        write!(file, "X\n________\n________\n________\n___OX___\n___XO___\n________\n________\n________\n").unwrap();
        file.seek(SeekFrom::Start(0)).unwrap();
        assert_eq!(Board::try_from(file), Ok(Board::new(8)));

        let mut file: File = tempfile::tempfile().unwrap();
        write!(file, "X\n__________\n__________\n__________\n__________\n____OX____\n____XO____\n__________\n__________\n__________\n__________\n").unwrap();
        file.seek(SeekFrom::Start(0)).unwrap();
        assert_eq!(Board::try_from(file), Ok(Board::new(10)));

        let mut file: File = tempfile::tempfile().unwrap();
        write!(file, "X\n_a__\n_OX_\n_XO_\n____\n").unwrap();
        file.seek(SeekFrom::Start(0)).unwrap();
        assert_eq!(Board::try_from(file), Err(ParsingError::InvalidCharacter));

        let mut file: File = tempfile::tempfile().unwrap();
        write!(file, "X\n____\n_OX_\n_XO_\n____\n____").unwrap();
        file.seek(SeekFrom::Start(0)).unwrap();
        assert_eq!(Board::try_from(file), Err(ParsingError::InconsistentSize));

        let mut file: File = tempfile::tempfile().unwrap();
        write!(file, "X\n____\n_OX__\n_XO_\n____").unwrap();
        file.seek(SeekFrom::Start(0)).unwrap();
        assert_eq!(Board::try_from(file), Err(ParsingError::InconsistentSize));

        let mut file: File = tempfile::tempfile().unwrap();
        write!(file, "X\n__\nOX_\nXO\n__").unwrap();
        file.seek(SeekFrom::Start(0)).unwrap();
        assert_eq!(Board::try_from(file), Err(ParsingError::BadSize));
    }
}
