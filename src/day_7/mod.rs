const INPUT: &str = include_str!("input.txt");
// const INPUT: &str = include_str!("small_input.txt");

extern crate lazy_static;
use self::lazy_static::lazy_static;
extern crate regex;
use self::regex::Regex;
lazy_static! {
    static ref LINE_RE: Regex =
        Regex::new(r"Step (.) must be finished before step (.) can begin.").unwrap();
}

type PrevSteps = Vec<char>;

use std::collections::HashMap;
pub fn part_1() {
    let rules = INPUT.lines().map(|l| {
        (
            LINE_RE
                .captures(l)
                .unwrap()
                .get(1)
                .unwrap()
                .as_str()
                .chars()
                .next()
                .unwrap(),
            LINE_RE
                .captures(l)
                .unwrap()
                .get(2)
                .unwrap()
                .as_str()
                .chars()
                .next()
                .unwrap(),
        )
    });
    let mut steps: HashMap<char, PrevSteps> = HashMap::new();
    for rule in rules {
        steps.entry(rule.1).or_insert(vec![]).push(rule.0);
        steps.entry(rule.0).or_insert(vec![]);
    }
    println!("Steps {:?}", steps);
    let mut ordered_steps: PrevSteps = vec![];
    while !steps.is_empty() {
        let steps_copy = steps.clone();
        let mut next_steps = steps_copy
            .keys()
            .filter(|k| steps.get(k).unwrap().len() == 0)
            .collect::<Vec<&char>>();
        next_steps.sort();

        let next_step = next_steps[0];

        ordered_steps.push(*next_step);
        steps.remove(next_step);

        for step in steps.values_mut() {
            *step = step
                .into_iter()
                .filter(|c| *c != next_step)
                .map(|c| *c)
                .collect::<PrevSteps>();
        }
        println!("Steps {:?}", steps);
    }

    println!(
        "Steps order {}",
        ordered_steps.into_iter().collect::<String>()
    );
}
pub fn part_2() {}
