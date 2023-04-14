#![feature(option_result_contains, let_chains)]
#![allow(dead_code)]

use bit_vec::BitVec;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::attributes::main::{MoveData, PieceAttribute, PieceAttributeTrait};
use crate::board::Board;
use crate::piece::{Color, Piece};
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

    board.raw_move(&MoveData {
        castle: None,
        piece: Piece {
            color: Color::WHITE,
            loc: Loc(6, 6),
            info_index: 0,
        },
        to: Loc(6, 4),
        capture: Some(Loc(6, 1)),
    });

    let test_piece = board.get(&Loc(6, 1));
    if let Some(piece) = test_piece {
        let moves = piece.moves(&board);
        board.print(Some(&moves.iter().map(|m| m.to).collect()));
    }

    println!("{}", board.hash());
}

static mut GAME: Option<Board> = None;
pub(crate) fn game() -> &'static mut Board {
    unsafe { GAME.as_mut().unwrap() }
}

#[wasm_bindgen]
/// Sets the global game variable to a new board with the given fen, or the default fen if None is given.
pub fn set_game(fen: Option<String>) {
    unsafe {
        GAME = Some(Board::new(8, 8));
        if let Some(fen) = fen {
            game().load_fen(&fen);
        } else {
            game().load_fen(Board::DEFAULT_FEN);
        }
    }
}

#[wasm_bindgen]
/// Checks if a game has been set.
pub fn game_exists() -> bool {
    unsafe { GAME.is_some() }
}

#[wasm_bindgen]
/// Returns the current fen of the game.
pub fn get_fen() -> Option<String> {
    unsafe { GAME.as_ref().map(|game| game.to_fen()) }
}

#[wasm_bindgen]
pub fn get_attribute_infos() -> String {
    let defaults = PieceAttribute::default_iter();

    let mut vec = Vec::with_capacity(defaults.len());
    for attribute in defaults.iter() {
        vec.push(attribute.info());
    }

    serde_json::to_string(&vec).unwrap()
}
