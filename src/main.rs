use std::io;
use rand::Rng;
use colored::*;
use std::process;
use crossterm::{ExecutableCommand, terminal, cursor};

fn main() {
    let mut board: Vec<&str> = vec!["1", "2", "3", "4", "5", "6", "7", "8", "9"];
    let win_conditions: [[i32; 3]; 8] = [
        [0, 1, 2],
        [3, 4, 5],
        [6, 7, 8],
        [0, 3, 6],
        [1, 4, 7],
        [2, 5, 8],
        [0, 4, 8],
        [2, 4, 6]
    ];

    render(&board);

    let mut player_move: i32;
    let mut bot_move: i32;
    loop {
        println!("Choose a cell do draw X:");
        let mut player_input = String::new();
        io::stdin().read_line(&mut player_input).expect("cannot read a line");
        player_move = match player_input.trim().parse() {
            Ok(num) => {
                if num >= 1 && num <= 9 {
                    num
                } else {
                    println!("Out of bounds, enter a correct cell number");
                    continue;
                }
            },
            Err(_) => {
                println!("Not a number");
                continue;
            },
        };
        
        match is_occupied(player_move, &board) {
            true => {
                println!("This cell is already occupied, choose an empty one");
                continue;
            },
            false => {},
        };

        // player move
        board[player_move as usize - 1] = "X";


        check_win_conditions(&board, win_conditions);

        // bot move
        loop {
            bot_move = rand::thread_rng().gen_range(1..=9);
            match is_occupied(bot_move, &board) {
                true => {
                    if is_full(&board) {
                        println!("board is full");
                    } else {
                        continue;
                    }
                },
                false => {
                    board[bot_move as usize - 1] = "O";
                    render(&board);
                    break;
                },
            };
        }

        check_win_conditions(&board, win_conditions);
    }
}

fn render(board: &Vec<&str>) {
    io::stdout().execute(terminal::Clear(terminal::ClearType::All)).unwrap();
    io::stdout().execute(cursor::MoveTo(0, 1)).unwrap();
    println!(" {} | {} | {} \n---+---+---\n {} | {} | {} \n---+---+---\n {} | {} | {} ",
    if board[0] == "X" {board[0].blue()} else if board[0] == "O" {board[0].red()} else {board[0].white()},
    if board[1] == "X" {board[1].blue()} else if board[1] == "O" {board[1].red()} else {board[1].white()},
    if board[2] == "X" {board[2].blue()} else if board[2] == "O" {board[2].red()} else {board[2].white()},
    if board[3] == "X" {board[3].blue()} else if board[3] == "O" {board[3].red()} else {board[3].white()},
    if board[4] == "X" {board[4].blue()} else if board[4] == "O" {board[4].red()} else {board[4].white()},
    if board[5] == "X" {board[5].blue()} else if board[5] == "O" {board[5].red()} else {board[5].white()},
    if board[6] == "X" {board[6].blue()} else if board[6] == "O" {board[6].red()} else {board[6].white()},
    if board[7] == "X" {board[7].blue()} else if board[7] == "O" {board[7].red()} else {board[7].white()},
    if board[8] == "X" {board[8].blue()} else if board[8] == "O" {board[8].red()} else {board[8].white()});
}

fn is_occupied(position: i32, board: &Vec<&str>) -> bool {
    if board[position as usize - 1] == "X" || board[position as usize - 1] == "O" {
        return true;
    }
    return false;
}

fn is_full(board: &Vec<&str>) -> bool {
    let mut k: i32 = 0;
    for i in board {
        if *i == "X" || *i == "O" {
            k += 1;
        }
    }
    if k >= 9 {
        return true;
    }
    return false;
}

fn check_win_conditions(board: &Vec<&str>, win_conditions: [[i32; 3]; 8]) {
   for i in &win_conditions {
       if board[i[0] as usize] == board[i[1] as usize] && board[i[1] as usize] == board[i[2] as usize] {
           if board[i[0] as usize] == "X" {
               render(&board);
               println!("You win!");
               menu();
           } else if board[i[0] as usize] == "O"{
               // render(&board);
               println!("You lose :(");
               menu();
           }
       }
   }
   if is_full(&board) {
       render(&board);
       println!("A draw!");
       menu();
   }
}

fn menu() {
    println!("Hit ENTER to play again or (q) to exit:");
    let mut tmp = String::new();
    io::stdin().read_line(&mut tmp).expect("cannot read a line");
    if tmp.trim() == "q" {
        process::exit(0);
    }
    main();
}

