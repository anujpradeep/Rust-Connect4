use std::fmt;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum PlayerId {
    Player1,
    Player2,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum PlayerType {
    Human,
    AI,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Player {
    pub id: PlayerId,
    pub kind: PlayerType,
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.id {
            PlayerId::Player1 => write!(f, "Player 1"),
            PlayerId::Player2 => write!(f, "Player 2"),
        }
    }
}
