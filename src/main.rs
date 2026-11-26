use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};
use std::{write, io::{self}};

mod tic_tac_toe;

#[derive(Debug, Display, EnumIter)]
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

    let is_playing = true;
    let mut board = [" ", " ", " ", " ", " ", " ", " ", " ", " "];

    let mut isPlayerX = true;

    while is_playing {
        clearscreen::clear().expect("Failed to clear screen");
        tic_tac_toe::print_board(board);
        
        if isPlayerX {
            player_play("X");
        } else {
            player_play("O");
        }
    }
}

fn player_play(player: &str, ) {
    println!("\n\n Please write where you want to place your {} \n", player);

    for position in BoardPositions::iter() {
        println!("{}", position);
    }
}
