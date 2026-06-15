use crate::board::Board;
use crate::input::*;
use crate::render::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameState {
    PLAYING,
    WON,
    LOST,
    QUIT,
}

pub struct Game {
    board: Board,
    state: GameState,
}

impl Game {
    pub fn new() -> Self {
        Game {
            board: Board::default(),
            state: GameState::PLAYING,
        }
    }
    pub fn start(&mut self) {
        let board_width =
            read_usize_in_range("Width of the board", Board::MIN_WIDTH, Board::MAX_WIDTH);

        let board_height =
            read_usize_in_range("Height of the board", Board::MIN_HEIGHT, Board::MAX_HEIGHT);

        let min_mines = 1;
        let max_mines = (board_width - 1) * (board_height - 1);

        let mines = read_usize_in_range(
            "How many mines do you want on the board",
            min_mines,
            max_mines,
        );

        self.board = Board::new(board_width, board_height, mines as u16);
        self.board.generate_board();

        render(&self.board);

        while self.state == GameState::PLAYING {
            self.play_turn();
            clear_before_render(board_height + 5);
            render(&self.board);
            self.check_game_state();
        }

        self.end_game();
    }

    fn play_turn(&mut self) {
        loop {
            match get_player_action() {
                Ok(PlayerAction::Quit) => {
                    self.state = GameState::QUIT;
                    break;
                }
                Ok(PlayerAction::Flag { x, y }) => {
                    self.board.flag_cell(x, y);
                    break;
                }
                Ok(PlayerAction::Reveal { x, y }) => {
                    self.board.reveal_cell(x, y);
                    break;
                }
                Err(err) => {
                    println!("{}", err);
                }
            }
        }
    }

    fn check_game_state(&mut self) {
        if self.state == GameState::QUIT {
            return;
        }
        self.state = self.board.check_game_state();
    }

    fn end_game(&mut self) {
        let height = self.board.height() + 4;
        clear_before_render(height);
        if self.state == GameState::WON {
            render(&self.board);
            println!("Congratulations, you won the game!");
        } else {
            self.board.reveal_all_cells();
            render(&self.board);
            let msg = match self.state {
                GameState::LOST => "You lost!",
                _ => "Goodbye!",
            };
            println!("{}", msg);
        }
    }
}
