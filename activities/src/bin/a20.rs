

// Topic: User input
//
// Requirements:
// * Verify user input against pre-defined keywords
// * The keywords represent possible power options for a computer:
//   * Off
//   * Sleep
//   * Reboot
//   * Shutdown
//   * Hibernate
// * If the user enters one of the keywords, a message should be printed to
//   the console indicating which action will be taken
//   * Example: If the user types in "shutdown" a message should display such
//     as "shutting down"
// * If the keyword entered does not exist, an appropriate error message
//   should be displayed
//
// Notes:
// * Use an enum to store the possible power states
// * Use a function with a match expression to print out the power messages
//   * The function should accept the enum as an input
// * Use a match expression to convert the user input into the power state enum
// * The program should be case-insensitive (the user should be able to type
//   Reboot, reboot, REBOOT, etc.)

use std::io;
enum State {
    Off,
    Sleep,
    Reboot,
    Shutdown,
    Hibernate,
}

fn user_input() -> io::Result<String> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    Ok(buffer.trim().to_owned())
}

fn print(input: State) {
    match input {
        State::Off => println!("PC is switching off"),
        State::Sleep => println!("PC is going to sleep"),
        State::Reboot => println!("PC is going to reboot"),
        State::Shutdown => println!("PC is shutting down"),
        State::Hibernate => println!("PC is going into hibernation mode"),
        _ => ()

    }
}
fn main() {
    println!("Please type the action for the PC");
    let user_input = user_input();
    match user_input {
        Ok(input) => match input.to_lowercase().as_str() {
            "off" => print(State::Off),
            "sleep" => print(State::Sleep),
            "reboot" => print(State::Reboot),
            "shutdown" => print(State::Shutdown),
            "hibernate" => print(State::Hibernate),
            _ => (),
        }
        Err(err) => println!("{}", err)
    }
}
