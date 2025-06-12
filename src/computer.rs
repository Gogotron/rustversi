use crate::board::{Board, Move, Player};

use std::cmp::{Ordering, Ord};
use rand::{rng, seq::SliceRandom};

type Heuristic<T> = fn(&Board, &Player) -> T;

pub fn minmax(board: &Board) -> Option<Move> {
    let player = board.player?;

    let mut moves = board.moves();
    moves.shuffle(&mut rng());
    moves.into_iter().max_by_key(|m| {
        helper(&board.play(m).unwrap(), &player, 3, heuristic)
    })
}

fn helper<T: Ord>(board: &Board, player: &Player, depth: u8, heuristic: Heuristic<T>) -> T {
    if depth == 0 || board.player.is_none() {
        return heuristic(board, player);
    }

    let current_player = board.player.unwrap();
    let maximize = current_player == *player;

    let branches = board.moves().into_iter().map(|m| {
        helper(&board.play(&m).unwrap(), player, depth - 1, heuristic)
    });

    if maximize {
        branches.max()
    } else {
        branches.min()
    }.unwrap()
}

fn heuristic(board: &Board, player: &Player) -> i16 {
    let (b, w) = board.score();
    let rel_score: i16 = match player {
        Player::Black => b as i16 - w as i16,
        Player::White => w as i16 - b as i16,
    };

    if board.player.is_none() {
        match rel_score.cmp(&0) {
            Ordering::Less => i16::MIN,
            Ordering::Greater => i16::MAX,
            Ordering::Equal => 0,
        }
    } else {
        rel_score
    }
}
