use bit_vec::BitVec;
use serde::{Deserialize, Serialize};

use crate::attributes::main::{
    InfoOption, MoveData, OptionValue, PieceAttributeTrait, PieceTraitInfo,
};
use crate::board::Board;
use crate::piece::Piece;
use crate::util::ILoc;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub(crate) struct Castle {
    pub(crate) destinations: Vec<ILoc>,
    pub(crate) black_destinations: Option<Vec<ILoc>>,
    pub(crate) rook: Vec<ILoc>,
    pub(crate) black_rook: Option<Vec<ILoc>>,
    pub(crate) rook_destination: Vec<ILoc>,
    pub(crate) black_rook_destination: Option<Vec<ILoc>>,
}
impl PieceAttributeTrait for Castle {
    fn moves(&self, board: &Board, piece: &Piece, moves: &mut Vec<MoveData>) {
        todo!()
    }

    fn attacks(&self, board: &Board, piece: &Piece, attacks: &mut BitVec) {
        todo!()
    }

    fn info(&self) -> PieceTraitInfo {
        PieceTraitInfo {
            name: "Castling",
            description: "Castling moves the piece to a given destination and moves a the given rook to its destination, only if the way is clear.",
            example: Some("King castling"),
            options: vec![InfoOption {
                name: "destinations",
                description: todo!(),
                optional: todo!(),
                options: todo!(),
                example: todo!()
            }]
        }
    }

    fn set_option(&mut self, name: &str, value: &Option<OptionValue>) {
        todo!()
    }
}
