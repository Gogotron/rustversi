use crate::board::{Board, Move, Player};

use std::cmp::Ordering;

pub fn minmax(board: &Board) -> Option<Move> {
    let player = board.player?;
    assert_eq!(player, Player::Black);

    board.moves().into_iter().max_by_key(|m| {
        helper(&board.play(m).unwrap(), player, 3)
    })
}

fn helper(board: &Board, player: Player, depth: u8) -> i16 {
    if depth == 0 || board.player.is_none() {
        return heuristic(board, player);
    }

    let current_player = board.player.unwrap();
    let maximize = current_player == player;
    println!("{}", maximize);

    let branches = board.moves().into_iter().map(|m| {
        helper(&board.play(&m).unwrap(), player, depth - 1)
    });

    if maximize {
        branches.max()
    } else {
        branches.min()
    }.unwrap()
}

fn heuristic(board: &Board, player: Player) -> i16 {
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
