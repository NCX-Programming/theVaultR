// Crates
extern crate termion;
// Local modules
mod helloworld;
mod guessmynumber;
// Includes
use termion::event::Key;
use termion::input::TermRead;
use termion::style;
use std::io::{Write, stdout, stdin};

fn main() {
    let stdin = stdin();
    let mut stdout = stdout();

    println!("{}{}{}Welcome to theVaultR!{}\n\
            1. Hello, World!\n\
            2. Guess My Number\n\
            q. Exit",
            termion::clear::All,
            termion::cursor::Goto(1, 1),
            style::Bold,
            style::Reset);

    for c in stdin.keys() {
        write!(stdout,
               "{}{}",
               termion::cursor::Goto(1, 5),
               termion::clear::CurrentLine)
                .unwrap();

        match c.unwrap() {
            Key::Char('q') => break,
            Key::Char('1') => {
                helloworld::hello_world();
                break;
            },
            Key::Char('2') => {
                guessmynumber::guess_my_number();
                break;
            },
            _ => {}
        }
        stdout.flush().unwrap();
    }
}
