use colored::Colorize;

use crate::{
    board::Board,
    cell::{Cell, CellState::*},
};

pub fn render(board: &Board) {
    let width = board.width();
    let height = board.height();
    let cells = board.cells();

    let digits = width.max(height).checked_ilog10().unwrap_or(0) as usize + 2;

    let mut buffer = String::new();

    buffer.push_str(&format!("{}|", " ".repeat(digits)));
    for col in 1..=width {
        buffer.push_str(&format!("{:^width$}", col, width = digits));
    }
    buffer.push_str("|\n");

    buffer.push_str(&format!(
        "{}|{}|\n",
        "-".repeat(digits),
        "-".repeat(digits).repeat(width)
    ));

    for row in 0..height {
        buffer.push_str(&format!("{:^width$}|", row + 1, width = digits));
        for col in 0..width {
            buffer.push_str(&get_cell_symbol(&cells[row][col], digits));
        }
        buffer.push_str("|\n");
    }

    buffer.push_str(&format!(
        "{}|{}|\n",
        "-".repeat(digits),
        "-".repeat(digits).repeat(width)
    ));

    println!("{}", buffer);
}

pub fn clear_before_render(height: usize) {
    print!("\x1b[{}A", height);
    print!("\x1b[J");
}

fn get_cell_symbol(cell: &Cell, width: usize) -> String {
    match cell.state() {
        HIDDEN => {
            let block = format!("{:^width$}", "█", width = width);
            format!("{}", block.bright_black())
        }
        FLAGGED => format!("{:^width$}", "⚑", width = width),
        REVEALED => {
            if cell.is_mine() {
                return format!("{:^width$}", "☼", width = width);
            }

            let adj_mines = cell.adjacent_mines();
            if adj_mines > 0 {
                let num_str = format!("{:^width$}", adj_mines, width = width);
                let colored = match adj_mines {
                    1 => num_str.bold().blue(),
                    2 => num_str.bold().green(),
                    3 => num_str.bold().red(),
                    4 => num_str.bold().truecolor(0, 0, 81),
                    5 => num_str.bold().truecolor(165, 42, 42),
                    6 => num_str.bold().cyan(),
                    7 => num_str.bold().black(),
                    _ => num_str.bold().truecolor(80, 80, 80),
                };
                return format!("{}", colored);
            }

            let empty_str = format!("{:^width$}", " ", width = width);
            format!("{}", empty_str)
        }
    }
}
