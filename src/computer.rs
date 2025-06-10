use crate::board::{Board, Move, Player};

use std::cmp::Ordering;

fn heuristic(board: &Board, player: Player) -> i16 {
    let (b, w) = board.score();
    let rel_score: i16 = match player {
        Player::Black => b - w,
        Player::White => w - b,
    }.into();

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

pub fn minmax(board: &Board) -> Option<Move> {
    let player = board.player?;

    board.moves().into_iter().max_by_key(|m| {
        helper(&board.play(m).unwrap(), 3, player)
    })
}

fn helper(board: &Board, depth: u8, player: Player) -> i16 {
    let current_player = board.player.unwrap();
    let _maximize = current_player == player;

    if depth == 0 {
        board.moves().into_iter().max_by_key(|m| {
            let (b, w) = board.play(m).unwrap().score();
            match player {
                Player::Black => b - w,
                Player::White => w - b,
            }
        });
    }

    board.moves().into_iter().max_by_key(|m| {
        helper(&board.play(m).unwrap(), depth - 1, player)
    });
    
    todo!()
}
