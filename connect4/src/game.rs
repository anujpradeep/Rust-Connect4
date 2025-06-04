use crate::player::{Player, PlayerId, PlayerType};
use std::fmt;

pub const ROWS: usize = 6;
pub const COLS: usize = 7;
pub const WINNING_LENGTH: usize = 4;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum GameState {
    InProgress,
    Draw,
    Win(Player),
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Cell {
    Empty,
    Player(Player),
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Position {
    pub row: usize,
    pub col: usize,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Move {
    pub player: Player,
    pub position: Position,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Board {
    board: Vec<Vec<Cell>>,
    game_state: GameState,
    players: [Player; 2],
    current_player: Player,
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Cell::Empty => write!(f, "The Cell is empty"),
            Cell::Player(player) => write!(f, "{} has this Cell!", player),
        }
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Row: {}, Col: {}", self.row, self.col)
    }
}

impl fmt::Display for GameState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GameState::InProgress => write!(f, "The Game is still in progress."),
            GameState::Draw => write!(f, "The game is Draw"),
            GameState::Win(player) => write!(f, "{} has won!", player),
        }
    }
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} has made a Move! They have placed it in {}!",
            self.player, self.position
        )
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut seq = String::new();

        for row in 0..ROWS {
            for col in 0..COLS {
                let ch = match self.board[col][row] {
                    Cell::Empty => '_',
                    Cell::Player(player) => match player.id {
                        PlayerId::Player1 => 'X',
                        PlayerId::Player2 => 'O',
                    },
                };
                seq.push(ch);
            }
            seq.push('\n');
        }
        write!(f, "{}", seq)
    }
}

impl Board {
    pub fn new(player1_type: PlayerType, player2_type: PlayerType) -> Self {
        let players = [
            Player {
                id: PlayerId::Player1,
                kind: player1_type,
            },
            Player {
                id: PlayerId::Player2,
                kind: player2_type,
            },
        ];
        Board {
            board: vec![vec![Cell::Empty; ROWS]; COLS], // This is Column Major instead of Row Major
            game_state: GameState::InProgress,
            players,
            current_player: players[0],
        }
    }

    pub fn play_move(&mut self, col: usize) {
        if self.game_state != GameState::InProgress {
            println!("GAME IS OVER!");
            return;
        }

        let player_move = self.drop_piece(col);

        match player_move {
            Some(dropped_piece) => {
                println!("{}", dropped_piece);
            }
            None => println!(
                "{} made an illegal move! Please try again.",
                self.current_player
            ),
        }

        // Checks for win and Tie
        self.check_win();

        if self.game_state != GameState::InProgress {
            println!("Game is Over!");
            println!("{}", self.game_state);
        }
    }

    pub fn drop_piece(&mut self, col: usize) -> Option<Move> {
        if col >= COLS {
            return None;
        }

        for row in (0..ROWS).rev() {
            if self.board[col][row] == Cell::Empty {
                let player_move = Some(Move {
                    player: self.current_player,
                    position: Position { row, col },
                });

                self.board[col][row] = Cell::Player(self.current_player);
                self.change_current_player();
                return player_move;
            }
        }

        return None;
    }

    fn change_current_player(&mut self) {
        if self.current_player == self.players[0] {
            self.current_player = self.players[1];
        } else {
            self.current_player = self.players[0];
        }
    }

    fn check_win(&mut self) {
        // Check horizontal, vertical, and diagonal wins
        for row in (0..ROWS).rev() {
            for col in 0..COLS {
                if let Cell::Player(player) = self.board[col][row] {
                    // Check Vertical
                    if col + WINNING_LENGTH <= COLS
                        && (0..WINNING_LENGTH)
                            .all(|i| self.board[col + i][row] == Cell::Player(player))
                    {
                        self.game_state = GameState::Win(player);
                        return;
                    }

                    // Check Horizontal
                    if row + WINNING_LENGTH <= ROWS
                        && (0..WINNING_LENGTH)
                            .all(|i| self.board[col][row + i] == Cell::Player(player))
                    {
                        self.game_state = GameState::Win(player);
                        return;
                    }

                    // Check diagonal (bottom-left to top-right)
                    if col + WINNING_LENGTH <= COLS
                        && row + WINNING_LENGTH <= ROWS
                        && (0..WINNING_LENGTH)
                            .all(|i| self.board[col + i][row + i] == Cell::Player(player))
                    {
                        self.game_state = GameState::Win(player);
                        return;
                    }

                    // Check diagonal (top-left to bottom-right)
                    if col + WINNING_LENGTH <= COLS
                        && row >= WINNING_LENGTH - 1
                        && (0..WINNING_LENGTH)
                            .all(|i| self.board[col + i][row - i] == Cell::Player(player))
                    {
                        self.game_state = GameState::Win(player);
                        return;
                    }
                }
            }
        }

        // Check for draw
        if self
            .board
            .iter()
            .all(|col| col.iter().all(|cell| *cell != Cell::Empty))
        {
            self.game_state = GameState::Draw;
        }
    }

    /**
     * This function will evaluate a board's state for the current player.
     * This is used to assign a scoring to a move done by the AI but can
     * also be used to show the game state to the players like in chess.
     *
     * Pieces in the center = 3 points for each
     * 2 in a row in any direction + 2 empty = 10 points for each
     * 3 in a row in any direction + 1 empty = 100 points for each
     * 4 in a row = 100000 points
     */
    fn evaluate(self, player: Player) -> i32 {
        let mut score: i32 = 0;
        let center_score: i32 = 3;
        let two_in_a_row: i32 = 10;
        let three_in_a_row: i32 = 100;
        let four_in_a_row: i32 = 100000;

        // Center Points

        let center_col = COLS / 2;
        let center_count = self.board[center_col]
            .iter()
            .filter(|&&cell| cell == Cell::Player(player))
            .count();

        score += (center_count as i32) * center_score;

        score
    }
    pub fn get_game_state(&self) -> &GameState {
        return &self.game_state;
    }

    pub fn get_board(&self) -> &Vec<Vec<Cell>> {
        return &self.board;
    }

    pub fn get_current_player(&self) -> &Player {
        return &self.current_player;
    }
}

#[cfg(test)]
mod tests {
    use super::*; // Brings Board, Player, etc. into scope

    /**
     * Helper Function
     */
    fn play_moves(board: &mut Board, moves: &[usize]) {
        for &col in moves {
            board.play_move(col);
        }
    }

    #[test]
    fn test_board_initialization() {
        let board = Board::new(PlayerType::Human, PlayerType::Human);

        for col in 0..COLS {
            for row in 0..ROWS {
                assert_eq!(board.board[col][row], Cell::Empty);
            }
        }

        assert_eq!(board.game_state, GameState::InProgress);
    }

    #[test]
    fn test_drop_piece() {
        let mut board = Board::new(PlayerType::Human, PlayerType::Human);
        let pos = board.drop_piece(0);
        assert!(pos.is_some());

        let pos = pos.unwrap();

        assert_eq!(pos.position.col, 0);
        assert_eq!(pos.position.row, ROWS - 1);
        assert_ne!(board.board[0][ROWS - 1], Cell::Empty);
    }

    #[test]
    fn test_invalid_column() {
        let mut board = Board::new(PlayerType::Human, PlayerType::Human);
        assert_eq!(board.drop_piece(COLS), None);
    }

    #[test]
    fn test_vertical_win() {
        let mut board = Board::new(PlayerType::Human, PlayerType::Human);

        /*
         * OUTPUT
         *
         *  _______
         *  _______
         *  X______
         *  XO_____
         *  XO_____
         *  XO_____
         */

        play_moves(&mut board, &[0, 1, 0, 1, 0, 1, 0]);
        assert_eq!(board.get_game_state(), &GameState::Win(board.players[0]))
    }

    #[test]
    fn test_horizontal_win() {
        let mut board = Board::new(PlayerType::Human, PlayerType::Human);

        /*
         * OUTPUT
         *
         *  _______
         *  _______
         *  _______
         *  _______
         *  OOO____
         *  XXXX___
         */

        play_moves(&mut board, &[0, 0, 1, 1, 2, 2, 3]);
        assert_eq!(board.get_game_state(), &GameState::Win(board.players[0]))
    }

    #[test]
    fn test_diagonal_1_win() {
        let mut board = Board::new(PlayerType::Human, PlayerType::Human);

        /*
         * OUTPUT
         *
         *  _______
         *  _______
         *  __OX___
         *  __XX___
         *  _XXO___
         *  XOOO___
         */

        play_moves(&mut board, &[0, 1, 1, 2, 2, 3, 2, 3, 3, 2, 3]);
        assert_eq!(board.get_game_state(), &GameState::Win(board.players[0]))
    }

    #[test]
    fn test_diagonal_2_win() {
        let mut board = Board::new(PlayerType::Human, PlayerType::Human);

        /*
         * OUTPUT
         *
         *  O______
         *  XO_____
         *  OOO____
         *  XXOO___
         *  OOXX_X_
         *  XXXO_X_
         */

        play_moves(
            &mut board,
            &[0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 2, 1, 2, 2, 5, 2, 5, 3, 3, 3],
        );
        assert_eq!(board.get_game_state(), &GameState::Win(board.players[1]))
    }

    #[test]
    fn test_draw() {
        //TODO: Find the correct inputs to create a draw

        let mut board = Board::new(PlayerType::Human, PlayerType::Human);
        let moves = [];

        play_moves(&mut board, &moves);
        // assert_eq!(board.get_game_state(), &GameState::Draw)
        assert!(true);
    }
}
