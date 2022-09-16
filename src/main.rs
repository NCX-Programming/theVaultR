extern crate termion;

mod helloworld;

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::style;
use std::io::{Write, stdout, stdin};

fn main() {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    write!(stdout,
            "{}{}{}Welcome to theVaultR!{}\
            {}1. Hello, World!\
            {}q. Exit\
            {}",
            termion::clear::All,
            termion::cursor::Goto(1, 1),
            style::Bold,
            style::Reset,
            termion::cursor::Goto(1, 2),
            termion::cursor::Goto(1, 3),
            termion::cursor::Goto(1, 4))
            .unwrap();
    stdout.flush().unwrap();

    for c in stdin.keys() {
        write!(stdout,
               "{}{}",
               termion::cursor::Goto(1, 4),
               termion::clear::CurrentLine)
                .unwrap();

        match c.unwrap() {
            Key::Char('q') => break,
            Key::Char('1') => {
                helloworld::hello_world();
                break;
            },
            _ => {}
        }
        stdout.flush().unwrap();
    }
}
