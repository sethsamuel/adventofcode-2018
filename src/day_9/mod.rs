use std::collections::HashMap;
pub fn part_1() {
    let player_count = 428;
    let max_marble = 70825;
    let mut scores: HashMap<i32, i32> = HashMap::new();
    //Avoid ugly conditionals in early insertion steps
    let mut marbles = vec![0, 1];
    let mut current_marble = 1;
    let mut current_player = 1;
    for marble in 2..=max_marble {
        if marble % 23 == 0 {
            let score = scores.entry(current_player).or_insert(0);
            *score += marble;
            //For type ease current_marble is usize, so manually handle the wrap around
            let score_marble = ((marbles.len() + current_marble) - 7) % marbles.len();
            // println!(
            //     "SCORE! {} {} {} {}",
            //     current_marble,
            //     ((marbles.len() + current_marble) - 7) % marbles.len(),
            //     marbles.len(),
            //     score_marble
            // );
            current_marble = score_marble;
            *score += marbles.remove(score_marble);
        } else {
            current_marble = current_marble + 2;
            if current_marble > marbles.len() {
                current_marble = current_marble % marbles.len();
            }
            marbles.insert(current_marble, marble);
        }
        // println!(
        //     "[{}] {:?} ({})",
        //     current_player + 1,
        //     marbles,
        //     current_marble
        // );

        current_player = (current_player + 1) % player_count;
    }
    // println!("Scores: {:?}", scores);
    println!("Winner!: {}", scores.values().max().unwrap());
}

extern crate indicatif;
use self::indicatif::ProgressBar;
extern crate skiplist;
use self::skiplist::skiplist::SkipList;
pub fn part_2() {
    // let i = 5;
    // println!("{}", (3 - 7) % 7);
    // return;

    let player_count = 428;
    let max_marble = 70825 * 100;
    let mut scores: HashMap<u32, u64> = HashMap::new();
    //Avoid ugly conditionals in early insertion steps
    // let mut marbles = vec![0, 1];
    let mut marbles = SkipList::new();
    marbles.insert(0, 0);
    marbles.insert(1, 1);
    let mut current_marble = 1;
    let mut current_player = 1;

    let progress_bar = ProgressBar::new(max_marble as u64);
    for marble in 2..=max_marble {
        progress_bar.inc(1);
        if marble % 23 == 0 {
            let score = scores.entry(current_player).or_insert(0);
            *score += marble;
            //For type ease current_marble is usize, so manually handle the wrap around
            let score_marble = ((marbles.len() + current_marble) - 7) % marbles.len();
            // println!(
            //     "SCORE! {} {} {} {}",
            //     current_marble,
            //     ((marbles.len() + current_marble) - 7) % marbles.len(),
            //     marbles.len(),
            //     score_marble
            // );
            current_marble = score_marble;
            *score += marbles.remove(score_marble);
        } else {
            current_marble = current_marble + 2;
            if current_marble > marbles.len() {
                current_marble = current_marble % marbles.len();
            }
            //Skip list reverses arguments
            marbles.insert(marble, current_marble);
            // marbles.insert(current_marble, marble);
        }
        // println!(
        //     "[{}] {:?} ({})",
        //     current_player + 1,
        //     marbles,
        //     current_marble
        // );

        current_player = (current_player + 1) % player_count;
    }
    progress_bar.finish();
    // println!("Scores: {:?}", scores);
    println!("Winner!: {}", scores.values().max().unwrap());
}
