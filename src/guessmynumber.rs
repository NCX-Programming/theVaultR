// Crates
extern crate termion;
// Includes
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
//use termion::style; <-- may need in the future
use std::{io::{Write, stdout, stdin}, i32};

pub fn guess_my_number() {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

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
    let mut stdout = stdout().into_raw_mode().unwrap();

    let mut menuDone = 0;
    let mut maxNumber = 100;
    let mut minNumber = 0;

    write!(stdout,
        "{}{}Game Options\
        {}1. Max Number: {}\
        {}2. Min Number: {}\
        {}3. Play!
        {}",
        termion::clear::All,
        termion::cursor::Goto(1, 1),
        termion::cursor::Goto(1, 2),
        maxNumber,
        termion::cursor::Goto(1, 3),
        minNumber,
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
                menuDone = 0;
                write!(stdout,
                    "{}{}Enter a new maximum number:{}",
                termion::clear::All,
                termion::cursor::Goto(1, 1),
                termion::cursor::Goto(1, 2))
                    .unwrap();
                break;
            },
            Key::Char('2') => break,
            Key::Char('3') => {
                run_game(maxNumber, minNumber);
                break;
            },
            _ => {}
        }
    }
}

fn run_game(maxNumber: i32, minNumber: i32) {
    print!("hi");
}