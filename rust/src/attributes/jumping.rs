use bit_vec::BitVec;
use serde::{Deserialize, Serialize};

use crate::attributes::main::{
    bw, first_move_option, InfoOption, MoveData, OptionType, OptionValue, PieceAttributeTrait,
    PieceTraitInfo,
};
use crate::board::Board;
use crate::piece::Piece;
use crate::util::ILoc;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub(crate) struct Jumping {
    pub(crate) directions: Vec<ILoc>,
    pub(crate) black_directions: Option<Vec<ILoc>>,
    pub(crate) capture: bool,
    pub(crate) capture_only: bool,
    pub(crate) first_move_only: bool,
}
impl PieceAttributeTrait for Jumping {
    fn moves(&self, board: &Board, piece: &Piece, moves: &mut Vec<MoveData>) {
        if self.first_move_only
            && board.first_moves[piece.color]
                .get(board.loc_as_bit(&piece.loc))
                .contains(&true)
        {
            return;
        }

        let directions = bw(&self.directions, &self.black_directions, piece.color);
        for dir in directions {
            let loc = &(piece.loc.as_iLoc() + *dir).try_as_loc();
            if let Some(loc) = loc {
                if !board.valid_loc(loc) {
                    continue;
                }

                if !self.capture && board.check_loc(loc).is_some() {
                    continue;
                } else {
                    let occupied = board.check_loc(loc);
                    if let Some(color) = occupied {
                        if color != piece.color {
                            moves.push(MoveData {
                                castle: None,
                                piece: *piece,
                                to: *loc,
                                capture: Some(*loc),
                            });
                        }
                        continue;
                    }
                }

                if !self.capture_only {
                    moves.push(MoveData {
                        castle: None,
                        piece: *piece,
                        to: *loc,
                        capture: None,
                    });
                }
            }
        }
    }

    fn attacks(&self, board: &Board, piece: &Piece, attacks: &mut BitVec) {
        if !self.capture
            || (self.first_move_only
                && board.first_moves[piece.color]
                    .get(board.loc_as_bit(&piece.loc))
                    .contains(&true))
        {
            return;
        }

        let directions = bw(&self.directions, &self.black_directions, piece.color);
        for dir in directions {
            let loc = &(piece.loc.as_iLoc() + *dir).try_as_loc();
            if let Some(loc) = loc {
                if !board.valid_loc(loc) {
                    continue;
                }
                attacks.set(board.loc_as_bit(loc), true);
            }
        }
    }

    fn info(&self) -> PieceTraitInfo {
        PieceTraitInfo {
            name: "Jumping",
            description:
                "Can jump to any square in a given direction and capture enemy pieces (if configured).",
            example: Some("Knight, King, and Pawn (single & double move)"),
            options: vec![
                InfoOption {
                    optional: false,
                    name: "directions",
                    description: "The directions the piece can jump in.",
                    options: OptionType::ILocVec,
                    example: Some("Knight"),
                },
                InfoOption {
                    optional: true,
                    name: "black_directions",
                    description: "The directions the piece can jump in when black.",
                    options: OptionType::ILocVec,
                    example: Some("Pawn"),
                },
                InfoOption {
                    optional: false,
                    name: "capture",
                    description: "Can capture enemy pieces.",
                    options: OptionType::Bool,
                    example: Some("Knight"),
                },
                InfoOption {
                    optional: false,
                    name: "capture_only",
                    description: "Can only capture enemy pieces. Capture must be true.",
                    options: OptionType::Bool,
                    example: Some("Pawn"),
                },
                first_move_option(),
            ],
        }
    }

    fn set_option(&mut self, name: &str, value: &Option<OptionValue>) {
        if let Some(value) = value {
            match name {
                "directions" => self.directions = value.as_iloc_vec().unwrap(),
                "black_directions" => self.black_directions = Some(value.as_iloc_vec().unwrap()),
                "capture" => self.capture = value.as_bool().unwrap(),
                "capture_only" => self.capture_only = value.as_bool().unwrap(),
                "first_move_only" => self.first_move_only = value.as_bool().unwrap(),
                _ => {}
            }
        }
    }
}
