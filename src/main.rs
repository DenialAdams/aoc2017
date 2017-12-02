mod lib;

use lib::*;

fn main() {
    println!("{:?}", inverse_captcha_part_one(include_str!("day1.in").trim_right().as_bytes()));
    println!("{:?}", inverse_captcha_part_two(include_str!("day1.in").trim_right().as_bytes()));
}
