use std::io::{self, BufRead};

use clap::Parser;

pub static VALID_AFFIRMATIVE_RESPONSES: [&str; 2] = ["yes", "y"];
pub static VALID_NEGATIVE_RESPONSES: [&str; 2] = ["no", "n"];
static VALID_ACTIONS: [&str; 1] = ["move"];

pub fn get_valid_response(input: String) -> (String, bool) {
    let response_is_valid = is_valid(&input, &VALID_AFFIRMATIVE_RESPONSES)
        || is_valid(&input, &VALID_NEGATIVE_RESPONSES);
    if response_is_valid {
        return (input, true);
    }
    return (
        format!(
            "Please provide a valid response: {}, {}.",
            VALID_AFFIRMATIVE_RESPONSES.join(", "),
            VALID_NEGATIVE_RESPONSES.join(", ")
        ),
        false,
    );
}

pub fn get_valid_action(input: String) -> (String, bool) {
    let action_is_valid = is_valid(&input, &VALID_ACTIONS);
    if action_is_valid {
        return (input, true);
    }
    return (
        format!(
            "Please provide a valid action: {}.",
            VALID_ACTIONS.join(", ")
        ),
        false,
    );
}

pub fn get_user_input(f: &dyn Fn(&String) -> (String, bool)) -> String {
    let stdin = io::stdin();

    let mut buffer = String::with_capacity(2048);
    // Lock our standard input to eliminate synchronization overhead (unlocks when dropped)
    let mut stdin = io::stdin().lock();

    // Read our first line.
    stdin.read_line(&mut buffer);

    let mut result = f(&buffer);
    while !result.1 {
        println!("{}", result.0);
        stdin.read_line(&mut buffer);
        result = f(&buffer);
        buffer.clear();
    }

    return result.0;
}

fn is_valid(command: &str, valid_commands: &[&str]) -> bool {
    return valid_commands.contains(&command.trim());
}
