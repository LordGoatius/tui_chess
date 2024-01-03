use std::str::FromStr;

use crate::app::App;
use crate::game::to_chess_notation;
use chess::{Square, ChessMove, Piece, Rank};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers, MouseEvent, MouseEventKind, MouseButton};
use crossterm::terminal::size;
use anyhow::{Result, Ok};

pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> Result<()> {
    match key_event.code {
        // Exit application on `ESC` or `q`
        KeyCode::Esc | KeyCode::Char('q') => {
            app.quit();
        }
        // Exit application on `Ctrl-C`
        KeyCode::Char('c') | KeyCode::Char('C') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.quit();
            }
        }
        // Counter handlers
        // KeyCode::Right => {
        //     app.increment_counter();
        // }
        // KeyCode::Left => {
        //     app.decrement_counter();
        // }
        // KeyCode::Char('j') => {
        //     app.decrement_counter();
        // },
        // KeyCode::Char('k') => {
        //     app.increment_counter();
        // }
        // Other handlers you could add here.
        _ => {}
    }
    Ok(())
}

pub fn handle_mouse_events(mouse_event: MouseEvent, app: &mut App) -> Result<()> {
    match mouse_event.kind {
        MouseEventKind::Up(MouseButton::Left) => {
            let x = mouse_event.column;
            let y = mouse_event.row;
            let (size_x, size_y) = size().unwrap();
            let offset_x = (size_x - 40)/2;
            let offset_y = (size_y - 24)/2;
            if x < offset_x || y < offset_y || x > offset_x + 40 || y > offset_y + 24 {
                app.game_state.selected = None;
                return Ok(());
            }

            let col = (x - offset_x)/5;
            let row = (y - offset_y)/3;

            if col >= 8 || row >= 8 {
                app.game_state.selected = None;
                return Ok(());
            }
            
            if let Some((s_y, s_x)) = app.game_state.selected {
                let start_square = Square::from_str(&app.game_state.to_chess_notation().unwrap()[..]).unwrap();

                if let Some(piece) = app.game.0.current_position().piece_on(start_square) {
                    let end_square   = Square::from_str(&to_chess_notation(row, col)[..]).unwrap();

                    let current_move = ChessMove::new(start_square, end_square, None);

                    if app.game.0.current_position().legal(current_move) {
                        app.game.0.make_move(current_move);
                        return Ok(());
                    }

                    if piece == Piece::Pawn {
                        let offset_x = offset_x + (s_x * 5);
                        let offset_y = offset_y + (s_y * 3);

                        let offset_x = if s_x > col {
                            offset_x - 5
                        } else if s_x < col {
                            offset_x + 5
                        } else {
                            offset_x + 0
                        };

                        let board = app.game.0.current_position();

                        let offset_y = if board.color_on(start_square).unwrap() == chess::Color::White {
                            offset_y - 3
                        } else {
                            offset_y + 3
                        };

                        if (start_square.get_rank() == Rank::Seventh && board.color_on(start_square).unwrap() == chess::Color::White) ||
                           (start_square.get_rank() == Rank::Second && board.color_on(start_square).unwrap() == chess::Color::Black) {
                            if y == offset_y && x == offset_x + 1 {
                                let promotion_move = ChessMove::new(start_square, end_square, Some(Piece::Queen));
                                if board.legal(promotion_move) {
                                    app.game.0.make_move(promotion_move);
                                    return Ok(());
                                }
                            } else if y == offset_y && x == offset_x + 3 {
                                let promotion_move = ChessMove::new(start_square, end_square, Some(Piece::Rook));
                                if board.legal(promotion_move) {
                                    app.game.0.make_move(promotion_move);
                                    return Ok(());
                                }
                            } else if y == offset_y + 2 && x == offset_x + 1 {
                                let promotion_move = ChessMove::new(start_square, end_square, Some(Piece::Knight));
                                if board.legal(promotion_move) {
                                    app.game.0.make_move(promotion_move);
                                    return Ok(());
                                }
                            } else if y == offset_y + 2 && x == offset_x + 3 {
                                let promotion_move = ChessMove::new(start_square, end_square, Some(Piece::Bishop));
                                if board.legal(promotion_move) {
                                    app.game.0.make_move(promotion_move);
                                    return Ok(());
                                }
                            } 
                        }
                    }
                }
            }
            
            app.game_state.selected = Some((row, col));

        },
        _ => (),
    }
    Ok(())
}
