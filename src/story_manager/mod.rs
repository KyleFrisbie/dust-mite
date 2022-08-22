use crate::user_command::{
    get_user_input, get_valid_action, get_valid_response, VALID_AFFIRMATIVE_RESPONSES,
};

pub fn introduction() {
    println!("Welcome to Dust Plight!")
}

pub fn tutorial() {
    // General Tutorial Intro
    println!("This tutorial will guide you through a basic scenario, like you might encounter while playing the game.
            // \nIn each state of the game, you will be presented with an environment described to you based on your current position. It will be up to you to determine your next action based on this description. Are you interested in running the tutorial?");


    let response = get_user_input(&|input| get_valid_response(input.to_string()));
    if VALID_AFFIRMATIVE_RESPONSES.contains(&response.as_str()) {
        println!("Great! Let's get started with the tutorial.")
    }

    println!("Excellent! Now that you're ready to go, let's practice some movement.");
    get_user_input(&|input| get_valid_action(input.to_string()));
}
