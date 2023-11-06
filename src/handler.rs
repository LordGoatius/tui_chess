use crate::app::App;
use crate::game::Coordinates;
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
                return Ok(());
            }
            let col = (x - offset_x)/5;
            let row = (y - offset_y)/3;

            if let Some(_) = app.board.board[app.board_state.0 as usize][app.board_state.1 as usize] {
                if app.board.selected_moves(app.board_state).contains(&Coordinates(row as u8, col as u8)) {
                    // TODO make move 
                    // TEMP
                    let temp = app.board.board[app.board_state.0 as usize][app.board_state.1 as usize];
                    app.board.board[app.board_state.0 as usize][app.board_state.1 as usize] = None;
                    app.board.board[row as usize][col as usize] = temp;
                }
            }

            app.board_state.1 = col as u8;
            app.board_state.0 = row as u8;

        },
        _ => (),
    }
    Ok(())
}
