use std::io;
use rand::Rng;
use colored::*;
use std::process;
use crossterm::{ExecutableCommand, terminal, cursor};

fn main() {
    let mut board: Vec<&str> = vec!["1", "2", "3", "4", "5", "6", "7", "8", "9"];
    let win_conditions: [[usize; 3]; 8] = [
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

    let mut player_move: usize;
    let mut bot_move: usize;
    
    // main game loop
    loop {
        println!("Choose a cell do draw X:");
        let mut player_input = String::new();
        io::stdin().read_line(&mut player_input).expect("cannot read a line");
        player_move = match player_input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Not a number");
                continue;
            },
        };
        
        // check if player makes a correct move
        match player_move < 1 && player_move > 9 {        
            true => {
                println!("Out of bounds, enter a correct cell number");
                continue;
            },
            false => {},
        }
        match is_occupied(player_move, &board) {
            true => {
                println!("This cell is already occupied, choose an empty one");
                continue;
            },
            false => {},
        };

        // player move
        board[player_move - 1] = "X";

        check_win_conditions(&board, &win_conditions);

        // bot move
        loop {
            bot_move = rand::thread_rng().gen_range(1..=9);
            match is_occupied(bot_move, &board) {
                true => { continue },
                false => {
                    board[bot_move - 1] = "O";
                    render(&board);
                    break;
                },
            };
        }

        check_win_conditions(&board, &win_conditions);
    }
}

fn colorize(cell: &str) -> ColoredString {
    match cell {
        "X" => cell.blue(),
        "O" => cell.red(),
        _ => cell.white()
    }
}

fn render(board: &Vec<&str>) {
    io::stdout().execute(terminal::Clear(terminal::ClearType::All)).unwrap();
    io::stdout().execute(cursor::MoveTo(0, 1)).unwrap();
    println!(" {} | {} | {} \n---+---+---\n {} | {} | {} \n---+---+---\n {} | {} | {} ",
    colorize(board[0]),
    colorize(board[1]),
    colorize(board[2]),
    colorize(board[3]),
    colorize(board[4]),
    colorize(board[5]),
    colorize(board[6]),
    colorize(board[7]),
    colorize(board[8]));
}

fn is_occupied(position: usize, board: &Vec<&str>) -> bool {
    if board[position - 1] == "X" || board[position - 1] == "O" {
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
    if k == 9 {
        return true;
    }
    return false;
}

fn check_win_conditions(board: &Vec<&str>, win_conditions: &[[usize; 3]; 8]) {
    if is_full(board) {
       render(board);
       println!("A draw!");
       call_menu();
    }
    for i in win_conditions {
        if board[i[0]] == board[i[1]] && board[i[1]] == board[i[2]] {
            if board[i[0]] == "X" {
                render(board);
                println!("You win!");
                call_menu();
            } else if board[i[0]] == "O"{
                println!("You lose :(");
                call_menu();
            }
        }
    }
}

fn call_menu() {
    println!("Hit ENTER to play again or (q) to exit:");
    let mut menu_input = String::new();
    io::stdin().read_line(&mut menu_input).expect("cannot read a line");
    if menu_input.trim() == "q" {
        process::exit(0);
    }
    main();
}

