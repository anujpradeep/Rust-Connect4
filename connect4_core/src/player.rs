use std::fmt;

use crate::ai::AI;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum PlayerId {
    One,
    Two,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum PlayerType {
    Human,
    AI(AI),
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Player {
    pub id: PlayerId,
    pub kind: PlayerType,
}

impl fmt::Display for PlayerId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return match self {
            PlayerId::One => write!(f, "1"),
            PlayerId::Two => write!(f, "2"),
        };
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Player {} is a {}",
            match self.id {
                PlayerId::One => 1,
                PlayerId::Two => 2,
            },
            match &self.kind {
                PlayerType::Human => "Human",
                PlayerType::AI(_ai) => "AI",
            }
        )
    }
}
