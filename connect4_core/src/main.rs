use crate::game::Board;
mod ai;
mod game;
mod player;

fn main() {
    let mut board = Board::new_human_vs_human();
    println!("{}", board);
}
