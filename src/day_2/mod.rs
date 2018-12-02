use console::style;
use std::collections::HashMap;

pub fn part_1() {
    let input = include_str!("input.txt");

    let count_2s = input
        .lines()
        .filter(|l| {
            let chars = l.chars();
            let mut counts: HashMap<char, i32> = HashMap::new();
            for c in chars {
                let count = counts.entry(c).or_insert(0);
                *count += 1;
            }
            counts.values().any(|v| *v == 2)
        }).count();

    let count_3s = input
        .lines()
        .filter(|l| {
            let chars = l.chars();
            let mut counts: HashMap<char, i32> = HashMap::new();
            for c in chars {
                let count = counts.entry(c).or_insert(0);
                *count += 1;
            }
            counts.values().any(|v| *v == 3)
        }).count();

    println!("Found checksum: {}", style(count_2s * count_3s).green())
}

pub fn part_2() {
    let input = include_str!("input.txt");
    let lines = input.lines();
    let other_lines = input.lines();
    for line in lines {
        for other_line in other_lines.clone() {
            let mut diff_chars = 0;
            for c in other_line.char_indices() {
                if Some(c.1) != line.chars().nth(c.0) {
                    diff_chars += 1;
                }
            }

            if diff_chars == 1 {
                println!("Line {} and line {}", line, other_line);
                let common_chars = line
                    .char_indices()
                    .filter(|c| other_line.chars().nth(c.0) == Some(c.1))
                    .map(|c| c.1)
                    .collect::<String>();
                println!("Common chars: {}", style(common_chars).green());
                return;
            }
        }
    }
}
