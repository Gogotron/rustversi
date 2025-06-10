use crate::board::{Board, Move};

pub fn minmax(board: &Board) -> Option<Move> {
    let player = board.player?;
    board.moves().into_iter().max_by_key(|m| {
        let (b, w) = board.play(m).unwrap().score();
        b - w
    })
}
