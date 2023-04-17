use std::hash::{Hash, Hasher};

use bit_vec::BitVec;
use rustc_hash::{FxHashMap, FxHasher};
use serde::{Deserialize, Serialize};

use crate::attributes::main::{MoveData, PieceAttribute};
use crate::hashmap;
use crate::piece::{default_pieces, Color, ColorType, Piece, PieceType};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(not(feature = "bare"), derive(schemars::JsonSchema))]
pub(crate) struct PieceInfo {
    pub(crate) id: usize,
    pub(crate) display: String,
    pub(crate) icon: char,
    pub(crate) value: i32,
    pub(crate) image_path: [String; 2],
    pub(crate) attributes: Vec<PieceAttribute>,
}

#[test]
fn schemas() {
    let schema = schemars::schema_for!(PieceInfo);
    println!("PieceInfo:\n{}\n", serde_json::to_string(&schema).unwrap());
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
    pub(crate) turn: ColorType,
    pub(crate) move_history: Vec<MoveData>,
    pub(crate) hashes: FxHashMap<u64, u8>,
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
            turn: Color::WHITE,
            move_history: vec![],
            hashes: hashmap! {},
        }
    }

    fn raw_raw_move(&mut self, from: usize, to: usize, piece: &Piece) {
        let piece_locations = self.piece_locations[piece.color]
            .get_mut(&piece.info_index)
            .unwrap();
        piece_locations.set(from, false);
        piece_locations.set(to, true);
        self.general_locations[piece.color].set(from, false);
        self.general_locations[piece.color].set(to, true);
    }

    pub(crate) fn raw_move(&mut self, move_data: &MoveData) {
        let from = self.loc_as_bit(&move_data.piece.loc);
        let to = self.loc_as_bit(&move_data.to);

        if let Some(capture) = &move_data.capture {
            let capture_index = self.loc_as_bit(capture);
            let piece = self.get(capture).unwrap();

            self.general_locations[piece.color].set(capture_index, false);
            self.piece_locations[piece.color]
                .get_mut(&piece.info_index)
                .unwrap()
                .set(capture_index, false);
        }

        if let Some((from, to)) = &move_data.castle {
            let piece = self.get(from).unwrap();

            let from = self.loc_as_bit(from);
            let to = self.loc_as_bit(to);

            self.raw_raw_move(from, to, &piece);
        }

        self.raw_raw_move(from, to, &move_data.piece);
    }

    pub(crate) fn move_piece(&mut self, move_data: &MoveData) {
        self.raw_move(move_data);
        self.move_history.push(*move_data);

        let hash_entry = self.hashes.entry(self.hash()).or_insert(0);
        *hash_entry += 1;
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
        self.half_moves().hash(&mut hash);
        hash.finish()
    }
}
