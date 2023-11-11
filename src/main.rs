use clearscreen::clear;
use regex::Regex;
use std::io::{stdout, Write};
use std::process::exit;
use std::thread;
use std::time::Duration;
use std::{io, println};

const GAME_FILE_LOCATION: &str = "./game_file.ron";

pub mod game_lib;

fn main() {
    let world_result = init_game(GAME_FILE_LOCATION);

    match world_result {
        Ok(world) => {
            // Here we will run the game
            do_game(world);
        }
        Err(file_err) => {
            println!("Error: {}", file_err);
        }
    }
}
fn init_game(file_location: &str) -> Result<game_lib::World, std::io::Error> {
    //Here we will read the file and return the world we created.

    game_lib::World::read_from_file(file_location)
}

fn do_game(mut world: game_lib::World) {
    clear().expect("Failed to clear screen");
    println!("Hello, Player!\n");
    println!("Welcome to Rust In Peace\n");
    println!("Would you like to start the game? (Y/N)");
    let mut answer = String::new();
    io::stdin()
        .read_line(&mut answer)
        .expect("Failed to read input");
    let no = Regex::new("[nN]|[nN][oO]").unwrap();
    if no.is_match(answer.trim()) {
        println!("Goodbye!");
        std::process::exit(0);
    }
    clear().expect("Failed to clear screen");

    let message="You find yourself lost in a gloomy forest. You see a column of smoke rising in the sky. It seems to be very far away.\n";
    for c in message.chars() {
        print!("{}", c);
        stdout().flush().unwrap(); // Flush the output to make it appear immediately
        thread::sleep(Duration::from_millis(25)); // Delay between characters
    }