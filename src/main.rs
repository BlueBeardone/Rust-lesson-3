use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};

mod tic_tac_toe;

#[derive(Debug, Display, EnumIter, Clone, Copy)]
enum BoardPositions {
    #[strum(serialize = "Top Left: 1")]
    TopLeft,
    #[strum(serialize = "Top Center: 2")]
    TopCenter,
    #[strum(serialize = "Top Right: 3")]
    TopRight,
    #[strum(serialize = "Center Left: 4")]
    CenterLeft,
    #[strum(serialize = "Center: 5")]
    Center,
    #[strum(serialize = "Center Right: 6")]
    CenterRight,
    #[strum(serialize = "Bottom Left: 7")]
    BottomLeft,
    #[strum(serialize = "Bottom Center: 8")]
    BottomCenter,
    #[strum(serialize = "Bottom Right: 9")]
    BottomRight
}

fn main() {

    let mut is_playing = true;
    let mut board = [" ", " ", " ", " ", " ", " ", " ", " ", " "];

    let mut is_player_x = true;

    while is_playing {
        clearscreen::clear().expect("Failed to clear screen");
        
        if is_player_x {
            player_play("X", &mut board);
        } else {
            player_play("O", &mut board);
        }

        if let Some(winner) = is_winner(&board) {
            print_winning(&board);
            println!("Player {} wins!", winner);
            is_playing = false;
        } else if board.iter().all(|&x| x != " ") {
            print_winning(&board);
            println!("It's a tie!");
            is_playing = false;
        } else {
            is_player_x = !is_player_x;
        }
        let mut input: String = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read input");
    }

    clearscreen::clear().expect("Failed to clear screen");
}

fn player_play<'a>(player: &'a str, board: &mut [&'a str; 9]) {
    loop {
        clearscreen::clear().expect("Failed to clear screen");
        tic_tac_toe::print_board(*board);
        print!("\n\n Please write the number of the position you want to place your {}: \n\n", player);
    
        for position in BoardPositions::iter() {
            if board[position as usize] == " " {
                println!("{}", position);
            }
        }
        println!("\n");
        let mut input: String = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read input");
        let position: usize = input.trim().parse().expect("Please enter a valid number");
        if is_valid_move(position, board) {
            board[position - 1] = player;
            break;
        } else {
            println!("Invalid move. Please try again.");
            std::io::stdin().read_line(&mut input).expect("Failed to read input");
        }
    }
}

fn is_valid_move(position: usize, board: &[&str; 9]) -> bool {
    if position - 1 > 8 {
        return false;
    }
    if board[position - 1] != " " {
        return false;
    }
    true
}

fn is_winner<'a>(board: &'a [&'a str; 9]) -> Option<&'a str> {
    let winning_combinations = [
        [0, 1, 2],
        [3, 4, 5],
        [6, 7, 8],
        [0, 3, 6],
        [1, 4, 7],
        [2, 5, 8],
        [0, 4, 8],
        [2, 4, 6],
    ];

    for combination in winning_combinations.iter() {
        if board[combination[0]] != " "
            && board[combination[0]] == board[combination[1]]
            && board[combination[1]] == board[combination[2]]
        {
            return Some(board[combination[0]]);
        }
    }
    None
}

fn print_winning(board: &[&str; 9]) {
    clearscreen::clear().expect("Failed to clear screen");
    tic_tac_toe::print_board(*board);
}