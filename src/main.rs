extern crate console;
use console::style;

extern crate dialoguer;
use dialoguer::Select;

mod day_1;

fn main() {
    let _ = puzzle_prompt();
}

fn puzzle_prompt() -> Result<bool, Box<std::error::Error>> {
    println!("Advent of {}!", style("Code").yellow());

    let mut select = Select::new();
    select.with_prompt("Which part?");
    select.items(&["Part 1", "Part 2"]);
    select.default(0);
    let result = select.interact();
    match result {
        Ok(0) => day_1::frequency(),
        Ok(1) => day_1::frequency_match(),
        _ => panic!("Unknown part"),
    };

    Ok(true)
}
