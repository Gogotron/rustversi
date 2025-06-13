use crate::board::{Board, Move, Player};

use std::fmt::Display;
use std::cmp;
use std::cmp::{Ordering, Ord};
use rand::{rng, seq::SliceRandom};

type Heuristic<T> = fn(&Board, &Player) -> T;

pub trait BoundedOrd: Ord + Display {
    const MIN: Self;
    const MAX: Self;
}

impl BoundedOrd for i16 {
    const MIN: i16 = i16::MIN;
    const MAX: i16 = i16::MAX;
}

pub fn minmax(board: &Board) -> Option<Move> {
    let player = board.player?;

    let depth = 4;
    let mut moves = board.moves();
    moves.shuffle(&mut rng());
    moves.into_iter().max_by_key(|m| {
        helper(&board.play(m).unwrap(), &player, depth - 1, heuristic)
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

pub fn ab_minmax(board: &Board) -> Option<Move> {
    generic_ab_minmax(board, heuristic)
}

fn generic_ab_minmax<T: BoundedOrd + Copy>(board: &Board, heuristic: Heuristic<T>) -> Option<Move> {
    let player = &board.player?;

    let mut moves = board.moves();
    moves.shuffle(&mut rng());

    let depth = 4;
    let mut alpha = T::MIN;
    let beta = T::MAX;
    let mut optimal_move = None;
    let mut optimal_eval = T::MIN;
    for m in moves {
        let eval = ab_helper(&board.play(&m).unwrap(), player, depth - 1, alpha, beta, heuristic);

        if eval == optimal_eval {
            continue;
        }

        optimal_eval = cmp::max(eval, optimal_eval);
        alpha = cmp::max(optimal_eval, alpha);

        if eval == optimal_eval {
            optimal_move = Some(m);
        }

        if alpha >= beta {
            break;
        }
    }

    optimal_move
}

fn ab_helper<T: BoundedOrd + Copy>(board: &Board, player: &Player, depth: u8, mut alpha: T, mut beta: T, heuristic: Heuristic<T>) -> T {
    if depth == 0 || board.player.is_none() {
        return heuristic(board, player);
    }

    let current_player = board.player.unwrap();
    let maximize = current_player == *player;

    let mut optimal_eval = if maximize { T::MIN } else { T::MAX };
    for m in board.moves() {
        let eval = ab_helper(&board.play(&m).unwrap(), player, depth - 1, alpha, beta, heuristic);

        if eval == optimal_eval {
            continue;
        }

        if maximize {
            optimal_eval = cmp::max(eval, optimal_eval);
            alpha = cmp::max(optimal_eval, alpha);
        } else {
            optimal_eval = cmp::min(eval, optimal_eval);
            beta = cmp::min(optimal_eval, beta);
        }

        if alpha >= beta {
            break;
        }
    }

    optimal_eval
}

fn heuristic(board: &Board, player: &Player) -> i16 {
    let (b, w) = board.score();
    let rel_score: i16 = match player {
        Player::Black => b as i16 - w as i16,
        Player::White => w as i16 - b as i16,
    };

    let rel_score = if board.player.is_none() {
        match rel_score.cmp(&0) {
            Ordering::Less => i16::MIN,
            Ordering::Greater => i16::MAX,
            Ordering::Equal => 0,
        }
    } else {
        rel_score
    };
    rel_score
}
