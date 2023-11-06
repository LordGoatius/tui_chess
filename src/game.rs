use std::fmt::{Debug, Display};

use ratatui::style::Color;
use ratatui::widgets::StatefulWidget;
use ratatui::style::{Styled, Style};
use ratatui::prelude::{Rect, Buffer};

// Custom Piece Type Data

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum PieceColor {
    Black,
    White,
}

impl PieceColor {
    pub fn swap(&self) -> PieceColor {
        match self {
            PieceColor::White => PieceColor::Black,
            PieceColor::Black => PieceColor::White,
        }
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct PawnData {
    pub has_moved: bool,
    pub can_en_pessant: bool,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum PieceType {
    Pawn(PawnData),
    Rook(bool),
    King(bool),
    Bishop,
    Knight,
    Queen,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Coordinates(pub u8, pub u8);

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Piece {
    pub piece_type: PieceType,
    pub color: PieceColor,
}

// Custom Piece Type Traits

impl Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let letter: char = if self.color == PieceColor::White {
            match &self.piece_type {
                PieceType::Pawn(_) => {
                    '󰡙'
                },
                PieceType::Rook(_) => {
                    '󰡛'
                },
                PieceType::King(_) => {
                    '󰡗'
                },
                PieceType::Bishop => {
                    '󰡜'
                },
                PieceType::Knight => {
                    '󰡘'
                },
                PieceType::Queen =>
                    '󰡚'
            }
        } else {
            match &self.piece_type {
                PieceType::Pawn(_) => {
                    ''
                },
                PieceType::Rook(_) => {
                    ''
                },
                PieceType::King(_) => {
                    ''
                },
                PieceType::Bishop => {
                    ''
                },
                PieceType::Knight => {
                    ''
                },
                PieceType::Queen =>
                    ''
            }
        };
        write!(f, "{letter}")
    }
}

impl Piece {
    fn char(&self) -> char {
        self.to_string().chars().nth(0).unwrap()
    }
}

impl Display for PieceColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PieceColor::Black => {
                write!(f, "Black")
            },
            PieceColor::White => {
                write!(f, "White")
            },
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Board {
    pub board: [[Option<Piece>; 8]; 8],
    pub style: Style,
}

impl Default for Board {
    fn default() -> Self {
        let black_pawn = Piece { piece_type: PieceType::Pawn(PawnData { has_moved: false, can_en_pessant: false }), color: PieceColor::Black };
        let black_rook = Piece { piece_type: PieceType::Rook(false), color: PieceColor::Black };
        let black_king = Piece { piece_type: PieceType::King(false), color: PieceColor::Black };
        let black_bishop = Piece { piece_type: PieceType::Bishop, color: PieceColor::Black };
        let black_knight = Piece { piece_type: PieceType::Knight, color: PieceColor::Black };
        let black_queen = Piece { piece_type: PieceType::Queen, color: PieceColor::Black };
        let white_pawn = Piece { piece_type: PieceType::Pawn(PawnData { has_moved: false, can_en_pessant: false }), color: PieceColor::White };
        let white_rook = Piece { piece_type: PieceType::Rook(false), color: PieceColor::White };
        let white_king = Piece { piece_type: PieceType::King(false), color: PieceColor::White };
        let white_bishop = Piece { piece_type: PieceType::Bishop, color: PieceColor::White };
        let white_knight = Piece { piece_type: PieceType::Knight, color: PieceColor::White };
        let white_queen = Piece { piece_type: PieceType::Queen, color: PieceColor::White };
        Board { board: [
                [Some(black_rook), Some(black_knight), Some(black_bishop), Some(black_queen), Some(black_king), Some(black_bishop), Some(black_knight), Some(black_rook)],
                [Some(black_pawn), Some(black_pawn), Some(black_pawn), Some(black_pawn), Some(black_pawn), Some(black_pawn), Some(black_pawn), Some(black_pawn)],
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
                [Some(white_pawn), Some(white_pawn), Some(white_pawn), Some(white_pawn), Some(white_pawn), Some(white_pawn), Some(white_pawn), Some(white_pawn)],
                [Some(white_rook), Some(white_knight), Some(white_bishop), Some(white_queen), Some(white_king), Some(white_bishop), Some(white_knight), Some(white_rook)],
            ],
            style: Style::default()
        }
    }
}

impl Board {
    pub fn selected_moves(&self, state: BoardState) -> Vec<Coordinates> {
        vec![Coordinates(3,2),Coordinates(4,6),Coordinates(0,1)]
    }
}

#[derive(Debug, Clone, Copy)]
pub struct BoardState(pub u8, pub u8);

impl Default for BoardState {
    fn default() -> Self {
        BoardState(0, 0)
    }
}

impl StatefulWidget for Board {
    type State = BoardState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        if area.width < 40 || area.height < 24 {
            return;
        } else if state.0 > 7 || state.1 > 7 {
            return;
        }

        let offset_x = ((area.width - 40)/2) + area.x;
        let offset_y = ((area.height - 24)/2) + area.y;

        for i in 0..8 {
            for j in 0..8 {
                let color = if (i + j) % 2 == 0 {
                    Color::Rgb(0xc0, 0xc1, 0xc3)
                } else {
                    Color::Rgb(0xb5,0x87,0x63)
                };
                for k in 0..5 {
                   for l in 0..3 {
                       buf.get_mut(offset_x+5*i+k, offset_y+3*j+l).set_bg(color);
                   }
                }
            }
        }

        for (y, row) in self.board.iter().enumerate() {
            for (x, space) in row.iter().enumerate() {
                if let Some(piece) = space {
                    let fg_color = if let PieceColor::White = piece.color {
                        Color::White
                    } else {
                        Color::Black
                    };
                    buf.get_mut(offset_x+(5*x as u16)+2, offset_y+(3*y as u16)+1).set_char(piece.char()).set_fg(fg_color);
                }
            }
        }

        // Render selected piece

        for k in 0..5 {
           for l in 0..3 {
               buf.get_mut(offset_x+5*state.1 as u16+k, offset_y+3*state.0 as u16+l).set_bg(Color::Red);
           }
        }

        for Coordinates(y,x) in self.selected_moves(*state) {
            let cell = buf.get_mut(offset_x+(5*x as u16)+2, offset_y+(3*y as u16)+1);
            match &cell.symbol[..] {
                " " => cell.set_char('⬤').set_fg(Color::Red),
                _ => cell.set_fg(Color::Red),
            };
        }
    }
}

impl Styled for Board {
    type Item = Board;

    fn style(&self) -> Style {
        self.style
    }

    fn set_style(mut self, style: Style) -> Self::Item {
        self.style = style;
        self
    }
}


