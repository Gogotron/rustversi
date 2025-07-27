mod bitmap;
use bitmap::Bitmap;

use std::{
    fs::File,
    io::{BufReader, Read, stdout},
    str::FromStr,
};


#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Player {
    Black,
    White
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Square {
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

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Move { x: u8, y: u8, }

#[derive(Debug, PartialEq)]
pub enum ParsingError {
    IOError,
    Generic,
    EmptyFile,
    InvalidCharacter(char),
    PlayerParseError(char),
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
        assert!(size % 2 == 0 && (2..=10).contains(&size));

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
            player: if size != 2 { Some(Player::Black) } else { None },
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

    pub fn set(&self, x: u8, y: u8, squ: Square) -> Self {
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

    pub fn score(&self) -> (u8, u8) {
        (self.black.popcount().try_into().unwrap(),
        self.white.popcount().try_into().unwrap())
    }

    pub fn moves(&self) -> Vec<Move> {
        self.moves.clone().map(|(x, y)| Move { x, y }).collect()
    }

    pub fn is_valid_move(&self, m: &Move) -> bool {
        m.x < self.size && m.y < self.size
            && self.moves.get(m.x, m.y)
    }

    pub fn play(&self, m: &Move) -> Option<Self> {
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

          while line != prev_line && line.subset_of(opponent) {
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

impl FromStr for Move {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.is_ascii() { return Err(()) };

        let Some(mut column) = s.chars().nth(0) else {
            return Err(());
        };
        column.make_ascii_uppercase();
        if !column.is_ascii_uppercase() {
            return Err(());
        }
        let column = (column as u8) - b'A';

        let (_, row) = s.split_at(1);
        let Ok(row): Result<u8, _> = row.parse() else {
            return Err(());
        };
        if row <= 1 {
            return Err(());
        }

        Ok(Move { x: column, y: row - 1 })
    }
}

impl From<Move> for String {
    fn from(m: Move) -> Self {
        format!("{}{}", (m.y + b'A') as char, m.x + 1)
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

impl From<Option<Player>> for Square {
    fn from(val: Option<Player>) -> Self {
        match val {
            Some(p) => Square::Disc(p),
            None => Square::Empty,
        }
    }
}

pub struct PlayerParseError {
    c: char
}
impl TryFrom<char> for Player {
    type Error = PlayerParseError;

    fn try_from(val: char) -> Result<Self, Self::Error> {
        match val {
            'X' => Ok(Player::Black),
            'O' => Ok(Player::White),
            _ => Err(PlayerParseError { c: val }),
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

impl From<PlayerParseError> for ParsingError {
    fn from(val: PlayerParseError) -> Self {
        Self::PlayerParseError(val.c)
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

        let player: Player = match next_ignore_chars_and_newlines(&mut chars) {
            Some(c) => Player::try_from(c)?,
            None => return Err(Self::Error::EmptyFile),
        };

        let mut first_row: Vec<Square> = vec!();
        while let Some(c) = next_ignore_chars(&mut chars) {
            match c {
                '\n' => if first_row.is_empty() { } else { break },
                'X' | 'O' | '_' => first_row.push(c.try_into().expect("Should be valid character")),
                _ => return Err(Self::Error::InvalidCharacter(c)),
            }
        }
        let size = first_row.len();
        if !(size % 2 == 0 && (2..=10).contains(&size)) {
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
                    0 => { },
                    _ => return Err(Self::Error::InconsistentSize),
                },
                'X' | 'O' | '_' => {
                    if row.len() < size && grid.len() < size {
                        row.push(c.try_into().expect("Should be valid character"))
                    } else { return Err(Self::Error::InconsistentSize) }
                },
                _ => return Err(Self::Error::InvalidCharacter(c)),
            }
        }
        match row.len() {
            l if l == size => { grid.push(row) },
            0 => { },
            _ => return Err(Self::Error::InconsistentSize),
        };

        if grid.len() != size {
            return Err(ParsingError::InconsistentSize)
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

        let (first, second) = match player {
            Player::Black => (&black, &white),
            Player::White => (&white, &black),
        };
        let moves = compute_moves(first, second);
        let (moves, player) = if !moves.is_empty() {
            (moves, Some(player))
        } else {
            let moves = compute_moves(second, first);
            if !moves.is_empty() {
                (moves, Some(player.other()))
            } else {
                (moves, None)
            }
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

impl From<&Board> for String {
    fn from(b: &Board) -> Self {
        let mut out = String::new();
        out.push(Square::from(b.player).into());
        out.push('\n');
        for y in 0..b.size {
            for x in 0..b.size {
                out.push(b.get(x, y).into());
            }
            out.push('\n');
        }
        out
    }
}

fn next_ignore_chars<T: Iterator<Item = char>>(iter: &mut T) -> Option<char> {
    match iter.find(|c| !c.is_ascii_whitespace() || *c =='\n') {
        Some('#') => iter.find(|c| *c == '\n'),
        c => c,
    }
}

fn next_ignore_chars_and_newlines<T: Iterator<Item = char>>(iter: &mut T) -> Option<char> {
    loop {
        match next_ignore_chars(iter) {
            Some('\n') => { },
            c => return c,
        }
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
        Board::new(8).pretty_print();
    }

    #[test]
    fn score() {
        assert_eq!(Board::new(8).score(), (2, 2));
        assert_eq!(Board::new(8).set(0, 0, Square::Disc(Player::Black)).score(), (3, 2));
    }

    #[test]
    fn compute_moves() {
        let moves = Board::new(8).moves();
        assert_eq!(moves, vec![Move { x: 3, y: 2 }, Move { x: 2, y: 3 }, Move { x: 5, y: 4 }, Move { x: 4, y: 5 }]);
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
        assert_eq!(Board::try_from(file), Err(ParsingError::InvalidCharacter('a')));

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
        assert_eq!(Board::try_from(file), Err(ParsingError::InconsistentSize));
    }
}
