extern crate termion;

pub fn guess_my_number() {
    println!("{}{}Hello, world!{}",
            termion::clear::All,
            termion::cursor::Goto(1, 1),
            termion::cursor::Goto(1, 1));
}