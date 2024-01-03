use std::fmt::Debug;
use std::str::FromStr;

use chess::{Game, Square, Piece, ChessMove, Rank};
use ratatui::style::Color;
use ratatui::widgets::StatefulWidget;
use ratatui::style::{Styled, Style};
use ratatui::prelude::{Rect, Buffer};

struct MyPiece(Piece);

impl MyPiece {
    fn char(&self) -> char {
        match self.0 {
            Piece::Pawn   => '󰡙',
            Piece::King   => '󰡗',
            Piece::Queen  => '󰡚',
            Piece::Knight => '󰡘',
            Piece::Bishop => '󰡜',
            Piece::Rook   => '󰡛',
        }
    }
}

#[derive(Debug, Clone)]
pub struct MyGame(pub Game);

impl Default for MyGame {
    fn default() -> Self {
        MyGame(Game::new())
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct MyGameState {
    pub selected: Option<(u16, u16)>
}

impl MyGameState {
    pub fn to_chess_notation(&self) -> Option<String> {
        match self.selected {
            None => None,
            Some((rank, file)) => {
                Some(format!("{}{}",char::from_u32(file as u32 + 97).unwrap(),8-rank))
            } 
        }
    }
}

impl StatefulWidget for MyGame {
    type State = MyGameState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        if area.width < 40 || area.height < 24 {
            return;
        }

        let offset_x = ((area.width - 40) / 2) + area.x;
        let offset_y = ((area.height - 24) / 2) + area.y;

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

        for (rank, y) in (1..=8).rev().zip(0u16..) {
            for (file, x) in ('a'..='h').zip(0u16..) {
                let pos = self.0.current_position();
                let square = Square::from_str(&format!("{file}{rank}")[..]).unwrap();
                if let Some(piece) = pos.piece_on(square) {
                    let fg_color = if let chess::Color::White = pos.color_on(square).unwrap() {
                        Color::White
                    } else {
                        Color::Black
                    };
                    buf.get_mut(offset_x+(5*x)+2, offset_y+(3*y)+1).set_fg(fg_color).set_char(MyPiece(piece).char());
                }
            }
        }

        if let Some((y, x)) = state.selected {
            for i in 0..5 {
                for j in 0..3 {
                    buf.get_mut(offset_x+(5*x)+i, offset_y+(3*y)+j).set_bg(Color::Rgb(0x33, 0xa0, 0x33));
                }
            }
        }

        if let Some(_) = state.selected {
            for i in 0u8..8 {
                for j in 0u8..8 {
                    let int = 8 * (7 - i) + j;
                    let end_square: Square;
                    unsafe {
                        end_square = Square::new(int);
                    }
                    let start_square = Square::from_str(&state.to_chess_notation().unwrap()[..]).unwrap();
                    let chess_move = ChessMove::new(start_square, end_square, None);
                    if self.0.current_position().legal(chess_move) {
                        let buf_ref = buf.get_mut(offset_x+(5*j as u16)+2, offset_y+(3*i as u16)+1);
                        if buf_ref.symbol == " ".to_owned() {
                            buf_ref.set_char('⬤').set_fg(Color::Rgb(0x33, 0xa0, 0x33));
                        } else {
                            buf_ref.set_fg(Color::Rgb(0x33, 0xa0, 0x33));
                        }
                                        
                    }
                }
            }
            let selected_square = Square::from_str(&state.to_chess_notation().unwrap()[..]).unwrap();
            if Some(Piece::Pawn) == self.0.current_position().piece_on(selected_square) {
                let color = self.0.current_position().color_on(selected_square);
                if let Some(color) = color {
                    if color == chess::Color::Black && selected_square.get_rank() == Rank::Second {
                        let square_1 = selected_square.down().unwrap();
                        let square_2 = selected_square.down().unwrap().uright();
                        let square_3 = selected_square.down().unwrap().uleft();
                        let square_vec = vec![square_1, square_2, square_3];

                        for (i, square) in square_vec.iter().enumerate() {
                            if self.0.current_position().legal(ChessMove::new(selected_square, *square, Some(Piece::Queen))) {
                                let offset_x = offset_x + (state.selected.unwrap().1 * 5);
                                let offset_y = offset_y + (state.selected.unwrap().0 * 3) + 3;
                                match i {
                                    0 => {
                                        buf.get_mut(offset_x + 1, offset_y).set_char('󰡚').set_fg(Color::Black);
                                        buf.get_mut(offset_x + 3, offset_y).set_char('󰡛').set_fg(Color::Black);
                                        buf.get_mut(offset_x + 1, offset_y + 2).set_char('󰡘').set_fg(Color::Black);
                                        buf.get_mut(offset_x + 3, offset_y + 2).set_char('󰡜').set_fg(Color::Black);
                                    },
                                    1 => {
                                        buf.get_mut(offset_x + 6, offset_y).set_char('󰡚').set_fg(Color::Black);
                                        buf.get_mut(offset_x + 8, offset_y).set_char('󰡛').set_fg(Color::Black);
                                        buf.get_mut(offset_x + 6, offset_y + 2).set_char('󰡘').set_fg(Color::Black);
                                        buf.get_mut(offset_x + 8, offset_y + 2).set_char('󰡜').set_fg(Color::Black);
                                    },
                                    2 => {
                                        buf.get_mut(offset_x -4, offset_y).set_char('󰡚').set_fg(Color::Black);
                                        buf.get_mut(offset_x - 2, offset_y).set_char('󰡛').set_fg(Color::Black);
                                        buf.get_mut(offset_x - 4, offset_y + 2).set_char('󰡘').set_fg(Color::Black);
                                        buf.get_mut(offset_x - 2, offset_y + 2).set_char('󰡜').set_fg(Color::Black);
                                    },
                                    _ => (),
                                }
                            }
                        }
                    } else if color == chess::Color::White && selected_square.get_rank() == Rank::Seventh {
                        let square_1 = selected_square.up().unwrap();
                        let square_2 = selected_square.up().unwrap().uright();
                        let square_3 = selected_square.up().unwrap().uleft();
                        let square_vec = vec![square_1, square_2, square_3];

                        for (i, square) in square_vec.iter().enumerate() {
                            if self.0.current_position().legal(ChessMove::new(selected_square, *square, Some(Piece::Queen))) {
                                let offset_x = offset_x + (state.selected.unwrap().1 * 5);
                                let offset_y = offset_y + (state.selected.unwrap().0 * 3) - 3;
                                match i {
                                    0 => {
                                        buf.get_mut(offset_x + 1, offset_y).set_char('󰡚').set_fg(Color::White);
                                        buf.get_mut(offset_x + 3, offset_y).set_char('󰡛').set_fg(Color::White);
                                        buf.get_mut(offset_x + 1, offset_y + 2).set_char('󰡘').set_fg(Color::White);
                                        buf.get_mut(offset_x + 3, offset_y + 2).set_char('󰡜').set_fg(Color::White);
                                    },
                                    1 => {
                                        buf.get_mut(offset_x + 6, offset_y).set_char('󰡚').set_fg(Color::White);
                                        buf.get_mut(offset_x + 8, offset_y).set_char('󰡛').set_fg(Color::White);
                                        buf.get_mut(offset_x + 6, offset_y + 2).set_char('󰡘').set_fg(Color::White);
                                        buf.get_mut(offset_x + 8, offset_y + 2).set_char('󰡜').set_fg(Color::White);
                                    },
                                    2 => {
                                        buf.get_mut(offset_x -4, offset_y).set_char('󰡚').set_fg(Color::White);
                                        buf.get_mut(offset_x - 2, offset_y).set_char('󰡛').set_fg(Color::White);
                                        buf.get_mut(offset_x - 4, offset_y + 2).set_char('󰡘').set_fg(Color::White);
                                        buf.get_mut(offset_x - 2, offset_y + 2).set_char('󰡜').set_fg(Color::White);
                                    },
                                    _ => (),
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn to_chess_notation(rank: u16, file: u16) -> String {
    format!("{}{}",char::from_u32(file as u32 + 97).unwrap(),8-rank)
}
