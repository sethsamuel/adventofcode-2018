fn power(serial: i64, x: i64, y: i64) -> i64 {
    let rack_id = x + 10;
    let power = ((rack_id * y + serial) * rack_id / 100) % 10 - 5;
    power
}

fn power_square(serial: i64, x: i64, y: i64, size: i64) -> i64 {
    (0i64..size)
        .map(|dx| {
            (0i64..size)
                .map(|dy| power(serial, x + dx, y + dy))
                .fold(0, |acc, x| x + acc)
        })
        .fold(0, |acc, x| x + acc)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_power() {
        assert_eq!(super::power(8, 3, 5), 4);
        assert_eq!(super::power(57, 122, 79), -5);
        assert_eq!(super::power(39, 217, 196), 0);
        assert_eq!(super::power(71, 101, 153), 4);

        assert_eq!(super::power(18, 33, 45), 4);
        assert_eq!(super::power(18, 34, 45), 4);
        assert_eq!(super::power(18, 35, 45), 4);
        assert_eq!(super::power(18, 33, 45), 4);
        assert_eq!(super::power(18, 33, 46), 3);
        assert_eq!(super::power(18, 33, 47), 1);
    }

    #[test]
    fn test_power_square() {
        assert_eq!(super::power_square(18, 33, 45, 3), 29);
        assert_eq!(super::power_square(18, 90, 269, 16), 113);
    }
}

pub fn part_1() {
    let mut max_power = 0;
    let mut max_x = 0;
    let mut max_y = 0;
    let serial = 2187;
    for x in 1..=(300 - 2) {
        for y in 1..=(300 - 2) {
            let power = power_square(serial, x, y, 3);

            if power >= max_power {
                max_power = power;
                max_x = x;
                max_y = y;
            }
        }
    }
    println!("Max power {} found at {}, {}", max_power, max_x, max_y);
}

extern crate indicatif;
use self::indicatif::ProgressBar;
pub fn part_2() {
    let mut max_power = 0;
    let mut max_x = 0;
    let mut max_y = 0;
    let mut max_size = 0;
    let serial = 2187;
    let progress_bar = ProgressBar::new(300 * 300);

    for x in 1..=(300 - 2) {
        for y in 1..=(300 - 2) {
            progress_bar.inc(1);
            for size in 1..=300 {
                if x + size > 300 || y + size > 300 {
                    continue;
                }

                let power = power_square(serial, x, y, size);

                if power >= max_power {
                    max_power = power;
                    max_x = x;
                    max_y = y;
                    max_size = size;
                }
            }
        }
    }
    progress_bar.finish();
    println!(
        "Max power {} found at {}, {}, {}",
        max_power, max_x, max_y, max_size
    );
}
