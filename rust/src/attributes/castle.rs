use bit_vec::BitVec;
use serde::{Deserialize, Serialize};

use crate::attributes::main::{MoveData, OptionValue, PieceAttributeTrait, PieceTraitInfo};
use crate::board::Board;
use crate::piece::Piece;
use crate::util::ILoc;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub(crate) struct Castle {
    pub(crate) destinations: Vec<ILoc>,
}
impl PieceAttributeTrait for Castle {
    fn moves(&self, board: &Board, piece: &Piece, moves: &mut Vec<MoveData>) {
        todo!()
    }

    fn attacks(&self, board: &Board, piece: &Piece, attacks: &mut BitVec) {
        todo!()
    }

    fn info(&self) -> PieceTraitInfo {
        todo!()
    }

    fn set_option(&mut self, name: &str, value: &Option<OptionValue>) {
        todo!()
    }
}
