use colored::Colorize;
use ordinal::Ordinal;
use serde::{Deserialize, Serialize};

use crate::attributes::main::MoveData;
use crate::board::Board;
use crate::piece::{Color, ColorType, DefaultPiece, Piece};
use crate::util::Loc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct CpgnMetadata {
    pub(crate) white: Option<String>,
    pub(crate) black: Option<String>,
    pub(crate) result: Option<String>,
    pub(crate) reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct Cpgn {
    pub(crate) metadata: CpgnMetadata,
    pub(crate) moves: Vec<String>,
}

impl Board {
    pub(crate) fn bitlength(&self) -> usize {
        self.width * self.height
    }

    pub(crate) fn loc_as_bit(&self, loc: &Loc) -> usize {
        loc.1 * self.width + loc.0
    }

    pub(crate) fn bit_as_loc(&self, index: usize) -> Loc {
        let x = index % self.width;
        let y = index / self.width;
        Loc(x, y)
    }

    pub(crate) fn valid_loc(&self, loc: &Loc) -> bool {
        loc.0 < self.width && loc.1 < self.height
    }

    pub(crate) fn full_moves(&self) -> u16 {
        self.half_moves / 2
    }

    /// Check and see if a location is occupied by a piece, and if so, return the color of the piece
    pub(crate) fn check_loc(&self, loc: &Loc) -> Option<ColorType> {
        if !self.valid_loc(loc) {
            return None;
        }

        let index = self.loc_as_bit(loc);
        for color in [Color::WHITE, Color::BLACK].iter() {
            if self.general_locations[*color].get(index).contains(&true) {
                return Some(*color);
            }
        }
        None
    }

    pub(crate) fn get(&self, loc: &Loc) -> Option<Piece> {
        if !self.valid_loc(loc) {
            return None;
        }

        let color = self.check_loc(loc)?;
        let index = self.loc_as_bit(loc);

        for (info_index, map) in self.piece_locations[color].iter() {
            if map.get(index).contains(&true) {
                return Some(Piece {
                    color,
                    info_index: *info_index,
                    loc: *loc,
                });
            }
        }

        None
    }

    pub(crate) fn print(&self, highlight: Option<&Vec<Loc>>) {
        println!(
            "\n{}'s turn, {} half move",
            (if self.turn == 0 { "white" } else { "black" })
                .bold()
                .white(),
            Ordinal(self.half_moves).to_string().bold().white(),
        );
        println!(
            "\n{}",
            format!(
                "   {}",
                (0..self.width)
                    .map(|x| format!(" {x} "))
                    .collect::<String>()
            )
            .on_black()
            .white()
        );
        for y in 0..self.height {
            print!("{}", format!(" {y} ").to_string().on_black().white());
            for x in 0..self.width {
                let piece = self.get(&Loc(x, y));
                let mut str = String::new();
                str.push(' ');
                if let Some(piece) = piece {
                    let info = piece.info(self);
                    str.push(if piece.color == Color::WHITE {
                        info.icon.to_ascii_uppercase()
                    } else {
                        info.icon
                    });
                    str.push(' ');
                } else {
                    str.push(' ');
                    str.push(' ');
                }

                if let Some(highlight) = highlight && highlight.contains(&Loc(x, y)) {
                    print!("{}", str.on_truecolor(255, 255, 255).bold().black());
                } else if (x + y) % 2 == 0 {
                    print!("{}", str.on_truecolor(139, 110, 85).bold().white());
                } else {
                    print!("{}", str.on_truecolor(107, 82, 62).bold().white());
                }
            }
            println!();
        }
        println!();
    }

    pub(crate) const DEFAULT_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR";
    pub(crate) fn load_fen(&mut self, fen: &str) {
        for (y, row) in fen.split('/').enumerate() {
            let mut x = 0;
            for c in row.chars() {
                if c.is_ascii_digit() {
                    x += c.to_digit(10).unwrap() as usize;
                } else {
                    let color = if c.is_uppercase() {
                        Color::WHITE
                    } else {
                        Color::BLACK
                    };
                    let piece = DefaultPiece::from_char(c.to_ascii_lowercase());
                    if let Some(piece) = piece {
                        self.insert(&Piece {
                            loc: Loc(x, y),
                            color,
                            info_index: piece,
                        });
                        x += 1;
                    }
                }
            }
        }
    }

    pub(crate) fn to_fen(&self) -> String {
        let mut fen = String::new();
        for y in 0..self.height {
            let mut empty = 0;
            for x in 0..self.width {
                let piece = self.get(&Loc(x, y));
                if let Some(piece) = piece {
                    if empty > 0 {
                        fen.push_str(&empty.to_string());
                        empty = 0;
                    }
                    let info = piece.info(self);
                    fen.push(if piece.color == Color::WHITE {
                        info.icon.to_ascii_uppercase()
                    } else {
                        info.icon
                    });
                } else {
                    empty += 1;
                }
            }

            if empty > 0 {
                fen.push_str(&empty.to_string());
            }

            if y < self.height - 1 {
                fen.push('/');
            }
        }
        fen
    }

    pub(crate) const TEST_CPGN: &str = "{
        metadata: {
            white: \"lin_as_white\",
            black: \"lin_as_black\",
            result: \"1-0\",
            reason: \"checkmate\"
        },
        cpgn: [\"e2e4\", \"e7e5\", \"g1f3\". \"b8c6\"]
    }";
    pub(crate) fn load_cpgn(&mut self, _cpgn: &str) {
        todo!()
    }
    pub(crate) fn to_cpgn(&self, metadata: CpgnMetadata) -> Cpgn {
        let mut moves = Vec::with_capacity(self.half_moves as usize);
        for mov in self.move_history.iter() {
            moves.push(format!(
                "{}{}",
                mov.from.as_notation(),
                mov.to().as_notation()
            ));
        }
        Cpgn { metadata, moves }
    }

    pub(crate) fn insert(&mut self, piece: &Piece) {
        let index = self.loc_as_bit(&piece.loc);
        self.general_locations[piece.color].set(index, true);
        self.piece_locations[piece.color]
            .get_mut(&piece.info_index)
            .unwrap()
            .set(index, true);
    }

    pub(crate) fn raw_move(&mut self, piece: &Piece, move_data: &MoveData) {
        let from = self.loc_as_bit(&piece.loc);
        let to = self.loc_as_bit(&move_data.to);

        self.general_locations[piece.color].set(from, false);
        self.general_locations[piece.color].set(to, true);

        if let Some(capture) = move_data.capture {
            let capture_index = self.loc_as_bit(&capture);
            let piece = self.get(&capture).unwrap();

            self.general_locations[piece.color].set(capture_index, false);
            self.piece_locations[piece.color]
                .get_mut(&piece.info_index)
                .unwrap()
                .set(capture_index, false);
        }

        let piece_locations = self.piece_locations[piece.color]
            .get_mut(&piece.info_index)
            .unwrap();

        piece_locations.set(from, false);
        piece_locations.set(to, true);
    }
}
