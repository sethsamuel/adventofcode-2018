const INPUT: &str = include_str!("input.txt");
// const INPUT: &str = include_str!("small_input.txt");

#[derive(Debug, Eq, Hash, Copy)]
struct Coord {
    x: i32,
    y: i32,
}

impl Clone for Coord {
    fn clone(&self) -> Coord {
        *self
    }
}

impl Coord {
    fn distance(&self, other: &Coord) -> i32 {
        // println!("Distance from {:?} to {:?}", self, other);
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }

    fn closest_coord<T>(&self, coords: T) -> ClosestCoord
    where
        T: IntoIterator<Item = Coord>,
    {
        let point = self;
        let mut closest_coord = ClosestCoord::None;
        for coord in coords.into_iter() {
            match closest_coord {
                ClosestCoord::Multiple(closest) => {
                    let distance = point.distance(&coord);
                    let closest_distance = point.distance(&closest);
                    if distance < closest_distance {
                        closest_coord = ClosestCoord::Some(coord);
                    } else if distance == closest_distance {
                        closest_coord = ClosestCoord::Multiple(coord);
                    }
                }
                ClosestCoord::None => closest_coord = ClosestCoord::Some(coord),
                ClosestCoord::Some(closest) => {
                    let distance = point.distance(&coord);
                    let closest_distance = point.distance(&closest);
                    if distance < closest_distance {
                        closest_coord = ClosestCoord::Some(coord);
                    } else if distance == closest_distance {
                        closest_coord = ClosestCoord::Multiple(coord);
                    }
                }
            }
        }
        closest_coord
    }
}

impl PartialEq for Coord {
    fn eq(&self, other: &Coord) -> bool {
        self.x == other.x && self.y == other.y
    }
}

enum ClosestCoord {
    None,
    Some(Coord),
    Multiple(Coord),
}

#[derive(Debug)]
struct BoundingBox {
    x_min: i32,
    x_max: i32,
    y_min: i32,
    y_max: i32,
}

extern crate lazy_static;
use self::lazy_static::lazy_static;
extern crate regex;
use self::regex::Regex;
lazy_static! {
    static ref LINE_RE: Regex = Regex::new(r"(\d+), (\d+)").unwrap();
}

extern crate indicatif;
use self::indicatif::ProgressBar;
use std::collections::HashMap;
pub fn part_1() {
    let coords = INPUT.lines().map(|line| Coord {
        x: LINE_RE
            .captures(line)
            .unwrap()
            .get(1)
            .unwrap()
            .as_str()
            .parse::<i32>()
            .unwrap(),
        y: LINE_RE
            .captures(line)
            .unwrap()
            .get(2)
            .unwrap()
            .as_str()
            .parse::<i32>()
            .unwrap(),
    });

    let bounding_box = BoundingBox {
        x_min: coords.clone().min_by_key(|c| c.x).unwrap().x,
        x_max: coords.clone().max_by_key(|c| c.x).unwrap().x,
        y_min: coords.clone().min_by_key(|c| c.y).unwrap().y,
        y_max: coords.clone().max_by_key(|c| c.y).unwrap().y,
    };

    let mut closest_counts: HashMap<Coord, i32> = HashMap::new();
    let progress_bar = ProgressBar::new(
        ((bounding_box.x_max - bounding_box.x_min) * (bounding_box.y_max - bounding_box.y_min))
            .abs() as u64,
    );
    for x in bounding_box.x_min..=bounding_box.x_max {
        for y in bounding_box.y_min..=bounding_box.y_max {
            let point = Coord { x, y };
            let closest_coord = point.closest_coord(coords.clone());
            if let ClosestCoord::Some(closest) = closest_coord {
                *closest_counts.entry(closest).or_insert(0) += 1;
            }
            progress_bar.inc(1);
        }
    }
    progress_bar.finish();
    println!("Bounding box {:?}", bounding_box);
    println!("Closest coords {:?}", closest_counts);

    //Zero-out areas for coords that reach the edge and escape
    //Can a coord reach the edge then be contained?
    for x in bounding_box.x_min..=bounding_box.x_max {
        if let ClosestCoord::Some(closest) = (Coord {
            x,
            y: bounding_box.y_min,
        }.closest_coord(coords.clone()))
        {
            *closest_counts.entry(closest).or_insert(0) = 0;
        }
        if let ClosestCoord::Some(closest) = (Coord {
            x,
            y: bounding_box.y_max,
        }.closest_coord(coords.clone()))
        {
            *closest_counts.entry(closest).or_insert(0) = 0;
        }
    }
    for y in bounding_box.y_min..=bounding_box.y_max {
        if let ClosestCoord::Some(closest) = (Coord {
            x: bounding_box.x_min,
            y,
        }.closest_coord(coords.clone()))
        {
            *closest_counts.entry(closest).or_insert(0) = 0;
        }
        if let ClosestCoord::Some(closest) = (Coord {
            x: bounding_box.x_max,
            y,
        }.closest_coord(coords.clone()))
        {
            *closest_counts.entry(closest).or_insert(0) = 0;
        }
    }

    println!("Largest finite area: {:?}", closest_counts.values().max());
}
pub fn part_2() {
    let coords = INPUT.lines().map(|line| Coord {
        x: LINE_RE
            .captures(line)
            .unwrap()
            .get(1)
            .unwrap()
            .as_str()
            .parse::<i32>()
            .unwrap(),
        y: LINE_RE
            .captures(line)
            .unwrap()
            .get(2)
            .unwrap()
            .as_str()
            .parse::<i32>()
            .unwrap(),
    });

    let bounding_box = BoundingBox {
        x_min: coords.clone().min_by_key(|c| c.x).unwrap().x,
        x_max: coords.clone().max_by_key(|c| c.x).unwrap().x,
        y_min: coords.clone().min_by_key(|c| c.y).unwrap().y,
        y_max: coords.clone().max_by_key(|c| c.y).unwrap().y,
    };

    let mut distances: HashMap<Coord, i32> = HashMap::new();
    let progress_bar = ProgressBar::new(
        ((bounding_box.x_max - bounding_box.x_min) * (bounding_box.y_max - bounding_box.y_min))
            .abs() as u64,
    );
    for x in bounding_box.x_min..=bounding_box.x_max {
        for y in bounding_box.y_min..=bounding_box.y_max {
            let point = Coord { x, y };
            let mut distance = 0;
            for coord in coords.clone() {
                distance += point.distance(&coord);
            }
            distances.insert(point, distance);
            progress_bar.inc(1);
        }
    }
    progress_bar.finish();
    println!("Bounding box {:?}", bounding_box);
    let threshold = 10_000;
    println!(
        "Area < 10000: {}",
        distances.values().filter(|d| **d < threshold).count()
    );
}
