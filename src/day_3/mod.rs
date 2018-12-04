const INPUT: &str = include_str!("input.txt");
// const INPUT: &str = include_str!("small_input.txt");

#[derive(Eq, PartialEq, Debug)]
struct Claim {
    id: i32,
    top: i32,
    left: i32,
    width: i32,
    height: i32,
}

// impl PartialEq for Claim {
//     fn eq(&self, other: &Claim) {
//         self.id == other.id && self.top == other.top &&
//     }
// }

// impl Eq for Claim {}

extern crate lazy_static;
use self::lazy_static::lazy_static;
extern crate regex;
use self::regex::Regex;
lazy_static! {
    static ref LINE_RE: Regex = Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();
}

fn parse_line(line: &str) -> Claim {
    let captures = LINE_RE.captures(line).unwrap();
    Claim {
        id: captures.get(1).unwrap().as_str().parse::<i32>().unwrap(),
        left: captures.get(2).unwrap().as_str().parse::<i32>().unwrap(),
        top: captures.get(3).unwrap().as_str().parse::<i32>().unwrap(),
        width: captures.get(4).unwrap().as_str().parse::<i32>().unwrap(),
        height: captures.get(5).unwrap().as_str().parse::<i32>().unwrap(),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_parses() {
        assert_eq!(
            super::parse_line("#22 @ 527,886: 13x23"),
            super::Claim {
                id: 22,
                left: 527,
                top: 886,
                width: 13,
                height: 23,
            },
        )
    }
}

use std::collections::HashSet;
extern crate console;
use console::style;
pub fn part_1() {
    let claims = INPUT.lines().map(parse_line);
    let mut fabric: HashSet<(i32, i32)> = HashSet::new();
    let mut overlaps: HashSet<(i32, i32)> = HashSet::new();
    for claim in claims {
        if claim.width == 0 || claim.height == 0 {
            continue;
        }
        for y in claim.top..(claim.top + claim.height) {
            for x in claim.left..(claim.left + claim.width) {
                if fabric.contains(&(x, y)) {
                    overlaps.insert((x, y));
                } else {
                    fabric.insert((x, y));
                }
            }
        }
    }
    println!("Total overlaps: {}", style(overlaps.len()).green());
}

pub fn part_2() {
    let claims = INPUT.lines().map(parse_line);
    let mut fabric: HashSet<(i32, i32)> = HashSet::new();
    let mut overlaps: HashSet<(i32, i32)> = HashSet::new();
    for claim in claims.clone() {
        if claim.width == 0 || claim.height == 0 {
            continue;
        }
        for y in claim.top..(claim.top + claim.height) {
            for x in claim.left..(claim.left + claim.width) {
                if fabric.contains(&(x, y)) {
                    overlaps.insert((x, y));
                } else {
                    fabric.insert((x, y));
                }
            }
        }
    }
    for claim in claims {
        if claim.width == 0 || claim.height == 0 {
            continue;
        }
        let mut claim_overlaps = false;
        for y in claim.top..(claim.top + claim.height) {
            for x in claim.left..(claim.left + claim.width) {
                if overlaps.contains(&(x, y)) {
                    claim_overlaps = true;
                }
            }
        }
        if !claim_overlaps {
            println!("Fabric swatch found: {}", style(claim.id).green());
            return;
        }
    }
}
