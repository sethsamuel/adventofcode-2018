const INPUT: &str = include_str!("input.txt");
// const INPUT: &str = include_str!("small_input.txt");

type Rule = Vec<char>;

use std::collections::HashMap;
fn parse_line(line: &str) -> (Rule, char) {
    let mut parts = line.split(" => ");
    (
        parts.next().unwrap().chars().collect::<Vec<char>>(),
        parts.next().unwrap().chars().next().unwrap(),
    )
}
const PLANT: char = '#';
const NO_PLANT: char = '.';
pub fn part_1() {
    //First line
    let mut lines = INPUT.lines();
    let first_line = lines.next().unwrap();
    let initial_state = first_line.split(' ').last().unwrap().chars();
    println!(
        "Initial state: {:?}",
        &initial_state.clone().collect::<String>()
    );

    lines.next(); //Blank line
    let rules = lines.map(parse_line);
    let mut rule_hash = HashMap::<Rule, char>::new();
    for rule in rules {
        rule_hash.insert(rule.0, rule.1);
    }
    let max_generation = 20;
    //Plants can grow up to 4 to the left and 4 to the right
    let mut last_state = vec!['.'; 4];
    last_state.append(&mut initial_state.clone().collect::<Vec<_>>());
    last_state.append(&mut vec!['.'; 4]);
    let mut total_plants = initial_state.clone().filter(|c| *c == PLANT).count();
    for _i in 0..max_generation {
        // println!("{}", last_state.clone().into_iter().collect::<String>());
        let mut next_state: Vec<char> = vec!['.'; last_state.len()];
        for i in 2..next_state.len() - 2 {
            next_state[i] = *rule_hash
                .get(&last_state[i - 2..=i + 2])
                .unwrap_or(&NO_PLANT);
        }
        last_state = vec!['.'; 4];
        last_state.append(&mut next_state);
        last_state.append(&mut vec!['.'; 4]);
        total_plants += last_state
            .clone()
            .into_iter()
            .filter(|c| *c == PLANT)
            .count();
    }
    let offset = (max_generation + 1) * 4isize;
    let mut sum = 0;
    for i in 0..last_state.len() {
        let index = i as isize - offset;
        if last_state[i] == PLANT {
            // println!("Plant at {}", index);
            sum += index;
        }
    }
    println!("Sum of plant indicies grown: {}", sum);
}

extern crate indicatif;
use self::indicatif::ProgressBar;
pub fn part_2() {
    //First line
    let mut lines = INPUT.lines();
    let first_line = lines.next().unwrap();
    let initial_state = first_line.split(' ').last().unwrap().chars();
    println!(
        "Initial state: {:?}",
        &initial_state.clone().collect::<String>()
    );

    lines.next(); //Blank line
    let rules = lines.map(parse_line);
    let mut rule_hash = HashMap::<Rule, char>::new();
    for rule in rules {
        rule_hash.insert(rule.0, rule.1);
    }
    let max_generation = 50_000_000_000i64;
    //Plants can grow up to 4 to the left and 4 to the right
    let mut last_state = vec!['.'; 4];
    last_state.append(&mut initial_state.clone().collect::<Vec<_>>());
    last_state.append(&mut vec!['.'; 4]);
    let progress_bar = ProgressBar::new(max_generation as u64);
    for _i in 0..max_generation {
        progress_bar.inc(1);
        // println!("{}", last_state.clone().into_iter().collect::<String>());
        let mut next_state: Vec<char> = vec!['.'; last_state.len()];
        for i in 2..next_state.len() - 2 {
            next_state[i] = *rule_hash
                .get(&last_state[i - 2..=i + 2])
                .unwrap_or(&NO_PLANT);
        }
        last_state = vec!['.'; 4];
        last_state.append(&mut next_state);
        last_state.append(&mut vec!['.'; 4]);
    }
    progress_bar.finish();
    let offset: i64 = (max_generation + 1) * 4 as i64;
    let mut sum = 0;
    for i in 0..last_state.len() {
        let index = i as i64 - offset;
        if last_state[i] == PLANT {
            // println!("Plant at {}", index);
            sum += index;
        }
    }
    println!("Sum of plant indicies grown: {}", sum);
}
