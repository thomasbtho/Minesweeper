#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CellState {
    HIDDEN,
    REVEALED,
    FLAGGED,
}

use CellState::*;

#[derive(Debug)]
pub struct Cell {
    is_mine: bool,
    adjacent_mines: u8,
    state: CellState,
}

impl Cell {
    pub fn new() -> Self {
        Self {
            is_mine: false,
            adjacent_mines: 0,
            state: HIDDEN,
        }
    }

    pub fn is_mine(&self) -> bool {
        self.is_mine
    }

    pub fn adjacent_mines(&self) -> u8 {
        self.adjacent_mines
    }

    pub fn state(&self) -> CellState {
        self.state
    }

    pub fn is_revealed(&self) -> bool {
        self.state == REVEALED
    }

    pub fn set_mine(&mut self, is_mine: bool) {
        self.is_mine = is_mine;
    }

    pub fn set_adjacent_mines(&mut self, mines: u8) {
        self.adjacent_mines = mines;
    }

    pub fn reveal(&mut self, force_reveal: bool) {
        if self.state == HIDDEN || force_reveal {
            self.state = REVEALED;
        }
    }

    pub fn toggle_flag(&mut self) {
        self.state = match self.state {
            HIDDEN => FLAGGED,
            FLAGGED => HIDDEN,
            _ => self.state,
        };
    }
}
