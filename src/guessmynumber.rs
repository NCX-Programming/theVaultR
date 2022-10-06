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

    println!("{}{}Welcome to Guess My Number! (theVaultR Edition)\n\
            1. Play\n\
            2. Exit",
            termion::clear::All,
            termion::cursor::Goto(1, 1));

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
    let mut menu_done = 0;
    let mut max_number = 100;
    let mut min_number = 0;
    
    while menu_done == 0 {
        println!("{}{}Game Options\n\
                1. Max Number: {}\n\
                2. Min Number: {}\n\
                3. Play!",
                termion::cursor::Goto(1, 1),
                termion::clear::All,
                max_number,
                min_number);

        let menu_choice: String = read!("{}\n");

        match menu_choice.as_str() {
            "1" => {
                println!("{}{}Enter a new maximum number:",
                    termion::clear::All,
                    termion::cursor::Goto(1, 1));
                let max_input: String = read!("{}\n");
                max_number = max_input.parse::<i32>().unwrap();
            },
            "2" => {
                println!("{}{}Enter a new minimum number:",
                    termion::clear::All,
                    termion::cursor::Goto(1, 1));
                let min_input: String = read!("{}\n");
                min_number = min_input.parse::<i32>().unwrap();
            },
            "3" => {
                menu_done = 1;
                run_game(max_number, min_number);
            },
            _ => {}
        }
    }
}

fn run_game(max_number: i32, min_number: i32) {
    println!("{}{}",
            max_number,
            min_number);
}