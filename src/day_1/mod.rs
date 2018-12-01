extern crate indicatif;
use self::indicatif::ProgressBar;

pub fn frequency() {
    println!("Calculating frequency...");

    let input = include_str!("input.txt");

    let lines = input.split('\n');
    let line_count = lines.clone().count();

    let progress_bar = ProgressBar::new(line_count as u64);

    let frequency = progress_bar.wrap_iter(lines).fold(0, |current, line| {
        current + line
            .parse::<i32>()
            .unwrap_or_else(|err| panic!("Failed to parse line! {}", err))
    });
    progress_bar.finish();

    println!("Final frequency: {}", frequency);
}

use std::collections::HashMap;
pub fn frequency_match() {
    let input = include_str!("input.txt");
    let mut current = 0;
    let mut found = false;
    let mut loop_count = 0;
    let mut hash: HashMap<i32, bool> = HashMap::new();
    hash.insert(current, true);
    while !found {
        let lines = input.split('\n');
        for line in lines {
            current += line
                .parse::<i32>()
                .unwrap_or_else(|err| panic!("Failed to parse line! {}", err));
            if hash.get(&current).is_some() {
                //FOUND!
                found = true;
                println!(
                    "First repeated frequency {} after {} loops",
                    current, loop_count
                );
                break;
            } else {
                hash.insert(current, true);
            }
        }
        loop_count += 1;
    }
}
