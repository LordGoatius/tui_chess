use std::fmt::Debug;

use crate::game::{Board, BoardState};

#[derive(Debug, Default)]
pub struct App {
    /// should the application exit?
    pub should_quit: bool,
    /// game board
    pub board: Board,
    /// state
    pub board_state: BoardState,
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.should_quit = true;
    }
}
