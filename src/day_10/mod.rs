const INPUT: &str = include_str!("input.txt");
// const INPUT: &str = include_str!("small_input.txt");

extern crate lazy_static;
use self::lazy_static::lazy_static;
extern crate regex;
use self::regex::Regex;
lazy_static! {
    static ref LINE_RE: Regex =
        Regex::new(r"position=<([- ]?\d+), ([- ]?\d+)> velocity=<([- ]?\d+), ([- ]?\d+)>").unwrap();
}

#[derive(Debug, Clone, Copy)]
struct Particle {
    x: i32,
    y: i32,
    dx: i32,
    dy: i32,
}

#[derive(Debug)]
struct BoundingBox {
    x_min: i32,
    x_max: i32,
    y_min: i32,
    y_max: i32,
}
impl BoundingBox {
    fn width(&self) -> usize {
        (self.x_max - self.x_min) as usize
    }

    fn height(&self) -> usize {
        (self.y_max - self.y_min) as usize
    }
}

fn parse_line(line: &str) -> Particle {
    let captures = LINE_RE.captures(line).unwrap();
    // println!("Line: {:?}", line);
    let particle = Particle {
        x: captures
            .get(1)
            .unwrap()
            .as_str()
            .trim()
            .parse::<i32>()
            .unwrap(),
        y: captures
            .get(2)
            .unwrap()
            .as_str()
            .trim()
            .parse::<i32>()
            .unwrap(),
        dx: captures
            .get(3)
            .unwrap()
            .as_str()
            .trim()
            .parse::<i32>()
            .unwrap(),
        dy: captures
            .get(4)
            .unwrap()
            .as_str()
            .trim()
            .parse::<i32>()
            .unwrap(),
    };
    // println!("Particle: {:?}", &particle);
    particle
}

use console::Term;
use std::thread;
use std::time::Duration;
pub fn part_1() {
    let mut particles = INPUT.lines().map(parse_line).collect::<Vec<Particle>>();
    // println!("{:?}", &particles);
    let mut tick = 0;
    let term = Term::buffered_stdout();
    let mut min_area = std::usize::MAX;

    while tick < 100000 {
        // println!("Bounds {:?}", &bounds);
        // println!("Width {:?}", &bounds.width());
        // println!("Height {:?}", &bounds.height());
        let bounds = BoundingBox {
            x_min: particles.clone().into_iter().min_by_key(|p| p.x).unwrap().x,
            y_min: particles.clone().into_iter().min_by_key(|p| p.y).unwrap().y,
            x_max: particles.clone().into_iter().max_by_key(|p| p.x).unwrap().x,
            y_max: particles.clone().into_iter().max_by_key(|p| p.y).unwrap().y,
        };

        let area = ((bounds.width()) * (bounds.height() + 1)) as usize;
        if area < min_area {
            min_area = area;
        }

        if area < 1600 {
            let mut map: Vec<char> = Vec::with_capacity(area);
            for _i in 0..area {
                map.push('.');
            }
            for particle in particles.clone().into_iter() {
                if particle.y >= bounds.y_min && particle.x >= bounds.x_min {
                    let y = (particle.y - bounds.y_min) as usize;
                    let x = (particle.x - bounds.x_min) as usize;
                    let offset = y * (bounds.width()) + x;
                    // println!("x {}, y {}, offset {}", x, y, offset);
                    if offset < map.len() {
                        map[offset as usize] = '#';
                    }
                }
            }
            {
                // let _ = term.clear_last_lines(bounds.height() + 1);
            }

            for line in map.chunks(bounds.width()) {
                // println!("{:?}", &line.into_iter().collect::<String>());
                let _ = term.write_line(&line.into_iter().collect::<String>());
            }
            {
                let _ = term.flush();
            }
            thread::sleep(Duration::from_millis(1000));
        }
        for mut particle in &mut particles {
            particle.x = particle.x + particle.dx;
            particle.y = particle.y + particle.dy;
        }

        tick += 1;
    }
    println!("Smallest area: {}", min_area);
}
pub fn part_2() {
    let mut particles = INPUT.lines().map(parse_line).collect::<Vec<Particle>>();
    // println!("{:?}", &particles);
    let mut tick = 0;
    let term = Term::buffered_stdout();
    let mut min_area = std::usize::MAX;
    let mut min_area_tick = 0;

    while tick < 100000 {
        // println!("Bounds {:?}", &bounds);
        // println!("Width {:?}", &bounds.width());
        // println!("Height {:?}", &bounds.height());
        let bounds = BoundingBox {
            x_min: particles.clone().into_iter().min_by_key(|p| p.x).unwrap().x,
            y_min: particles.clone().into_iter().min_by_key(|p| p.y).unwrap().y,
            x_max: particles.clone().into_iter().max_by_key(|p| p.x).unwrap().x,
            y_max: particles.clone().into_iter().max_by_key(|p| p.y).unwrap().y,
        };

        let area = ((bounds.width()) * (bounds.height() + 1)) as usize;
        if area < min_area {
            min_area = area;
            min_area_tick = tick;
        }

        //Run once for smallest area
        if area == 610 {
            let mut map: Vec<char> = Vec::with_capacity(area);
            for _i in 0..area {
                map.push('.');
            }
            for particle in particles.clone().into_iter() {
                if particle.y >= bounds.y_min && particle.x >= bounds.x_min {
                    let y = (particle.y - bounds.y_min) as usize;
                    let x = (particle.x - bounds.x_min) as usize;
                    let offset = y * (bounds.width()) + x;
                    // println!("x {}, y {}, offset {}", x, y, offset);
                    if offset < map.len() {
                        map[offset as usize] = '#';
                    }
                }
            }
            {
                // let _ = term.clear_last_lines(bounds.height() + 1);
            }

            for line in map.chunks(bounds.width()) {
                // println!("{:?}", &line.into_iter().collect::<String>());
                let _ = term.write_line(&line.into_iter().collect::<String>());
            }
            {
                let _ = term.flush();
            }
            thread::sleep(Duration::from_millis(1000));
        }
        for mut particle in &mut particles {
            particle.x = particle.x + particle.dx;
            particle.y = particle.y + particle.dy;
        }

        tick += 1;
    }
    println!(
        "Smallest area: {} found on tick {}",
        min_area, min_area_tick
    );
}
