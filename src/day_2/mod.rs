use console::style;
use std::collections::HashMap;

const INPUT: &str = include_str!("input.txt");

pub fn _part_1() {
    let count_2s = INPUT
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

    let count_3s = INPUT
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
    let lines = INPUT.lines();
    let other_lines = INPUT.lines();
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

pub fn part_2_v2() {
    let mut line_minus_1: HashMap<String, &str> = HashMap::new();
    let last = INPUT.lines().find_map(|line| {
        for i in 0..line.chars().count() {
            let sans = line
                .char_indices()
                .filter(|c| c.0 != i)
                .map(|c| c.1)
                .collect::<String>();
            if line_minus_1.get(&sans).is_some() && line_minus_1[&sans] != line {
                return Some(sans);
            } else {
                line_minus_1.insert(sans, line);
            }
        }
        None
    });

    match last {
        Some(last) => println!("Common chars: {}", style(last).green()),
        None => println!("{}", style("No match found!").red()),
    }
}
