#![feature(option_result_contains, let_chains)]
#![allow(dead_code)]

use bit_vec::BitVec;

use crate::board::Board;
use crate::util::Loc;

mod attributes;
mod board;
mod board_util;
mod piece;
mod util;

fn bit_vec_to_list(map: &BitVec, board: &Board) -> Vec<Loc> {
    let mut vec = Vec::new();
    for (i, bit) in map.iter().enumerate() {
        if bit {
            vec.push(board.bit_as_loc(i));
        }
    }
    vec
}

fn main() {
    std::env::set_var("RUST_BACKTRACE", "1");

    let mut board = Board::new(8, 8);
    board.load_fen(Board::DEFAULT_FEN);

    let test_piece = board.get(&Loc(6, 1));
    if let Some(piece) = test_piece {
        let moves = piece.moves(&board);
        let moves = bit_vec_to_list(&moves, &board);
        board.print(Some(&moves));
    }
}
