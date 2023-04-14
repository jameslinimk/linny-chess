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
pub(crate) struct Sliding {
    pub(crate) directions: Vec<ILoc>,
    pub(crate) black_directions: Option<Vec<ILoc>>,
    pub(crate) capture: bool,
    pub(crate) first_move_only: bool,
}
impl PieceAttributeTrait for Sliding {
    fn moves(&self, board: &Board, piece: &Piece, moves: &mut Vec<MoveData>) {
        let directions = bw(&self.directions, &self.black_directions, piece.color);
        for dir in directions {
            let mut try_loc = (piece.loc.as_iLoc() + *dir).try_as_loc();
            while let Some(loc) = try_loc {
                if !board.valid_loc(&loc) {
                    break;
                }

                if !self.capture && board.check_loc(&loc).is_some() {
                    break;
                } else {
                    let occupied = board.check_loc(&loc);
                    if let Some(color) = occupied {
                        if color != piece.color {
                            moves.push(MoveData {
                                to: loc,
                                capture: Some(loc),
                            });
                        }
                        break;
                    }
                }

                moves.push(MoveData {
                    to: loc,
                    capture: None,
                });
                try_loc = (loc.as_iLoc() + *dir).try_as_loc();
            }
        }
    }

    fn attacks(&self, board: &Board, piece: &Piece, attacks: &mut BitVec) {
        if !self.capture {
            return;
        }

        let directions = bw(&self.directions, &self.black_directions, piece.color);
        for dir in directions {
            let mut try_loc = (piece.loc.as_iLoc() + *dir).try_as_loc();
            while let Some(loc) = try_loc {
                if !board.valid_loc(&loc) {
                    break;
                }

                let occupied = board.check_loc(&loc);
                if let Some(color) = occupied && color != piece.color {
					let index = board.loc_as_bit(&loc);
					attacks.set(index, true);
					break;
				}

                try_loc = (loc.as_iLoc() + *dir).try_as_loc();
            }
        }
    }

    fn info(&self) -> PieceTraitInfo {
        PieceTraitInfo {
            name: "Sliding",
            description: "Can move infinitely in a given direction as long as it is not occupied.",
            example: Some("Bishop, rook, queen"),
            options: vec![
                InfoOption {
                    optional: false,
                    name: "directions",
                    description: "The directions the piece can move in.",
                    options: OptionType::ILocVec,
                    example: Some("Bishop, rook, queen"),
                },
                InfoOption {
                    optional: true,
                    name: "black_directions",
                    description: "The directions the piece can move in when it is black.",
                    options: OptionType::ILocVec,
                    example: Some("Bishop, rook, queen"),
                },
                InfoOption {
                    optional: false,
                    name: "capture",
                    description: "Can capture enemy pieces at the end of the slide.",
                    options: OptionType::Bool,
                    example: Some("Bishop, rook, and queen"),
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
                "first_move_only" => self.first_move_only = value.as_bool().unwrap(),
                _ => {}
            }
        }
    }
}
