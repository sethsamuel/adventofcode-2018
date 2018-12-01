pub fn frequency() {
    println!("Calculating frequency...");

    let input = include_str!("input.txt");

    println!("File size read {}", input.len());

    let lines = input.split('\n');

    let frequency = lines.fold(0, |current, line| {
        current + line
            .parse::<i32>()
            .unwrap_or_else(|err| panic!("Failed to parse line! {}", err))
    });

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
