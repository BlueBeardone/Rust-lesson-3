pub fn print_board(board: [&str; 9]) {
    println!();
    for row in 0..3 {
        let column = row * 3;
        println!(" {} | {} | {} ", board[column], board[column+1], board[column+2]);
        if row < 2 {
            println!("---+---+---");
        }
    }
}
