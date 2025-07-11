use crate::ai::AI;
use crate::player::{Player, PlayerId, PlayerType};
use std::{fmt, io};

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
            GameState::Win(player) => write!(f, "Player {} has won!", player.id),
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

        let col_chr = [
            '0', ' ', '1', ' ', '2', ' ', '3', ' ', '4', ' ', '5', ' ', '6', '\n', '\n',
        ];

        seq.push('\n');

        for row in 0..ROWS {
            for col in 0..COLS {
                let ch = match self.board[col][row] {
                    Cell::Empty => '_',
                    Cell::Player(player) => match player.id {
                        PlayerId::One => 'X',
                        PlayerId::Two => 'O',
                    },
                };
                seq.push(ch);
                seq.push(' ');
            }
            seq.push('\n');
        }
        for c in col_chr {
            seq.push(c);
        }
        write!(f, "{}", seq)
    }
}

impl Board {
    pub fn new(player1_type: PlayerType, player2_type: PlayerType) -> Self {
        let players = [
            Player {
                id: PlayerId::One,
                kind: player1_type,
            },
            Player {
                id: PlayerId::Two,
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

    pub fn new_ai_vs_human(diff: usize) -> Self {
        Board::new(PlayerType::AI(AI::new(diff)), PlayerType::Human)
    }

    /// Player 1 is Human, Player 2 is AI
    pub fn new_human_vs_ai(diff: usize) -> Self {
        Board::new(PlayerType::Human, PlayerType::AI(AI::new(diff)))
    }

    /// Both players are Human
    pub fn new_human_vs_human() -> Self {
        Board::new(PlayerType::Human, PlayerType::Human)
    }

    /// Both players are AI
    pub fn new_ai_vs_ai(diff1: usize, diff2: usize) -> Self {
        Board::new(
            PlayerType::AI(AI::new(diff1)),
            PlayerType::AI(AI::new(diff2)),
        )
    }

    pub fn play_game(&mut self) {
        let start_game_msg = |current_player: Player, players: [Player; 2]| {
            println!("Welcome to Connect 4");
            println!("There are 2 players playing");

            for player in players {
                println!("{}", player);
            }

            println!(
                "Player {} is going First, They are {}",
                match current_player.id {
                    PlayerId::One => 1,
                    PlayerId::Two => 2,
                },
                match current_player.id {
                    PlayerId::One => "X",
                    PlayerId::Two => "O",
                }
            );
        };

        let start_turn_msg = |current_player: Player| {
            println!(
                "Player {}'s Turn, They are {}",
                match current_player.id {
                    PlayerId::One => 1,
                    PlayerId::Two => 2,
                },
                match current_player.id {
                    PlayerId::One => "X",
                    PlayerId::Two => "O",
                }
            );
        };

        let end_turn_msg = |player_move: Option<Move>, board: &mut Board| {
            match player_move {
                Some(player_move) => {
                    println!("{}", player_move);
                }
                None => {}
            };

            println!("{}", board.clone());

            if !matches!(board.get_game_state(), GameState::Win(_)) {
                println!("{}", board.get_game_state());
            }
        };

        let end_game_msg = |board: &mut Board| {
            println!("{}", board.get_game_state());

            println!("The final Board is...\n\n{}", board.clone());
        };

        let index = (rand::random::<f32>() * self.players.len() as f32).floor() as usize; // chooses which player goes first
        self.current_player = self.players[index];

        start_game_msg(self.current_player.clone(), self.players.clone());

        while self.game_state == GameState::InProgress {
            let col: usize;
            start_turn_msg(self.current_player.clone());

            if let PlayerType::AI(ai) = &self.current_player.kind {
                let best_move = ai.best_move(&self);
                match best_move {
                    Some(m) => col = m,
                    None => {
                        println!("AI Could not find a good move");
                        col = 8 // Invalid number
                    }
                }
            } else {
                let mut input = String::new();
                println!("Enter your move (Enter a number from 0-6):");
                io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read input");

                col = input.trim().parse().expect("Please enter a valid number");

                if !self.valid_move(col) {
                    continue;
                }
            }

            let player_move: Option<Move> = self.play_move(col);

            end_turn_msg(player_move.clone(), self);
        }

        end_game_msg(self);
    }

    pub fn play_move(&mut self, col: usize) -> Option<Move> {
        if !self.valid_move(col) {
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
                self.check_win();
                return player_move;
            }
        }

        return None;
    }

    pub fn valid_move(&mut self, col: usize) -> bool {
        if col >= COLS {
            return false;
        }

        for row in (0..ROWS).rev() {
            if self.board[col][row] == Cell::Empty {
                return true;
            }
        }

        return false;
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
                    // Check Horizontal
                    if col + WINNING_LENGTH <= COLS
                        && (0..WINNING_LENGTH)
                            .all(|i| self.board[col + i][row] == Cell::Player(player))
                    {
                        self.game_state = GameState::Win(player);
                        return;
                    }

                    // Check Vertical
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
     * The window will be a 4 by 4 window that will check the Horizontal, Vertical, and diagonally.
     */
    fn score_window(window: &[Cell], player: Player) -> i32 {
        let opponent_id = match player.id {
            PlayerId::One => PlayerId::Two,
            PlayerId::Two => PlayerId::One,
        };

        let opponent = Player {
            id: opponent_id,
            kind: player.kind,
        };

        let mut score = 0;
        let two_in_a_row: i32 = 500;
        let three_in_a_row: i32 = 5000;
        let four_in_a_row: i32 = 100000;
        let block_score: i32 = 4000;

        let count_player = window
            .iter()
            .filter(|&&c| c == Cell::Player(player))
            .count();
        let count_opponent = window
            .iter()
            .filter(|&&c| c == Cell::Player(opponent))
            .count();
        let count_empty = window.iter().filter(|&&c| c == Cell::Empty).count();

        match (count_player, count_opponent, count_empty) {
            (4, 0, 0) => score += four_in_a_row,
            (3, 0, 1) => score += three_in_a_row,
            (2, 0, 2) => score += two_in_a_row,
            (0, 3, 1) => score -= block_score, // block opponent threat
            _ => {}
        }

        return score;
    }

    /**
     * This function will evaluate a board's state for the current player.
     * This is used to assign a scoring to a move done by the AI but can
     * also be used to show the game state to the players like in chess.
     *
     * Pieces in the center = 3 points for each
     * 2 in a row in any direction + 2 empty = 500 points for each
     * 3 in a row in any direction + 1 empty = 5000 points for each
     * 4 in a row = 100000 points
     */
    pub fn evaluate(self, player: Player) -> i32 {
        let mut score: i32 = 0;
        let center_score: i32 = 3;

        // Center Points
        let center_col = COLS / 2;
        let center_count = self.board[center_col]
            .iter()
            .filter(|&&cell| cell == Cell::Player(player))
            .count();

        score += (center_count as i32) * center_score;

        // Evaluate all 4-cell windows
        for row in 0..ROWS {
            for col in 0..COLS {
                // Horizontal
                if col + 3 < COLS {
                    let window = (0..4).map(|i| self.board[col + i][row]).collect::<Vec<_>>();
                    score += Self::score_window(&window, player);
                }

                // Vertical
                if row + 3 < ROWS {
                    let window = (0..4).map(|i| self.board[col][row + i]).collect::<Vec<_>>();
                    score += Self::score_window(&window, player);
                }

                // Diagonal /
                if col + 3 < COLS && row >= 3 {
                    let window = (0..4)
                        .map(|i| self.board[col + i][row - i])
                        .collect::<Vec<_>>();
                    score += Self::score_window(&window, player);
                }

                // Diagonal \
                if col + 3 < COLS && row + 3 < ROWS {
                    let window = (0..4)
                        .map(|i| self.board[col + i][row + i])
                        .collect::<Vec<_>>();
                    score += Self::score_window(&window, player);
                }
            }
        }

        return score;
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
impl Board {
    fn test_drop_col(&mut self, col: usize, row: usize, player: Player) -> Option<Move> {
        if self.board[col][row] == Cell::Empty {
            let player_move = Some(Move {
                player: player,
                position: Position { row, col },
            });

            self.board[col][row] = Cell::Player(player);
            self.check_win();
            return player_move;
        }
        return None;
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

    fn play_move_draw(board: &mut Board, moves: Vec<Vec<usize>>) {
        let player = *board.get_current_player();
        for (row, cols) in moves.iter().enumerate() {
            for &col in cols {
                board.test_drop_col(col, row, player);
            }
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
    fn test_play_move() {
        let mut board = Board::new(PlayerType::Human, PlayerType::Human);
        let pos = board.play_move(0);
        assert!(pos.is_some());

        let pos = pos.unwrap();

        assert_eq!(pos.position.col, 0);
        assert_eq!(pos.position.row, ROWS - 1);
        assert_ne!(board.board[0][ROWS - 1], Cell::Empty);
    }

    #[test]
    fn test_invalid_column() {
        let mut board = Board::new(PlayerType::Human, PlayerType::Human);
        assert_eq!(board.play_move(COLS), None);
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
        let mut board = Board::new_human_vs_human();

        let moves_player1: Vec<Vec<usize>> = vec![
            vec![0, 1, 2, 4, 5, 6],
            vec![3],
            vec![0, 1, 5],
            vec![2, 3, 6],
            vec![0, 1, 4, 5],
            vec![0, 1, 4, 6],
        ];

        let moves_player2: Vec<Vec<usize>> = vec![
            vec![3],
            vec![0, 1, 2, 4, 5, 6],
            vec![2, 3, 4, 6],
            vec![0, 1, 4, 5],
            vec![2, 3, 6],
            vec![2, 3, 5],
        ];

        play_move_draw(&mut board, moves_player1);
        board.change_current_player();
        play_move_draw(&mut board, moves_player2);
        assert_eq!(board.get_game_state(), &GameState::Draw);
    }
}
