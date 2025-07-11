use connect4_core::Board;
use std::io;

fn read_input(prompt: &str, lower: usize, upper: usize) -> usize {
    loop {
        println!("{}", prompt);
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_ok() {
            if let Ok(value) = input.trim().parse::<usize>() {
                if value >= lower && value <= upper {
                    return value;
                }
            }
        }
        println!(
            "Please enter a valid number between {} and {}.",
            lower, upper
        );
    }
}
fn main() {
    let selection_str = "Enter The type of game you want to play (Enter a number from 1-3):
        Play a Human vs Human (Enter 1)
        Play a Human vs AI (Enter 2)
        Play a AI vs AI (Enter 3)";

    let ai_selection_str =
        "Enter The Difficulty of AI you want to play against (Enter a number from 1-3):
                Easy (Enter 1)
                Medium (Enter 2)
                Hard (Enter 3)";

    let create_board = match read_input(selection_str, 1, 3) {
        1 => Some(Board::new_human_vs_human()),
        2 => {
            let ai_diff = read_input(ai_selection_str, 1, 3);
            Some(Board::new_human_vs_ai(ai_diff))
        }
        3 => {
            let ai_diff1 = read_input(ai_selection_str, 1, 3);

            let ai_diff2 = read_input(ai_selection_str, 1, 3);

            Some(Board::new_ai_vs_ai(ai_diff1, ai_diff2))
        }
        _ => None,
    };

    if let Some(mut board) = create_board {
        board.play_game();
    } else {
        println!("The number entered is Invalid");
    }
}
