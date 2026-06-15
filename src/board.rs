use crate::cell::{Cell, CellState};
use crate::game::GameState::{self, *};
use rand::random_range;

#[derive(Debug, Default)]
pub struct Board {
    width: usize,
    height: usize,
    number_of_mines: u16,
    cells: Vec<Vec<Cell>>,
}

impl Board {
    pub const MIN_WIDTH: usize = 8;
    pub const MAX_WIDTH: usize = 30;
    pub const MIN_HEIGHT: usize = 8;
    pub const MAX_HEIGHT: usize = 24;

    pub fn new(width: usize, height: usize, number_of_mines: u16) -> Self {
        Self {
            width,
            height,
            number_of_mines,
            cells: vec![],
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn cells(&self) -> &Vec<Vec<Cell>> {
        &self.cells
    }

    pub fn check_game_state(&self) -> GameState {
        let max_possible_free_cells = (self.width * self.height) as u16 - self.number_of_mines;
        let mut free_cells: u16 = 0;

        for row in &self.cells {
            for cell in row {
                if cell.state() == CellState::REVEALED {
                    if cell.is_mine() {
                        return LOST;
                    }
                    free_cells += 1;
                }
            }
        }

        if free_cells == max_possible_free_cells {
            return WON;
        }

        PLAYING
    }

    pub fn reveal_all_cells(&mut self) {
        for row in &mut self.cells {
            for cell in row {
                cell.reveal(true);
            }
        }
    }

    pub fn generate_board(&mut self) {
        self.init_cells();
        self.place_mines();
        self.calculate_adjacent_numbers();
    }

    pub fn flag_cell(&mut self, x: usize, y: usize) {
        if x >= self.width || y >= self.height {
            println!("Error: wrong coordinates!");
            return;
        }
        self.cells[y][x].toggle_flag();
    }

    pub fn reveal_cell(&mut self, x: usize, y: usize) {
        if x >= self.width || y >= self.height {
            println!("Error: wrong coordinates!");
            return;
        }

        // you can't reveal a flagged cell
        if self.cells[y][x].state() == CellState::FLAGGED {
            println!("This cell is flagged!");
            return;
        }

        // if it's a mine, reveal it and stop
        if self.cells[y][x].is_mine() {
            self.cells[y][x].reveal(false);
            return;
        }

        let mut stack = vec![];
        stack.push((x, y));

        while let Some((cx, cy)) = stack.pop() {
            let cell = &mut self.cells[cy][cx];

            cell.reveal(false);

            // if this is a number cell, do nothing
            if cell.adjacent_mines() > 0 {
                continue;
            }

            let min_row = cy.saturating_sub(1);
            let max_row = (cy + 1).min(self.height - 1);
            let min_col = cx.saturating_sub(1);
            let max_col = (cx + 1).min(self.width - 1);

            for row in min_row..=max_row {
                for col in min_col..=max_col {
                    if row == cy && col == cx {
                        continue;
                    }
                    if !self.cells[row][col].is_revealed() {
                        self.cells[row][col].reveal(false);
                        stack.push((col, row));
                    }
                }
            }
        }
    }

    fn init_cells(&mut self) {
        let mut cells = Vec::with_capacity(self.height);

        for _ in 0..self.height {
            let mut current_row = Vec::with_capacity(self.width);

            for _ in 0..self.width {
                current_row.push(Cell::new());
            }

            cells.push(current_row);
        }

        self.cells = cells;
    }

    fn place_mines(&mut self) {
        let mut mines_placed = 0;

        while mines_placed < self.number_of_mines {
            let col = random_range(0..self.width);
            let row = random_range(0..self.height);

            if !self.cells[row][col].is_mine() {
                self.cells[row][col].set_mine(true);
                mines_placed += 1;
            }
        }
    }

    fn calculate_adjacent_numbers(&mut self) {
        for row in 0..self.height {
            for col in 0..self.width {
                if !self.cells[row][col].is_mine() {
                    let count = self.mines_around(col, row);
                    self.cells[row][col].set_adjacent_mines(count);
                }
            }
        }
    }

    fn mines_around(&self, x: usize, y: usize) -> u8 {
        let mut mines = 0;

        let min_row = y.saturating_sub(1);
        let max_row = (y + 1).min(self.height - 1);
        let min_col = x.saturating_sub(1);
        let max_col = (x + 1).min(self.width - 1);

        for row in min_row..=max_row {
            for col in min_col..=max_col {
                if row == y && col == x {
                    continue;
                }
                if self.cells[row][col].is_mine() {
                    mines += 1;
                }
            }
        }

        mines
    }
}
