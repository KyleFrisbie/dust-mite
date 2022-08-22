mod story_manager;
use story_manager::{introduction, tutorial};
mod user_command;

fn main() {
    introduction();
    tutorial();
}
