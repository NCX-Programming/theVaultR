// Crates
extern crate termion;
extern crate text_io;
// Includes
use termion::event::Key;
use termion::input::TermRead;
use text_io::read;
//use termion::style; <-- may need in the future
use std::{io::{Write, stdout, stdin}, i32};

pub fn guess_my_number() {
    let stdin = stdin();
    let mut stdout = stdout();

    write!(stdout,
        "{}{}Welcome to Guess My Number! (theVaultR Edition)\
        {}1. Play\
        {}2. Exit
        {}",
        termion::clear::All,
        termion::cursor::Goto(1, 1),
        termion::cursor::Goto(1, 2),
        termion::cursor::Goto(1, 3),
        termion::cursor::Goto(1, 4))
        .unwrap();
    stdout.flush().unwrap();

    for c in stdin.keys() {
        write!(stdout,
            "{}{}",
            termion::cursor::Goto(1, 5),
            termion::clear::CurrentLine)
            .unwrap();

        match c.unwrap() {
            Key::Char('1') => {
                set_up_game();
                break;
            },
            Key::Char('2') => break,
            _ => {}
        }
    }
}

fn set_up_game() {
    let stdin = stdin();
    let mut stdout = stdout();

    let mut menu_done = 0;
    let mut max_number = 100;
    let mut min_number = 0;
    
    write!(stdout,
        "{}{}Game Options\
        {}1. Max Number: {}\
        {}2. Min Number: {}\
        {}3. Play!
        {}",
        termion::clear::All,
        termion::cursor::Goto(1, 1),
        termion::cursor::Goto(1, 2),
        max_number,
        termion::cursor::Goto(1, 3),
        min_number,
        termion::cursor::Goto(1, 4),
        termion::cursor::Goto(1, 5))
        .unwrap();
    stdout.flush().unwrap();

    for c in stdin.keys() {
        write!(stdout,
            "{}{}",
            termion::cursor::Goto(1, 5),
            termion::clear::CurrentLine)
            .unwrap();

        
        match c.unwrap() {
            Key::Char('1') => {
                menu_done = 0;
                max_number = set_max_number();
                break;
            },
            Key::Char('2') => {
                menu_done = 0;
                min_number = set_min_number();
                break;
            },
            Key::Char('3') => {
                run_game(max_number, min_number);
                break;
            },
            _ => {}
        }
    }
}

fn set_max_number() -> i32 {
    let mut stdout = stdout();

    let mut max_number = 0;

    while max_number == 0 {
        write!(stdout,
            "{}{}Enter a new maximum number:{}",
        termion::clear::All,
        termion::cursor::Goto(1, 1),
        termion::cursor::Goto(1, 2))
            .unwrap();
        let max_input: String = read!("{}\n");
        max_number = max_input.parse::<i32>().unwrap();
    }

    return max_number;
}

fn set_min_number() -> i32 {
    let mut stdout = stdout();

    let mut min_number = 0;

    while min_number == 0 {
        write!(stdout,
            "{}{}Enter a new maximum number:{}",
        termion::clear::All,
        termion::cursor::Goto(1, 1),
        termion::cursor::Goto(1, 2))
            .unwrap();
        let max_input: String = read!("{}\n");
        min_number = max_input.parse::<i32>().unwrap();
    }

    return min_number;
}

fn run_game(max_number: i32, min_number: i32) {
    print!("hi");
}