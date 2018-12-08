#![allow(dead_code)]
extern crate console;
use console::style;

extern crate dialoguer;
use dialoguer::Select;

mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;

use std::time::Instant;
fn main() {
    // let _ = puzzle_prompt();
    {
        let now = Instant::now();
        day_7::part_2();
        println!(
            "Elapsed: {}.{}",
            now.elapsed().as_secs(),
            now.elapsed().subsec_millis()
        );
    }
}

fn _puzzle_prompt() -> Result<bool, Box<std::error::Error>> {
    println!("Advent of {}!", style("Code").yellow());

    let mut select = Select::new();
    select.with_prompt("Which part?");
    select.items(&["Part 1", "Part 2"]);
    select.default(0);
    let result = select.interact();
    match result {
        Ok(0) => day_1::_frequency(),
        Ok(1) => day_1::_frequency_match(),
        _ => panic!("Unknown part"),
    };

    Ok(true)
}
