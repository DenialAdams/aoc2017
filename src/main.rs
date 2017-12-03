#![feature(inclusive_range_syntax)]

extern crate itertools;

use itertools::Itertools;

fn main() {
    println!(
        "{:?}",
        inverse_captcha_part_one(include_str!("day1.in").trim_right().as_bytes())
    );
    println!(
        "{:?}",
        inverse_captcha_part_two(include_str!("day1.in").trim_right().as_bytes())
    );
    println!(
        "{:?}",
        corruption_checksum_part_one(include_str!("day2.in").trim_right())
    );
    println!(
        "{:?}",
        corruption_checksum_part_two(include_str!("day2.in").trim_right())
    );
    println!("{}", spiral_memory_part_one(265149));
    println!("{}", spiral_memory_part_two(265149));
}

pub fn inverse_captcha_part_one(captcha: &[u8]) -> Result<u64, &'static str> {
    let mut captcha_shift = Vec::with_capacity(captcha.len());
    unsafe { captcha_shift.set_len(captcha.len()) }
    captcha_shift[0] = captcha[captcha.len() - 1];
    captcha_shift[1..captcha.len()].copy_from_slice(&captcha[0..captcha.len() - 1]);
    let mut sum: u64 = 0;
    for (x, _) in captcha
        .iter()
        .zip(captcha_shift.iter())
        .filter(|&(x, y)| x == y)
    {
        sum += ascii_to_digit(*x)?;
    }
    Ok(sum)
}

pub fn inverse_captcha_part_two(captcha: &[u8]) -> Result<u64, &'static str> {
    let mut captcha_shift = Vec::with_capacity(captcha.len());
    unsafe { captcha_shift.set_len(captcha.len()) }
    captcha_shift[0..captcha.len() / 2].copy_from_slice(&captcha[captcha.len() / 2..captcha.len()]);
    captcha_shift[captcha.len() / 2..captcha.len()].copy_from_slice(&captcha[0..captcha.len() / 2]);
    let mut sum: u64 = 0;
    for (x, _) in captcha
        .iter()
        .zip(captcha_shift.iter())
        .filter(|&(x, y)| x == y)
    {
        sum += ascii_to_digit(*x)?;
    }
    Ok(sum)
}

pub fn corruption_checksum_part_one(spreadsheet: &str) -> Result<u64, ::std::num::ParseIntError> {
    let mut checksum = 0;
    for line in spreadsheet.lines() {
        let mut biggest_num: u64 = 0;
        let mut smallest_num: u64 = ::std::u64::MAX;
        for number in line.split_whitespace() {
            let i = number.parse::<u64>()?;
            if i > biggest_num {
                biggest_num = i;
            }
            if i < smallest_num {
                smallest_num = i;
            }
        }
        checksum += biggest_num - smallest_num;
    }
    Ok(checksum)
}

pub fn corruption_checksum_part_two(spreadsheet: &str) -> Result<u64, ::std::num::ParseIntError> {
    let mut sum = 0;
    for line in spreadsheet.lines() {
        let mut nums: Vec<u64> = Vec::new();
        for number in line.split_whitespace() {
            let i = number.parse::<u64>()?;
            nums.push(i);
        }
        'outer: for (x, y) in nums.iter().tuple_combinations() {
            if x % y == 0 {
                sum += x / y;
                break 'outer;
            } else if y % x == 0 {
                sum += y / x;
                break 'outer;
            }
        }
    }
    Ok(sum)
}

fn spiral_memory_part_one(start: i64) -> i64 {
    // TODO: we should reject negative numbers
    // Biggest number in ring
    let mut sum = 1;
    // Which ring we are in
    let mut n = 0;
    while sum < start {
        n += 1;
        sum += 8 * n;
    }
    // Distance is our ring number (n) + number of tiles we are away from the closest center
    let centers = vec![sum - (n * 1), sum - (n * 3), sum - (n * 5), sum - (n * 7)];
    let mut closest_center_dist = ::std::i64::MAX;
    for center in centers.into_iter() {
        let dist = (start - center).abs();
        if dist < closest_center_dist {
            closest_center_dist = dist;
        }
    }
    n + closest_center_dist
}

fn spiral_memory_part_two(num: u64) -> u64 {
    // trash tier solution
    let LENGTH: usize = 100;
    let CENTER = LENGTH / 2;
    let mut grid = [[0; 100]; 100];
    grid[CENTER][CENTER] = 1;
    let mut n = 1;
    let mut i = CENTER;
    let mut j = CENTER+1;
    let mut sum_surroundings = 0;
    while true {
        // UP N, UP N
        for _ in 0..(n*2)-1 {
            sum_surroundings = {
                grid[i-1][j] + grid[i-1][j-1] + grid[i-1][j+1] + grid[i][j+1] + grid[i][j-1] + grid[i+1][j+1] + grid[i+1][j] + grid[i+1][j-1]
            };
            if sum_surroundings > num {
                return sum_surroundings;
            }
            grid[i][j] = sum_surroundings;
            i -= 1;
        }
        // LEFT N, LEFT N
        for _ in 0..(n*2) {
            sum_surroundings = {
                grid[i-1][j] + grid[i-1][j-1] + grid[i-1][j+1] + grid[i][j+1] + grid[i][j-1] + grid[i+1][j+1] + grid[i+1][j] + grid[i+1][j-1]
            };
            if sum_surroundings > num {
                return sum_surroundings;
            }
            grid[i][j] = sum_surroundings;
            j -= 1;
        }
        // DOWN N, DOWN N
        for _ in 0..(n*2) {
            sum_surroundings = {
                grid[i-1][j] + grid[i-1][j-1] + grid[i-1][j+1] + grid[i][j+1] + grid[i][j-1] + grid[i+1][j+1] + grid[i+1][j] + grid[i+1][j-1]
            };
            if sum_surroundings > num {
                return sum_surroundings;
            }
            grid[i][j] = sum_surroundings;
            i += 1;
        }
        // RIGHT N, RIGHT N
        for _ in 0..(n*2)+1 {
            sum_surroundings = {
                grid[i-1][j] + grid[i-1][j-1] + grid[i-1][j+1] + grid[i][j+1] + grid[i][j-1] + grid[i+1][j+1] + grid[i+1][j] + grid[i+1][j-1]
            };
            if sum_surroundings > num {
                return sum_surroundings;
            }
            grid[i][j] = sum_surroundings;
            j += 1;
        }
        n += 1;
    }
    10
}

fn ascii_to_digit(ascii: u8) -> Result<u64, &'static str> {
    match ascii {
        b'0' => Ok(0),
        b'1' => Ok(1),
        b'2' => Ok(2),
        b'3' => Ok(3),
        b'4' => Ok(4),
        b'5' => Ok(5),
        b'6' => Ok(6),
        b'7' => Ok(7),
        b'8' => Ok(8),
        b'9' => Ok(9),
        _ => Err("Not a valid digit"),
    }
}

#[cfg(test)]
mod tests {
    use ::*;

    #[test]
    fn test_inverse_captcha_part_one() {
        assert_eq!(inverse_captcha_part_one("1122".as_bytes()), Ok(3));
        assert_eq!(inverse_captcha_part_one("1111".as_bytes()), Ok(4));
        assert_eq!(inverse_captcha_part_one("1234".as_bytes()), Ok(0));
        assert_eq!(inverse_captcha_part_one("91212129".as_bytes()), Ok(9));
    }

    #[test]
    fn test_inverse_captcha_part_two() {
        assert_eq!(inverse_captcha_part_two("1212".as_bytes()), Ok(6));
        assert_eq!(inverse_captcha_part_two("1221".as_bytes()), Ok(0));
        assert_eq!(inverse_captcha_part_two("123425".as_bytes()), Ok(4));
        assert_eq!(inverse_captcha_part_two("123123".as_bytes()), Ok(12));
        assert_eq!(inverse_captcha_part_two("12131415".as_bytes()), Ok(4));
    }

    #[test]
    fn test_corruption_checksum_part_one() {
        let spreadsheet = "5 1 9 5
7 5 3
2 4 6 8";
        assert_eq!(corruption_checksum_part_one(spreadsheet), Ok(18));
    }

    #[test]
    fn test_corruption_checksum_part_two() {
        let spreadsheet = "5 9 2 8
9 4 7 3
3 8 6 5";
        assert_eq!(corruption_checksum_part_two(spreadsheet), Ok(9));
    }

    #[test]
    fn test_spiral_memory_part_one() {
        assert_eq!(spiral_memory_part_one(1), 0);
        assert_eq!(spiral_memory_part_one(12), 3);
        assert_eq!(spiral_memory_part_one(22), 3);
        assert_eq!(spiral_memory_part_one(23), 2);
        assert_eq!(spiral_memory_part_one(1024), 31);
    }
}
