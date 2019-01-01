extern crate rand;

use rand::Rng;
use std::io;
use rand::thread_rng;
use self::Element::*;
use self::Operation::*;

#[derive(Copy, Clone, PartialEq, Debug)]
enum Element {
    Flag,
    Number(u8),
    Explosion,
    QuestionMark,
    Mine,
    Unknown
}

#[derive(Debug)]
enum Operation {
    Click,
    FlagOp,
    QuestionMarkOp
}

#[derive(Debug, PartialEq)]
enum State {
    Won, Lose, Playing
}

pub fn main() {
    let size = 8;
    let mut board = vec![vec![Unknown; size as usize]; size as usize];
    let solution = create_solution(size);

    println!("# Solution #");
    print_board(&solution);

    let mut state = State::Playing;

    while state == State::Playing {

        let mut operation = ask_operation();
        while operation.is_none() {
            operation = ask_operation();
        }

        let mut location = ask_location(size as u8);
        while location.is_none() {
            location = ask_location(size as u8);
        }

        operate(&mut board, &operation.unwrap(), location.unwrap(), &solution, &mut state);

        println!("# {:?} #", state);
        print_board(&board);
    }

    match state {
        State::Won => println!("YOU WON ! =D"),
        State::Lose => println!("YOU LOST, TRY AGAIN ! D="),
        _ => {}
    }
}

fn ask_operation() -> Option<Operation> {
    println!("Write operation: C (Click), F (Flag), Q (Question Mark)");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Woops");
    return match input.trim().as_ref() {
        "C" => Some(Click),
        "F" => Some(FlagOp),
        "Q" => Some(QuestionMarkOp),
        other => {
            println!("{} is not a valid option", other);
            return None;
        }
    }
}

fn ask_location(size: u8) -> Option<(u8, u8)> {
    println!("Write location as: X Y");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Woops");
    let clean_input: Vec<Result<u8, _>> = input.trim().split_whitespace().map(|v| v.parse()).collect();
    return match clean_input.as_slice() {
        [Ok(x), Ok(y)] if x < &0 || y < &0 || x >= &size || y >= &size => {
            println!("Invalid coordinate");
            return None;
        }
        [Ok(x), Ok(y)] => Some((*x, *y)),
        _ => {
            println!("{:?} does not conform with the format: X Y", input.trim());
            return None;
        }
    }
}

fn operate(board: &mut Vec<Vec<Element>>, operation: &Operation, location: (u8, u8), solution: &Vec<Vec<Element>>, state: &mut State ) {
    let (x, y) = location;
    let current = board[y as usize][x as usize];
    match (operation, current) {
        (FlagOp, Unknown) => board[y as usize][x as usize] = Flag,
        (QuestionMarkOp, Unknown) => board[y as usize][x as usize] = QuestionMark,
        (Click, Unknown) => click(board, location, solution, state),
        (QuestionMarkOp, QuestionMark) | (FlagOp, Flag) => board[y as usize][x as usize] = Unknown,
        _ => println!("Invalid Operation {:?} for the position {}, {}", operation, x, y)
    }
    if is_solved(&board, &solution) {
        *state = State::Won;
    }
}

fn is_valid_pos(x: i8, y: i8, size: i8) -> bool {
    return x >= 0 && y >= 0 && x < size && y < size;
}

fn click(board: &mut Vec<Vec<Element>>, location: (u8, u8), solution: &Vec<Vec<Element>>, state: &mut State) {
    let (x, y) = location;
    let guess = board[y as usize][x as usize];
    let real = solution[y as usize][x as usize];
    match (guess, real) {
        (Unknown, Number(_)) => {
            board[y as usize][x as usize] = real;
            if is_solved(&board, &solution) {
                *state = State::Won;
            }
            for xx in -1..2 {
                for yy in -1..2 {
                    if xx != 0 || yy != 0 {
                        let x_pos = x as i8 + xx;
                        let y_pos = y as i8 + yy;

                        if is_valid_pos(x_pos, y_pos, board.len() as i8) {
                            match solution[y_pos as usize][x_pos as usize] {
                                Number(0) => click(board, (x_pos as u8, y_pos as u8), solution, state),
                                _ => {}
                            }
                        }
                    }
                }
            }
        },
        (Unknown, Mine) => {
            board[y as usize][x as usize] = Explosion;
            *state = State::Lose;
        },
        _ => { }
    }
}

fn print_board(board: &Vec<Vec<Element>>) {
    println!("  {}", (0..8).map(|n| format!("{}", n)).collect::<Vec<_>>().join(""));
    board
        .iter()
        .enumerate()
        .for_each(|(row_num, row)| {
            let txt: Vec<String> = row.iter().map(|e| elem_to_string(e)).collect();
            println!("{} {}", row_num, txt.join(""))
        });
}

fn elem_to_string(elem: &Element) -> String {
    return match elem {
        Flag => String::from("F"),
        QuestionMark => String::from("?"),
        Unknown => String::from(" "),
        Explosion => String::from("#"),
        Mine => String::from("*"),
        Number(n) => format!("{}", n)
    }
}

fn create_solution(size: usize) -> Vec<Vec<Element>> {
    let mut board = vec![vec![Unknown; size]; size];
    for _ in 0..5 {
        let mut rng = thread_rng();

        let x = rng.gen_range(0, size);
        let y = rng.gen_range(0, size);

        board[y][x] = Element::Mine;
    }
    for y in 0..size {
        for x in 0..size {
            match board[y][x] {
                Element::Mine => {},
                _ => board[y][x] = Number(get_mines(x as i8, y as i8, &board))
            }
        }
    }
    return board;
}

fn get_mines(x: i8, y: i8, board: &Vec<Vec<Element>>) -> u8 {
    let mut amount: u8 = 0;
    for xx in -1..=1 {
        for yy in -1..=1 {
            if xx != 0 || yy != 0 {
                let x_pos: i8 = x + xx;
                let y_pos: i8 = y + yy;

                if is_valid_pos(x_pos, y_pos, board.len() as i8) && board[y_pos as usize][x_pos as usize] == Mine {
                    amount+=1;
                }
            }
        }
    }
    return amount;
}

fn is_solved(board: &Vec<Vec<Element>>, solution: &Vec<Vec<Element>>) -> bool {
    board
        .iter()
        .zip(solution)
        .all(|(board_row, sol_row)|
            board_row
                .iter()
                .zip(sol_row)
                .all(|(board_e, sol_e)| *board_e == *sol_e || (*board_e == Flag && *sol_e == Mine)))
}