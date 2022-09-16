extern crate termion;

pub fn hello_world() {
    println!("{}{}Hello, world!{}",
            termion::clear::All,
            termion::cursor::Goto(1, 1),
            termion::cursor::Goto(1, 1));
}