const INPUT: &str = include_str!("input.txt");
// const INPUT: &str = include_str!("small_input.txt");

struct Action<'a> {
    year: i32,
    month: i32,
    day: i32,
    hour: i32,
    minute: i32,
    event: &'a str,
}

extern crate lazy_static;
use self::lazy_static::lazy_static;
lazy_static! {
    static ref LINE_RE: Regex = Regex::new(r"\[(\d+)-(\d+)-(\d+) (\d+):(\d+)\] (.*)").unwrap();
}
lazy_static! {
    static ref GUARD_RE: Regex = Regex::new(r"#\d+").unwrap();
}

fn parse_line(line: &str) -> Action {
    let captures = LINE_RE.captures(line).unwrap();
    let year = captures.get(1).unwrap().as_str().parse::<i32>().unwrap();
    let month = captures.get(2).unwrap().as_str().parse::<i32>().unwrap();
    let day = captures.get(3).unwrap().as_str().parse::<i32>().unwrap();
    let hour = captures.get(4).unwrap().as_str().parse::<i32>().unwrap();
    let minute = captures.get(5).unwrap().as_str().parse::<i32>().unwrap();
    let event = captures.get(6).unwrap().as_str();

    Action {
        year,
        month,
        day,
        hour,
        minute,
        event,
    }
}

struct Guard<'a> {
    id: &'a str,
    sleeps: Vec<(i32, i32, i32, i32)>, //Month, day, minute start, minute end
    total_minutes: i32,
}

impl<'a> PartialEq for Guard<'a> {
    fn eq(&self, other: &Guard) -> bool {
        self.id == other.id
    }
}

impl<'a> Eq for Guard<'a> {}

// impl<'a> Copy for Guard<'a> {}
// impl<'a> Clone for Guard<'a> {
//     fn clone(&self) -> Guard<'a> {
//         Guard {
//             id: self.id,
//             sleeps: self.sleeps.clone(),
//             total_minutes: self.total_minutes,
//         }
//     }
// }

extern crate regex;
use self::regex::Regex;
use std::collections::HashMap;
pub fn part_1() {
    let mut actions = INPUT.lines().collect::<Vec<&str>>();
    actions.sort();
    let mut guards: HashMap<String, Guard> = HashMap::new();
    // let mut guards: Vec<Guard> = vec![];
    let mut done = false;
    let mut iter_actions = actions.into_iter().peekable();
    while !done {
        if let Some(action) = iter_actions.next() {
            let action = parse_line(action);

            let id = GUARD_RE
                .captures(action.event)
                .unwrap()
                .get(0)
                .unwrap()
                .as_str();
            let mut current_guard = guards.entry(id.to_string()).or_insert(Guard {
                id,
                sleeps: vec![],
                total_minutes: 0,
            });
            while iter_actions.peek().is_some() && parse_line(*iter_actions.peek().unwrap())
                .event
                .starts_with("falls")
            {
                let sleep = parse_line(iter_actions.next().unwrap());
                let wake = parse_line(iter_actions.next().unwrap());
                current_guard
                    .sleeps
                    .push((sleep.month, sleep.day, sleep.minute, wake.minute));
                println!(
                    "Guard {} slept another {} minutes",
                    current_guard.id,
                    wake.minute - sleep.minute
                );
                current_guard.total_minutes += wake.minute - sleep.minute;
            }
        } else {
            done = true;
        }
    }
    let longest_sleep = guards
        .values()
        .into_iter()
        .max_by_key(|g| g.total_minutes)
        .unwrap();
    println!("Longest sleep by {}", longest_sleep.id);
    let mut minutes_slept: HashMap<i32, i32> = HashMap::new();
    for sleep in longest_sleep.sleeps.clone().into_iter() {
        for m in sleep.2..sleep.3 {
            let minute = minutes_slept.entry(m).or_insert(0);
            *minute += 1;
        }
    }
    println!("Minute totals {:?}", minutes_slept);
    let most_slept_minute = minutes_slept
        .keys()
        .max_by_key(|m| minutes_slept[m])
        .unwrap();
    println!("Minutes slept {:?}", longest_sleep.sleeps);
    println!("Most slept minute {}", most_slept_minute);
}
pub fn part_2() {
    let mut actions = INPUT.lines().collect::<Vec<&str>>();
    actions.sort();
    let mut guards: HashMap<String, Guard> = HashMap::new();
    // let mut guards: Vec<Guard> = vec![];
    let mut done = false;
    let mut iter_actions = actions.into_iter().peekable();
    while !done {
        if let Some(action) = iter_actions.next() {
            let action = parse_line(action);

            let id = GUARD_RE
                .captures(action.event)
                .unwrap()
                .get(0)
                .unwrap()
                .as_str();
            let mut current_guard = guards.entry(id.to_string()).or_insert(Guard {
                id,
                sleeps: vec![],
                total_minutes: 0,
            });
            while iter_actions.peek().is_some() && parse_line(*iter_actions.peek().unwrap())
                .event
                .starts_with("falls")
            {
                let sleep = parse_line(iter_actions.next().unwrap());
                let wake = parse_line(iter_actions.next().unwrap());
                current_guard
                    .sleeps
                    .push((sleep.month, sleep.day, sleep.minute, wake.minute));
                // println!(
                //     "Guard {} slept another {} minutes",
                //     current_guard.id,
                //     wake.minute - sleep.minute
                // );
                current_guard.total_minutes += wake.minute - sleep.minute;
            }
        } else {
            done = true;
        }
    }

    let frequentest_sleep = guards
        .values()
        .into_iter()
        .max_by_key(|g| {
            let mut minutes_slept: HashMap<i32, i32> = HashMap::new();
            for sleep in g.sleeps.clone().into_iter() {
                for m in sleep.2..sleep.3 {
                    let minute = minutes_slept.entry(m).or_insert(0);
                    *minute += 1;
                }
            }
            println!("Minute totals {:?}", minutes_slept);
            let most_slept_minute = minutes_slept.values().max().unwrap_or(&0);
            most_slept_minute.clone()
        }).unwrap();
    println!("Most frequent sleep by {}", frequentest_sleep.id);
    let mut minutes_slept: HashMap<i32, i32> = HashMap::new();
    for sleep in frequentest_sleep.sleeps.clone().into_iter() {
        for m in sleep.2..sleep.3 {
            let minute = minutes_slept.entry(m).or_insert(0);
            *minute += 1;
        }
    }
    println!("Minute totals {:?}", minutes_slept);
    let most_slept_minute = minutes_slept
        .keys()
        .max_by_key(|m| minutes_slept[m])
        .unwrap();
    println!("Minutes slept {:?}", frequentest_sleep.sleeps);
    println!("Most slept minute {}", most_slept_minute);
}
