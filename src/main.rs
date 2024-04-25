use std::{fmt::LowerExp, io::{stdin,stdout,Write}};
use rand::Rng;

fn main() {
    let mut board = [[32u8; 7]; 6];
    // 32 is empty spot (space)
    // 82 is red spot (R)
    // 89 is yellow spot (Y)

    let mut player: char = 'R';
    // 1 is player 1
    // 2 is player 2
    // etc

    loop {
        display_board(&board);

        let mut col: usize = 0;

        if (player == 'Y') {
            ai_move(&mut board, player);
        } else {
            loop {
                loop {
                    println!("Player {}: Choose a collumn from 1 to 7", player);
    
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

                if play_move(&mut board, col, player) {
                    break;
                }
            }
        }

        if player == 'R' {
            player = 'Y';
        } else {
            player = 'R'
        }

        if check_four_in_a_row(&board, 'R') {
            display_board(&board);
            println!("Player 1 won");
            break;
        } else if check_four_in_a_row(&board, 'Y') {
            display_board(&board);
            println!("Player 2 won");
            break;
        } else {
            println!("Space won");
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
        for _i in 0..row.len() {
            print!("|___");
        }
        println!("|");
    }
}

fn play_move(board: &mut [[u8; 7]; 6], col: usize, player: char) -> bool {
    let mut lowest_row: i8 = -1;
    for i in 0..6 {
        if board[i][col] == b' ' {
            lowest_row += 1;
        } else {
            break;
        }
    }

    if lowest_row == -1 {
        return false;
    }

    if board[lowest_row as usize][col] != b' ' {
        return false;
    }

    board[lowest_row as usize][col] = player as u8;
    return true;
}

// checks the board for any four in a row and return the color (R, Y) that won
fn check_four_in_a_row(board: &[[u8; 7]; 6], player: char) -> bool {
    // vertical four in a row check
    for row in 0..(board.len()-3) {
        for col in 0..(board[row].len()) {
            if
                board[row][col] == board[row+1][col] &&
                board[row+1][col] == board[row+2][col] &&
                board[row+2][col] == board[row+3][col]
            {
                if board[row][col] == player as u8 {
                    return true;
                }
            }
        }
    }

    // horizontal four in a row check
    for row in 0..(board.len()) {
        for col in 0..(board[row].len()-3) {
            if
                board[row][col] == board[row][col+1] &&
                board[row][col+1] == board[row][col+2] &&
                board[row][col+2] == board[row][col+3]
            {
                if board[row][col] == player as u8 {
                    return true;
                }
            }
        }
    }

    // diagonal left to right four in a row check
    for row in 0..(board.len()-3) {
        for col in 0..(board[row].len()-3) {
            if
                board[row][col] == board[row+1][col+1] &&
                board[row+1][col+1] == board[row+2][col+2] &&
                board[row+2][col+2] == board[row+3][col+3]
            {
                if board[row][col] == player as u8 {
                    return true;
                }
            }
        }
    }

    // diagonal right to left four in a row check
    for row in 0..(board.len()-3) {
        for col in 3..(board[row].len()) {
            if
                board[row][col] == board[row+1][col-1] &&
                board[row+1][col-1] == board[row+2][col-2] &&
                board[row+2][col-2] == board[row+3][col-3]
            {
                if board[row][col] == player as u8 {
                    return true;
                }
            }
        }
    }
    return false;
}

fn ai_move(board: &mut [[u8; 7]; 6], player: char) {
    let mut tmp_board = board.clone();
    let other_player: char;

    if player == 'R' {
        other_player = 'Y';
    } else {
        other_player = 'R';
    }

    // checks if the ai can win
    for col in 0..7 {
        tmp_board = board.clone();
        play_move(&mut tmp_board, col, player);

        if check_four_in_a_row(&tmp_board, player) {
            play_move(board, col, player);
            return;
        }
    }

    // checks if the other player could win and block it
    for col in 0..7 {
        tmp_board = board.clone();
        play_move(&mut tmp_board, col, other_player);

        if check_four_in_a_row(&tmp_board, other_player) {
            play_move(board, col, player);
            return;
        }
    }

    let mut rng = rand::thread_rng();
    let num = rng.gen_range(0..7);
    play_move(board, num, player);
}