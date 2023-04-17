use bit_vec::BitVec;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::attributes::main::{
    bw, InfoOption, MoveData, OptionType, OptionValue, PieceAttributeTrait, PieceTraitInfo,
};
use crate::board::Board;
use crate::piece::{Piece, PieceType};
use crate::util::ILoc;

#[derive(Debug, Clone, Serialize, Deserialize, Default, JsonSchema)]
pub(crate) struct EnPassant {
    pub(crate) offsets: Vec<ILoc>,
    pub(crate) black_offsets: Option<Vec<ILoc>>,
    pub(crate) capture_offset: ILoc,
    pub(crate) black_capture_offset: Option<ILoc>,
    pub(crate) piece: PieceType,
}
impl PieceAttributeTrait for EnPassant {
    fn moves(&self, board: &Board, piece: &Piece, moves: &mut Vec<MoveData>) {
        let offsets = bw(&self.offsets, &self.black_offsets, piece.color);
        for offset in offsets.iter() {
            let loc = (piece.loc.as_iLoc() + *offset).try_as_loc();
            if let Some(loc) = loc {
                if !board.valid_loc(&loc) {
                    return;
                }

                let occupied = board.check_loc(&loc);
                if let Some(color) = occupied {
                    if color != piece.color {
                        let capture_offset = bw(
                            &self.capture_offset,
                            &self.black_capture_offset,
                            piece.color,
                        );
                        let to = (loc.as_iLoc() + *capture_offset).try_as_loc();
                        if let Some(to) = to {
                            moves.push(MoveData {
                                castle: None,
                                piece: *piece,
                                to,
                                capture: Some(loc),
                            });
                        }
                    }
                }
            }
        }
    }

    /// Because en passant only attacks certain pieces, it cannot be used here
    fn attacks(&self, _: &Board, _: &Piece, _: &mut BitVec) {}

    fn info(&self) -> PieceTraitInfo {
        PieceTraitInfo {
            name: "En passant",
            description: "Allows the piece to capture a given piece that has just moved and is offset by a given amount.",
            example: Some("Pawn en passant"),
            options: vec![
                InfoOption {
                    optional: false,
                    name: "offsets",
                    description: "The offsets to check for a piece to capture.",
                    options: OptionType::ILocVec,
                    example: Some("Pawns can only en passant other pawns that are next to them."),
                },
                InfoOption {
                    optional: true,
                    name: "black_offsets",
                    description: "The offsets to check for a piece to capture if the piece is black. If not provided, the white offsets will be used.",
                    options: OptionType::ILocVec,
                    example: Some("Pawns can only en passant other pawns that are next to them."),
                },
                InfoOption {
                    optional: false,
                    name: "capture_offset",
                    description: "After the piece takes, the offset of the resulting position. If not provided, the piece will be captured as normal.",
                    options: OptionType::ILoc,
                    example: Some("Pawns land on the square behind the captured pawn."),
                },
                InfoOption {
                    optional: true,
                    name: "black_capture_offset",
                    description: "After the piece takes, the offset of the resulting position if the piece is black. If not provided, the white offset will be used.",
                    options: OptionType::ILoc,
                    example: Some("Pawns land on the square behind the captured pawn."),
                },
                InfoOption {
                    optional: false,
                    name: "piece",
                    description: "The piece to capture.",
                    options: OptionType::DefaultPiece,
                    example: Some("Pawns can only en passant other pawns."),
                },
			],
        }
    }

    fn set_option(&mut self, name: &str, value: &Option<OptionValue>) {
        if let Some(value) = value {
            match name {
                "offsets" => self.offsets = value.as_iloc_vec().unwrap(),
                "black_offsets" => self.black_offsets = Some(value.as_iloc_vec().unwrap()),
                "capture_offset" => self.capture_offset = value.as_iloc().unwrap(),
                "black_capture_offset" => {
                    self.black_capture_offset = Some(value.as_iloc().unwrap())
                }
                "piece" => self.piece = value.as_default_piece().unwrap(),
                _ => {}
            }
        }
    }
}
