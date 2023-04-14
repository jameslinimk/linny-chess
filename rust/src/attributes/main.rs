use bit_vec::BitVec;
use enum_dispatch::enum_dispatch;
use serde::{Deserialize, Serialize};

use crate::attributes::enpassant::EnPassant;
use crate::attributes::jumping::Jumping;
use crate::attributes::sliding::Sliding;
use crate::board::Board;
use crate::piece::{Color, ColorType, Piece, PieceType};
use crate::util::{ILoc, Loc};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub(crate) enum OptionType {
    Bool,
    ILoc,
    ILocVec,
    DefaultPiece,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) enum OptionValue {
    Bool(bool),
    ILoc(ILoc),
    ILocVec(Vec<ILoc>),
    DefaultPiece(PieceType),
}
impl OptionValue {
    pub(crate) fn as_bool(&self) -> Option<bool> {
        if let OptionValue::Bool(value) = self {
            Some(*value)
        } else {
            None
        }
    }

    pub(crate) fn as_iloc(&self) -> Option<ILoc> {
        if let OptionValue::ILoc(value) = self {
            Some(*value)
        } else {
            None
        }
    }

    pub(crate) fn as_iloc_vec(&self) -> Option<Vec<ILoc>> {
        if let OptionValue::ILocVec(value) = self {
            Some(value.clone())
        } else {
            None
        }
    }

    pub(crate) fn as_default_piece(&self) -> Option<PieceType> {
        if let OptionValue::DefaultPiece(value) = self {
            Some(*value)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct InfoOption {
    pub(crate) name: &'static str,
    pub(crate) description: &'static str,
    pub(crate) optional: bool,
    pub(crate) options: OptionType,
    pub(crate) example: Option<&'static str>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct PieceTraitInfo {
    pub(crate) name: &'static str,
    pub(crate) description: &'static str,
    pub(crate) example: Option<&'static str>,
    pub(crate) options: Vec<InfoOption>,
}

pub(crate) fn first_move_option() -> InfoOption {
    InfoOption {
        optional: false,
        name: "first_move_only",
        description: "Can only move on the first move of the piece.",
        options: OptionType::Bool,
        example: Some("Pawn (double move)"),
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct MoveData {
    pub(crate) to: Loc,
    pub(crate) capture: Option<Loc>,
    pub(crate) castle: Option<(Loc, Loc)>,
    pub(crate) piece: Piece,
}

#[enum_dispatch]
pub(crate) trait PieceAttributeTrait {
    fn moves(&self, board: &Board, piece: &Piece, moves: &mut Vec<MoveData>);
    fn attacks(&self, board: &Board, piece: &Piece, attacks: &mut BitVec);
    fn info(&self) -> PieceTraitInfo;
    fn set_option(&mut self, name: &str, value: &Option<OptionValue>);
}

#[enum_dispatch(PieceAttributeTrait)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) enum PieceAttribute {
    Jumping,
    Sliding,
    EnPassant,
}
impl PieceAttribute {
    pub(crate) fn default_iter() -> [PieceAttribute; 3] {
        [
            PieceAttribute::from(Jumping::default()),
            PieceAttribute::from(Sliding::default()),
            PieceAttribute::from(EnPassant::default()),
        ]
    }
}

/// Returns the black or white version of a piece attribute depending on the color of the piece.
pub(crate) fn bw<'a, T>(white: &'a T, black: &'a Option<T>, color: ColorType) -> &'a T {
    if let Some(dirs) = black && color == Color::BLACK {
        dirs
    } else {
        white
    }
}
