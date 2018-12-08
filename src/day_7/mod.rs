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

struct Worker {
    on_step: Option<char>,
    finished_at: u32,
}

pub fn part_2() {
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
    let mut tick = 0;
    let worker_count = 5;
    let work_base_time = 60;
    let mut workers: Vec<Worker> = (0..worker_count)
        .map(|_| Worker {
            on_step: None,
            finished_at: 0,
        }).collect();
    let total_steps = steps.keys().count();
    println!("Total steps {}", total_steps);

    while ordered_steps.len() < total_steps {
        println!("TICK {}", tick);
        for worker in &mut workers {
            if worker.finished_at <= tick {
                //Finish previous step
                if let Some(finished_step) = worker.on_step {
                    println!("Finished step {}", finished_step);
                    ordered_steps.push(finished_step);
                    worker.on_step = None;

                    for step in steps.values_mut() {
                        *step = step
                            .into_iter()
                            .filter(|c| *c != &finished_step)
                            .map(|c| *c)
                            .collect::<PrevSteps>();
                    }
                }
            }
        }
        for worker in &mut workers {
            if worker.on_step.is_none() {
                //Get next step
                let steps_copy = steps.clone();
                let mut next_steps = steps_copy
                    .keys()
                    .filter(|k| steps.get(k).unwrap().len() == 0)
                    .collect::<Vec<&char>>();
                next_steps.sort();

                if next_steps.len() > 0 {
                    let mut next_step = next_steps[0];
                    steps.remove(next_step);
                    worker.on_step = Some(*next_step);
                    let work_time = work_base_time + (*next_step as u32 - 64) as u32; //u32 for A is 64
                    worker.finished_at = tick + work_time;
                    println!(
                        "Starting work on letter {} finished in {}",
                        worker.on_step.unwrap(),
                        work_time
                    );

                    println!("Steps {:?}", steps);
                }
            }
        }

        tick += 1;
    }

    println!(
        "Steps order {}",
        ordered_steps.into_iter().collect::<String>()
    );
    //Last tick is extra
    println!("Took {} ticks", tick - 1);
}
