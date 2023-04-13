use bit_vec::BitVec;
use rustc_hash::FxHashMap;

use crate::attributes::enpassant::EnPassant;
use crate::attributes::jumping::Jumping;
use crate::attributes::main::PieceAttributeTrait;
use crate::attributes::sliding::Sliding;
use crate::board::{Board, PieceInfo};
use crate::hashmap;
use crate::util::{ILoc, Loc};

#[derive(Debug, Clone, Copy)]
pub(crate) struct Color(usize);
impl Color {
    pub(crate) const WHITE: usize = 0;
    pub(crate) const BLACK: usize = 1;
}
pub(crate) type ColorType = usize;

#[derive(Debug, Clone, Copy)]
pub(crate) struct DefaultPiece;
impl DefaultPiece {
    pub(crate) const PAWN: usize = 0;
    pub(crate) const BISHOP: usize = 1;
    pub(crate) const KNIGHT: usize = 2;
    pub(crate) const ROOK: usize = 3;
    pub(crate) const QUEEN: usize = 4;
    pub(crate) const KING: usize = 5;

    pub(crate) fn from_char(char: char) -> Option<usize> {
        match char {
            'p' => Some(Self::PAWN),
            'b' => Some(Self::BISHOP),
            'n' => Some(Self::KNIGHT),
            'r' => Some(Self::ROOK),
            'q' => Some(Self::QUEEN),
            'k' => Some(Self::KING),
            _ => None,
        }
    }
}
pub(crate) type PieceType = usize;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Piece {
    pub(crate) color: ColorType,
    pub(crate) info_index: usize,
    pub(crate) loc: Loc,
}
impl Piece {
    pub(crate) fn info<'a>(&'a self, board: &'a Board) -> &PieceInfo {
        &board.pieces[&self.info_index]
    }

    pub(crate) fn moves(&self, board: &Board) -> BitVec {
        let info = self.info(board);
        let mut moves = BitVec::from_elem(board.width * board.height, false);
        for attribute in info.attributes.iter() {
            attribute.moves(board, self, &mut moves);
        }
        moves
    }

    pub(crate) fn attacks(&self, board: &Board) -> BitVec {
        let info = self.info(board);
        let mut attacks = BitVec::from_elem(board.width * board.height, false);
        for attribute in info.attributes.iter() {
            attribute.attacks(board, self, &mut attacks);
        }
        attacks
    }
}

pub(crate) fn default_pieces() -> FxHashMap<PieceType, PieceInfo> {
    hashmap! {
        DefaultPiece::PAWN => PieceInfo {
            id: DefaultPiece::PAWN,
            display: "Pawn".to_string(),
            icon: 'p',
            value: 1,
            attributes: vec![
                Jumping {
                    black_directions: Some(vec![ILoc(0, 1)]),
                    directions: vec![ILoc(0, -1)],
                    capture: false,
                    capture_only: false,
                    first_move_only: false,
                }.into(),
                Jumping {
                    black_directions: Some(vec![ILoc(0, 2)]),
                    directions: vec![ILoc(0, -2)],
                    capture: false,
                    capture_only: false,
                    first_move_only: true,
                }.into(),
                Jumping {
                    black_directions: Some(vec![ILoc(1, 1), ILoc(-1, 1)]),
                    directions: vec![ILoc(1, -1), ILoc(-1, -1)],
                    capture: true,
                    capture_only: true,
                    first_move_only: false,
                }.into(),
                EnPassant {
                    offsets: vec![ILoc(1, 0), ILoc(-1, 0)],
                    black_offsets: Some(vec![ILoc(1, 0), ILoc(-1, 0)]),
                    capture_offset: ILoc(0, 1),
                    black_capture_offset: Some(ILoc(0, -1)),
                    piece: DefaultPiece::PAWN,
                }.into()
            ],
        },
        DefaultPiece::BISHOP => PieceInfo {
            id: DefaultPiece::BISHOP,
            display: "Bishop".to_string(),
            icon: 'b',
            value: 3,
            attributes: vec![
                Sliding {
                    directions: vec![ILoc(1, 1), ILoc(-1, 1), ILoc(1, -1), ILoc(-1, -1)],
                    black_directions: None,
                    capture: true,
                    first_move_only: false,
                }.into(),
            ],
        },
        DefaultPiece::KNIGHT => PieceInfo {
            id: DefaultPiece::KNIGHT,
            display: "Knight".to_string(),
            icon: 'n',
            value: 3,
            attributes: vec![
                Jumping {
                    directions: vec![
                        ILoc(1, 2),
                        ILoc(2, 1),
                        ILoc(-1, 2),
                        ILoc(-2, 1),
                        ILoc(1, -2),
                        ILoc(2, -1),
                        ILoc(-1, -2),
                        ILoc(-2, -1),
                    ],
                    black_directions: None,
                    capture: true,
                    capture_only: false,
                    first_move_only: false,
                }.into()
            ],
        },
        DefaultPiece::ROOK => PieceInfo {
            id: DefaultPiece::ROOK,
            display: "Rook".to_string(),
            icon: 'r',
            value: 5,
            attributes: vec![
                Sliding {
                    directions: vec![ILoc(1, 0), ILoc(-1, 0), ILoc(0, 1), ILoc(0, -1)],
                    black_directions: None,
                    capture: true,
                    first_move_only: false,
                }.into(),
            ],
        },
        DefaultPiece::QUEEN => PieceInfo {
            id: DefaultPiece::QUEEN,
            display: "Queen".to_string(),
            icon: 'q',
            value: 9,
            attributes: vec![
                Sliding {
                    directions: vec![
                        ILoc(1, 0),
                        ILoc(-1, 0),
                        ILoc(0, 1),
                        ILoc(0, -1),
                        ILoc(1, 1),
                        ILoc(-1, 1),
                        ILoc(1, -1),
                        ILoc(-1, -1),
                    ],
                    black_directions: None,
                    capture: true,
                    first_move_only: false,
                }.into(),
            ],
        },
        DefaultPiece::KING => PieceInfo {
            id: DefaultPiece::KING,
            display: "King".to_string(),
            icon: 'k',
            value: 0,
            attributes: vec![
                Jumping {
                    directions: vec![
                        ILoc(1, 0),
                        ILoc(-1, 0),
                        ILoc(0, 1),
                        ILoc(0, -1),
                        ILoc(1, 1),
                        ILoc(-1, 1),
                        ILoc(1, -1),
                        ILoc(-1, -1),
                    ],
                    black_directions: None,
                    capture: true,
                    capture_only: false,
                    first_move_only: false,
                }.into(),
            ],
        },
    }
}
