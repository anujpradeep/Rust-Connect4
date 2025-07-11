use crate::game::{Board, GameState, COLS};
use crate::player::Player;

#[derive(Clone, Copy, PartialEq, Debug)]

pub struct AI {
    pub depth: usize,
}

impl AI {
    pub fn new(diff: usize) -> Self {
        let depth = match diff {
            1 => 4,
            2 => 6,
            3 => 8,
            10 => 10, // Test a strong AI
            _ => 6,   // default depth for any other value
        };
        AI { depth: depth }
    }
    pub fn best_move(self, board: &Board) -> Option<usize> {
        let mut alpha: i32 = i32::MIN + 1;
        let beta: i32 = i32::MAX;
        let mut best_col: Option<usize> = None;
        let mut best_score: i32 = i32::MIN + 1;

        let ai_player = board.get_current_player(); // Save this for evaluation perspective

        for col in 0..COLS {
            // Simulate move
            let mut temp_board = board.clone();
            if temp_board.play_move(col).is_some() {
                let raw_score =
                    self.negamax(&temp_board, self.depth - 1, -beta, -alpha, &ai_player);

                let score = if raw_score == i32::MIN {
                    i32::MIN + 1
                } else {
                    -raw_score
                };

                if score > best_score {
                    best_score = score;
                    best_col = Some(col);
                }

                alpha = alpha.max(score);
            }
        }

        best_col
    }

    fn negamax(
        &self,
        board: &Board,
        depth: usize,
        mut alpha: i32,
        beta: i32,
        ai_player: &Player,
    ) -> i32 {
        // board is a copy of the main board. The AI will make moves on it and the state of the game will be changed

        if board.get_game_state() != &GameState::InProgress || depth == 0 {
            let score = board.clone().evaluate(*ai_player);
            return score;
        }

        let mut max_score = i32::MIN + 1;

        for col in 0..COLS {
            let mut temp_board = board.clone();
            if let Some(_) = temp_board.play_move(col) {
                let raw_score = self.negamax(
                    &temp_board,
                    depth - 1,
                    -beta,
                    -alpha,
                    &temp_board.get_current_player(),
                );

                let score = if raw_score == i32::MIN {
                    i32::MIN + 1
                } else {
                    -raw_score
                };

                max_score = max_score.max(score);
                alpha = alpha.max(score);

                if alpha >= beta {
                    break; // β cut-off
                }
            }
        }
        return max_score;
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_best_move_basic() {
        let mut board = Board::new_human_vs_human();

        board.play_move(0); // X
        board.play_move(1); // O
        board.play_move(0); // X
        board.play_move(1); // O

        let ai = AI { depth: 5 };
        let best_col = ai.best_move(&board);

        assert!(best_col.is_some()); // Should return a valid column
        assert!(best_col.unwrap() < COLS); // Must be within bounds
    }
    #[test]
    fn test_ai_blocks_win() {
        let mut board = Board::new_human_vs_human();

        // X X X _ (AI should place in col 3 to block)
        board.play_move(0); // X
        board.play_move(4); // O
        board.play_move(1); // X
        board.play_move(4); // O
        board.play_move(2); // X

        let ai = AI { depth: 5 };
        let best = ai.best_move(&board);

        assert_eq!(best, Some(3));
    }
}
