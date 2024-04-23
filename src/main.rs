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
        display_board(board);

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

        if player_turn == 1 {
            board[lowest_row][col] = 82;
            player_turn = 2;
        } else if player_turn == 2 {
            board[lowest_row][col] = 89;
            player_turn = 1;
        }
    }
}

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

fn display_board(board: [[u8; 7]; 6]) {
    print!("{}[2J", 27 as char);
    for row in board {
        for spot in row {
            print!("| {} ", spot as char);
        }
        println!("|");
        for i in 0..row.len() {
            print!("|___");
        }
        println!("|");
    }
}