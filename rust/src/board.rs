use std::hash::{Hash, Hasher};

use bit_vec::BitVec;
use rustc_hash::{FxHashMap, FxHasher};

use crate::attributes::enpassant::EnPassant;
use crate::attributes::main::PieceAttribute;
use crate::hashmap;
use crate::piece::{default_pieces, Color, ColorType, Piece, PieceType};
use crate::util::Loc;

#[derive(Debug, Clone)]
pub(crate) struct PieceInfo {
    pub(crate) id: usize,
    pub(crate) display: String,
    pub(crate) icon: char,
    pub(crate) value: i32,
    pub(crate) attributes: Vec<PieceAttribute>,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct MoveHistory {
    pub(crate) piece: Piece,
    pub(crate) from: Loc,
}
impl MoveHistory {
    pub(crate) fn to(&self) -> &Loc {
        &self.piece.loc
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Board {
    pub(crate) width: usize,
    pub(crate) height: usize,
    pub(crate) pieces: FxHashMap<PieceType, PieceInfo>,
    /// What squares are occupied by a piece of a given color and type
    /// - `piece_locations[color][piece_type]`
    pub(crate) piece_locations: [FxHashMap<PieceType, BitVec>; 2],
    /// What squares are occupied by a piece of a given color
    pub(crate) general_locations: [BitVec; 2],
    /// What pieces haven't moved yet
    pub(crate) first_moves: [BitVec; 2],
    /// Whats squares are under attack by a piece of a given color
    pub(crate) attacks: [BitVec; 2],
    pub(crate) half_moves: u16,
    pub(crate) turn: ColorType,
    pub(crate) move_history: Vec<MoveHistory>,
}
impl Board {
    pub(crate) fn new(width: usize, height: usize) -> Self {
        let pieces = default_pieces();
        let bitvec = BitVec::from_elem(width * height, false);
        let piece_location = {
            let mut map = hashmap! {};
            for (piece_type, _) in pieces.iter() {
                map.insert(*piece_type, bitvec.clone());
            }
            map
        };

        Self {
            width,
            height,
            pieces,
            piece_locations: [piece_location.clone(), piece_location],
            general_locations: [bitvec.clone(), bitvec.clone()],
            first_moves: [bitvec.clone(), bitvec.clone()],
            attacks: [bitvec.clone(), bitvec],
            half_moves: 0,
            turn: Color::WHITE,
            move_history: vec![],
        }
    }

    pub(crate) fn move_piece(&mut self, from: &Loc, to: &Loc, piece: &Piece) {
        self.raw_move(from, to);
        let info = piece.info(self);
        let en_passant = info
            .attributes
            .iter()
            .find(|&attr| matches!(attr, PieceAttribute::EnPassant(_)));
        if let Some(en_passant) = en_passant {}
    }

    pub(crate) fn hash(&self) -> u64 {
        let mut hash = FxHasher::default();
        self.general_locations.hash(&mut hash);
        for test in self.piece_locations.iter() {
            for (piece_type, bitvec) in test.iter() {
                piece_type.hash(&mut hash);
                bitvec.hash(&mut hash);
            }
        }
        self.half_moves.hash(&mut hash);
        hash.finish()
    }
}
