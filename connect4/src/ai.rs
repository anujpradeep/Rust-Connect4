use crate::game::{Board, GameState};
use crate::player::Player;
pub struct AI {
    pub diff: usize,
}

impl AI {
    pub fn new(self, diff: usize) -> Self {
        AI { diff: diff }
    }
    pub fn best_move(self, board: &Board, depth: usize) -> Option<usize> {
        let alpha = i32::MIN;
        let beta = i32::MAX;
        let score = self.negamax(
            &board.clone(),
            depth,
            alpha,
            beta,
            board.get_current_player(),
        );
        None
    }

    fn negamax(&self, board: &Board, depth: usize, alpha: i32, beta: i32, player: &Player) -> i32 {
        // board is a copy of the main board. The AI will make moves on it and the state of the game will be changed

        if board.get_game_state() != &GameState::InProgress || depth == 0 {
            return board.evaluate(player);
        }

        return 0;
    }
}
