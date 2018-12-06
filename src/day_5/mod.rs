const INPUT: &str = include_str!("input.txt");
// const INPUT: &str = include_str!("small_input.txt");

extern crate indicatif;
use self::indicatif::ProgressBar;

fn reduce(string: &str) -> String {
    let mut input = string.clone().to_string();
    let mut was_reaction = true;
    while was_reaction {
        //Find reaction and update string
        was_reaction = false;
        // println!("Current chars {:?}", input);
        // let progress_bar = ProgressBar::new(input.len() as u64);

        for i in 0..input.len() - 1 {
            // progress_bar.inc(1);
            if i > input.len() - 2 {
                //Mutated outside length
                continue;
            }
            // println!("Current chars {:?}", input.chars());
            // println!("i {}", i);

            let char0 = input.chars().nth(i).unwrap();
            let char1 = input.chars().nth(i + 1).unwrap();
            // println!("chars {} {}", char0, char1);
            if char0.to_string().to_lowercase() == char1.to_string().to_lowercase()
                && char0 != char1
            {
                was_reaction = true;
                let mut left = i;
                let mut right = i + 1;
                let mut is_matching = true;
                while is_matching && left > 0 && right < input.len() - 2 {
                    left -= 1;
                    right += 1;
                    let char0 = input.chars().nth(left).unwrap();
                    let char1 = input.chars().nth(right).unwrap();
                    if char0.to_string().to_lowercase() == char1.to_string().to_lowercase()
                        && char0 != char1
                    {
                        is_matching = true;
                    } else {
                        is_matching = false;
                        left += 1;
                        right -= 1;
                    }
                }
                let input_clone = input.clone();
                let chars = input_clone.chars();
                input = chars
                    .clone()
                    .take(left)
                    .chain(chars.skip(right + 1))
                    .collect::<String>();
                // input = i == input.len() - 2 ? input.get(0..i) : input.get(i..);
                continue;
            }
        }
        // progress_bar.finish();
    }
    input
}

pub fn part_1() {
    let input = reduce(INPUT);
    println!("Final product {} of length {}", input, input.len());
}
use std::collections::HashMap;
use std::thread;
use std::time::Duration;
pub fn part_2() {
    let mut lengths: HashMap<char, usize> = HashMap::new();
    (b'a'..=b'z')
        .map(|alpha_byte| {
            let alpha = alpha_byte as char;
            println!("Removing all {}", alpha);
            let mut input = INPUT.clone().to_string();
            input = input.replace(&alpha.to_string(), "");
            input = input.replace(&alpha.to_string().to_uppercase(), "");
            let child = thread::spawn(move || {
                let reduced = reduce(&input);
                // println!("Final product {} of length {}", reduced, reduced.len());
                reduced
            });
            thread::sleep(Duration::from_millis(1));

            (alpha, child)
        }).collect::<Vec<_>>()
        .into_iter()
        .for_each(|(alpha, child)| {
            let reduced = child.join().unwrap();
            lengths.insert(alpha, reduced.len());
        });

    let shortest = lengths.keys().min_by_key(|k| lengths[k]).unwrap();
    println!(
        "Shortest length with char {}: {}",
        shortest, lengths[shortest]
    );
}
