use std::io::{stdin,stdout,Write};

fn main() {
    let mut board = [[32u8; 7]; 6];
    // 32 is empty spot (space)
    // 82 is red spot (R)
    // 89 is yellow spot (Y)

    let mut player_turn: u8 = 1;
    // 1 is player 1
    // 2 is player 2
    // etc

    loop {
        display_board(&board);

        let mut col: usize = 0;
        let mut lowest_row: usize = 0;

        loop {
            loop {
                println!("Player {}: Choose a collumn from 1 to 7", player_turn);

                col = match get_input().parse::<usize>() {
                    Ok(col) => col,
                    Err(_e) => 255,
                };

                col -= 1;

                if col >= 7 {
                    println!("Not a valid number");
                } else {
                    break;
                }
            }
            
            // lowest_row = 0;
            for row in board {
                if row[col] == 32 {
                    lowest_row += 1;
                } else {
                    break;
                }
            }

            if lowest_row == 0 {
                println!("Collumn full!");
            } else {
                break;
            }
        }

        lowest_row -= 1;

        let winner = check_win(&board);

        if winner == 'R' {
            println!("Player 1 won");
            break;
        } else if winner == 'Y' {
            println!("Player 2 won");
            break;
        }

        if player_turn == 1 {
            board[lowest_row][col] = 82;
            player_turn = 2;
        } else if player_turn == 2 {
            board[lowest_row][col] = 89;
            player_turn = 1;
        }
    }
}

// returns input from the terminal
fn get_input() -> String {
    let mut s = String::new();
    let _ = stdout().flush();
    stdin().read_line(&mut s).expect("Did not enter a valid string");
    if let Some('\n')=s.chars().next_back() {
        s.pop();
    }
    if let Some('\r')=s.chars().next_back() {
        s.pop();
    }

    return s;
}

// outputs the board to the screen
fn display_board(board: &[[u8; 7]; 6]) {
    print!("{}[2J", 27 as char);
    for row in board {
        for spot in row {
            print!("| {} ", *spot as char);
        }
        println!("|");
        for i in 0..row.len() {
            print!("|___");
        }
        println!("|");
    }
}

// checks the board for any four in a row and return the color (R, Y) that won
fn check_win(board: &[[u8; 7]; 6]) -> char {
    for row in 0..(board.len()-3) {
        for col in 0..(board[row].len()-3) {
            // vertical four in a row
            if
                board[row][col] == board[row+1][col] &&
                board[row+1][col] == board[row+2][col] &&
                board[row+2][col] == board[row+3][col]
            {
                return board[row][col] as char;
            }

            // horizontal four in a row
            if
                board[row][col] == board[row][col+1] &&
                board[row][col+1] == board[row][col+2] &&
                board[row][col+2] == board[row][col+3]
            {
                return board[row][col] as char;
            }

            // diagonal four in a row
            if
                board[row][col] == board[row+1][col+1] &&
                board[row+1][col+1] == board[row+2][col+2] &&
                board[row+2][col+2] == board[row+3][col+3]
            {
                return board[row][col] as char;
            }
        }
    }
    return ' ';
}