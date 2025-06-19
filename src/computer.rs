use crate::board::{Board, Move, Player};

use std::{
    cmp::{max, min, Ord, Ordering},
    time::{Duration, Instant}
};
use rand::{rng, seq::SliceRandom};

const DEPTH: u8 = 10;

type Heuristic<T> = fn(&Board, &Player) -> T;

pub trait BoundedOrd: Ord {
    const MIN: Self;
    const MAX: Self;
}

impl BoundedOrd for i16 {
    const MIN: i16 = i16::MIN;
    const MAX: i16 = i16::MAX;
}

pub fn minmax(board: &Board, timeout: Duration) -> Option<Move> {
    generic_minmax(board, timeout, heuristic)
}

fn generic_minmax<T: Ord>(board: &Board, timeout: Duration, heuristic: Heuristic<T>) -> Option<Move> {
    let player = board.player?;

    let start = Instant::now();
    let end = start + timeout;

    let depth = DEPTH;
    let mut moves = board.moves();
    moves.shuffle(&mut rng());
    moves.into_iter().max_by_key(|m| {
        helper(&board.play(m).unwrap(), &player, depth - 1, end, heuristic)
    })
}

fn helper<T: Ord>(board: &Board, player: &Player, depth: u8, end: Instant, heuristic: Heuristic<T>) -> T {
    if depth == 0 || board.player.is_none() || Instant::now() >= end {
        return heuristic(board, player);
    }

    let current_player = board.player.unwrap();
    let maximize = current_player == *player;

    let branches = board.moves().into_iter().map(|m| {
        helper(&board.play(&m).unwrap(), player, depth - 1, end, heuristic)
    });

    if maximize {
        branches.max()
    } else {
        branches.min()
    }.unwrap()
}

pub fn ab_minmax(board: &Board, timeout: Duration) -> Option<Move> {
    generic_ab_minmax(board, timeout, heuristic)
}

fn generic_ab_minmax<T: BoundedOrd + Copy>(board: &Board, timeout: Duration, heuristic: Heuristic<T>) -> Option<Move> {
    let player = &board.player?;

    let start = Instant::now();
    let end = start + timeout;

    let mut moves = board.moves();
    moves.shuffle(&mut rng());

    let depth = DEPTH;
    let mut alpha = T::MIN;
    let beta = T::MAX;
    let mut optimal_move = moves[0];
    let mut optimal_eval = T::MIN;
    for m in moves {
        let eval = ab_helper(&board.play(&m).unwrap(), player, depth - 1, alpha, beta, end, heuristic);

        if eval == optimal_eval {
            continue;
        }

        optimal_eval = max(eval, optimal_eval);
        alpha = max(optimal_eval, alpha);

        if eval == optimal_eval {
            optimal_move = m;
        }

        if alpha >= beta {
            break;
        }
    }

    Some(optimal_move)
}

fn ab_helper<T: BoundedOrd + Copy>(board: &Board, player: &Player, depth: u8, mut alpha: T, mut beta: T, end: Instant, heuristic: Heuristic<T>) -> T {
    if depth == 0 || board.player.is_none() || Instant::now() >= end {
        return heuristic(board, player);
    }

    let current_player = board.player.unwrap();
    let maximize = current_player == *player;

    let mut optimal_eval = if maximize { T::MIN } else { T::MAX };
    for m in board.moves() {
        let eval = ab_helper(&board.play(&m).unwrap(), player, depth - 1, alpha, beta, end, heuristic);

        if eval == optimal_eval {
            continue;
        }

        if maximize {
            optimal_eval = max(eval, optimal_eval);
            alpha = max(optimal_eval, alpha);
        } else {
            optimal_eval = min(eval, optimal_eval);
            beta = min(optimal_eval, beta);
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
