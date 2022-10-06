// Includes
use termion::event::Key;
use termion::input::TermRead;
use text_io::read;
use rand::{thread_rng, Rng};
//use termion::style; <-- may need in the future
use std::{io::{Write, stdout, stdin}, i32, thread, time};

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
    let mut total_guesses = 10;
    
    while menu_done == 0 {
        println!("{}{}Game Options\n\
                1. Max Number: {}\n\
                2. Min Number: {}\n\
                3. Guesses: {}\n\
                4. Play!",
                termion::cursor::Goto(1, 1),
                termion::clear::All,
                max_number,
                min_number,
                total_guesses);

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
                println!("{}{}Enter a new number of guesses:",
                    termion::clear::All,
                    termion::cursor::Goto(1, 1));
                let total_guesses_input: String = read!("{}\n");
                total_guesses = total_guesses_input.parse::<i32>().unwrap();
            },
            "4" => {
                menu_done = 1;
                run_game(max_number, min_number, total_guesses);
            },
            _ => {}
        }
    }
}

fn run_game(max_number: i32, min_number: i32, total_guesses: i32) {
    let mut game_finished = 0;
    let mut guesses_used = 0;
    let mut guess_string: String;
    let random_number = thread_rng().gen_range(min_number..=max_number);

    while game_finished == 0 {
        println!("{}{}I'm thinking of a number between {} and {}!",
            termion::clear::All,
            termion::cursor::Goto(1, 1),
            min_number, max_number);
        println!("Enter your guess:");
        
        guess_string = read!("{}\n");

        match guess_string.parse::<i32>() {
            Ok(n) => {
                guesses_used += 1;
                if n == random_number {
                    game_finished = 1;
                    break;
                } else {
                    if guesses_used >= total_guesses {
                        game_finished = -1;
                        break;
                    }
                    if n > random_number {
                        println!("Too high! Try again!");
                    } else {
                        println!("Too low! Try again!");
                    }
                    thread::sleep(time::Duration::from_millis(1500));
                }
            },
            _ => {
                println!("Invalid guess!");
                thread::sleep(time::Duration::from_millis(1500));
            }
        }
    }

    if game_finished == -1 {
        println!("{}{}You lose!\n\
            You couldn't guess the number in {} guesse(s).",
            termion::clear::All,
            termion::cursor::Goto(1, 1),
            guesses_used);
    } else {
        println!("{}{}You win!\n\
            You guessed the number in {} guesse(s)!",
            termion::clear::All,
            termion::cursor::Goto(1, 1),
            guesses_used);
    }
}