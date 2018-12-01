pub fn frequency() {
    println!("Calculating frequency...");

    let input = include_str!("input.txt");

    println!("File size read {}", input.len());

    let lines = input.split("\n");

    let frequency = lines.fold(0, |current, line| {
        current + line
            .parse::<i32>()
            .unwrap_or_else(|err| panic!("Failed to parse line! {}", err))
    });

    println!("Final frequency: {}", frequency);
}
