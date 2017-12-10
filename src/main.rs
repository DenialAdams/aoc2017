#![feature(inclusive_range_syntax)]

extern crate itertools;

use itertools::Itertools;
use std::collections::HashSet;
use std::collections::HashMap;

fn main() {
    /*
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
    println!(
        "{:?}",
        high_entropy_passphrases_part_one(include_str!("day4.in").trim_right())
    );
    println!(
        "{:?}",
        high_entropy_passphrases_part_two(include_str!("day4.in").trim_right())
    );
    println!(
        "{:?}",
        maze_twisty_trampolines_part_one(include_str!("day5.in").trim_right())
    );
    println!(
        "{:?}",
        maze_twisty_trampolines_part_two(include_str!("day5.in").trim_right())
    );
    println!(   
        "{:?}",
        memory_reallocation_part_one(include_str!("day6.in").trim_right())
    );
    println!(   
        "{:?}",
        memory_reallocation_part_two(include_str!("day6.in").trim_right())
    ); */
    println!(
        "{}",
        recursive_circus_part_one(include_str!("day7.in").trim_right())
    );
    println!(
        "{}",
        recursive_circus_part_two(include_str!("day7.in").trim_right())
    );
    println!{
        "{}",
        registers_part_one(include_str!("day8.in").trim_right())
    };
    println!{
        "{}",
        registers_part_two(include_str!("day8.in").trim_right())
    };
    println!{
        "{}",
        stream_processing_part_one(include_str!("day9.in").trim_right())
    }
    println!{
        "{}",
        stream_processing_part_two(include_str!("day9.in").trim_right())
    }
}

pub fn stream_processing_part_two(input: &str) -> u64 {
    let mut score = 0;
    let mut in_garbage = false;
    let mut skip = false;
    for a_char in input.chars() {
        if skip {
            skip = false;
        } else if in_garbage {
            match a_char {
                '!' => skip = true,
                '>' => in_garbage = false,
                _ => {
                    score += 1
                    // continue
                }
            }
        } else {
            match a_char {
                '<' => in_garbage = true,
                '!' => skip = true,
                _ => {
                    // continue
                }
            }
        }
    }
    score
}

pub fn stream_processing_part_one(input: &str) -> u64 {
    let mut depth = 0;
    let mut score = 0;
    let mut in_garbage = false;
    let mut skip = false;
    for a_char in input.chars() {
        if skip {
            skip = false;
        } else if in_garbage {
            match a_char {
                '!' => skip = true,
                '>' => in_garbage = false,
                _ => {
                    // continue
                }
            }
        } else {
            match a_char {
                '<' => in_garbage = true,
                '!' => skip = true,
                '{' => depth += 1,
                '}' => {
                    score += depth;
                    depth -= 1;
                },
                _ => {
                    // continue
                }
            }
        }
    }
    score
}

pub fn registers_part_one(input: &str) -> i64 {
    let mut registers: HashMap<&str, i64> = HashMap::new();
    for line in input.lines() {
        let line: Vec<&str> = line.split_whitespace().collect();
        assert_eq!(line[3], "if");
        assert_eq!(line.len(), 7);
        match line[5] {
            "<" => {
                if *registers.get(line[4]).unwrap_or(&0) < line[6].parse().unwrap() {
                    let val = { *registers.get_mut(line[0]).unwrap_or(&mut 0) };
                    match line[1] {
                        "inc" => {
                            registers.insert(line[0], val + line[2].parse::<i64>().unwrap());
                        },
                        "dec" => {
                            registers.insert(line[0], val - line[2].parse::<i64>().unwrap());
                        },
                        _ => {
                            assert!(false);
                        }
                    }
                }
            },
            ">" => {
                if *registers.get(line[4]).unwrap_or(&0) > line[6].parse().unwrap() {
                    let val = { *registers.get_mut(line[0]).unwrap_or(&mut 0) };
                    match line[1] {
                        "inc" => {
                            registers.insert(line[0], val + line[2].parse::<i64>().unwrap());
                        },
                        "dec" => {
                            registers.insert(line[0], val - line[2].parse::<i64>().unwrap());
                        },
                        _ => {
                            assert!(false);
                        }
                    }
                }
            },
            "==" => {
                if *registers.get(line[4]).unwrap_or(&0) == line[6].parse().unwrap() {
                    let val = { *registers.get_mut(line[0]).unwrap_or(&mut 0) };
                    match line[1] {
                        "inc" => {
                            registers.insert(line[0], val + line[2].parse::<i64>().unwrap());
                        },
                        "dec" => {
                            registers.insert(line[0], val - line[2].parse::<i64>().unwrap());
                        },
                        _ => {
                            assert!(false);
                        }
                    }
                }
            },
            ">=" => {
                if *registers.get(line[4]).unwrap_or(&0) >= line[6].parse().unwrap() {
                    let val = { *registers.get_mut(line[0]).unwrap_or(&mut 0) };
                    match line[1] {
                        "inc" => {
                            registers.insert(line[0], val + line[2].parse::<i64>().unwrap());
                        },
                        "dec" => {
                            registers.insert(line[0], val - line[2].parse::<i64>().unwrap());
                        },
                        _ => {
                            assert!(false);
                        }
                    }
                }
            },
            "<=" => {
                if *registers.get(line[4]).unwrap_or(&0) <= line[6].parse().unwrap() {
                    let val = { *registers.get_mut(line[0]).unwrap_or(&mut 0) };
                    match line[1] {
                        "inc" => {
                            registers.insert(line[0], val + line[2].parse::<i64>().unwrap());
                        },
                        "dec" => {
                            registers.insert(line[0], val - line[2].parse::<i64>().unwrap());
                        },
                        _ => {
                            assert!(false);
                        }
                    }
                }
            },
            "!=" => {
                if *registers.get(line[4]).unwrap_or(&0) != line[6].parse().unwrap() {
                    let val = { *registers.get_mut(line[0]).unwrap_or(&mut 0) };
                    match line[1] {
                        "inc" => {
                            registers.insert(line[0], val + line[2].parse::<i64>().unwrap());
                        },
                        "dec" => {
                            registers.insert(line[0], val - line[2].parse::<i64>().unwrap());
                        },
                        _ => {
                            assert!(false);
                        }
                    }
                }
            }
            _ => {
                assert!(false);
            }
        }
    }
    let max = registers.values().fold(0, |best, &value| if value > best { value } else { best });
    max
}

pub fn registers_part_two(input: &str) -> i64 {
    let mut registers: HashMap<&str, i64> = HashMap::new();
    let mut all_time_max = 0;
    for line in input.lines() {
        let line: Vec<&str> = line.split_whitespace().collect();
        assert_eq!(line[3], "if");
        assert_eq!(line.len(), 7);
        match line[5] {
            "<" => {
                if *registers.get(line[4]).unwrap_or(&0) < line[6].parse().unwrap() {
                    let val = { *registers.get_mut(line[0]).unwrap_or(&mut 0) };
                    match line[1] {
                        "inc" => {
                            registers.insert(line[0], val + line[2].parse::<i64>().unwrap());
                        },
                        "dec" => {
                            registers.insert(line[0], val - line[2].parse::<i64>().unwrap());
                        },
                        _ => {
                            assert!(false);
                        }
                    }
                    if registers[line[0]] > all_time_max {
                        all_time_max = registers[line[0]];
                    }
                }
            },
            ">" => {
                if *registers.get(line[4]).unwrap_or(&0) > line[6].parse().unwrap() {
                    let val = { *registers.get_mut(line[0]).unwrap_or(&mut 0) };
                    match line[1] {
                        "inc" => {
                            registers.insert(line[0], val + line[2].parse::<i64>().unwrap());
                        },
                        "dec" => {
                            registers.insert(line[0], val - line[2].parse::<i64>().unwrap());
                        },
                        _ => {
                            assert!(false);
                        }
                    }
                    if registers[line[0]] > all_time_max {
                        all_time_max = registers[line[0]];
                    }
                }
            },
            "==" => {
                if *registers.get(line[4]).unwrap_or(&0) == line[6].parse().unwrap() {
                    let val = { *registers.get_mut(line[0]).unwrap_or(&mut 0) };
                    match line[1] {
                        "inc" => {
                            registers.insert(line[0], val + line[2].parse::<i64>().unwrap());
                        },
                        "dec" => {
                            registers.insert(line[0], val - line[2].parse::<i64>().unwrap());
                        },
                        _ => {
                            assert!(false);
                        }
                    }
                    if registers[line[0]] > all_time_max {
                        all_time_max = registers[line[0]];
                    }
                }
            },
            ">=" => {
                if *registers.get(line[4]).unwrap_or(&0) >= line[6].parse().unwrap() {
                    let val = { *registers.get_mut(line[0]).unwrap_or(&mut 0) };
                    match line[1] {
                        "inc" => {
                            registers.insert(line[0], val + line[2].parse::<i64>().unwrap());
                        },
                        "dec" => {
                            registers.insert(line[0], val - line[2].parse::<i64>().unwrap());
                        },
                        _ => {
                            assert!(false);
                        }
                    }
                    if registers[line[0]] > all_time_max {
                        all_time_max = registers[line[0]];
                    }
                }
            },
            "<=" => {
                if *registers.get(line[4]).unwrap_or(&0) <= line[6].parse().unwrap() {
                    let val = { *registers.get_mut(line[0]).unwrap_or(&mut 0) };
                    match line[1] {
                        "inc" => {
                            registers.insert(line[0], val + line[2].parse::<i64>().unwrap());
                        },
                        "dec" => {
                            registers.insert(line[0], val - line[2].parse::<i64>().unwrap());
                        },
                        _ => {
                            assert!(false);
                        }
                    }
                    if registers[line[0]] > all_time_max {
                        all_time_max = registers[line[0]];
                    }
                }
            },
            "!=" => {
                if *registers.get(line[4]).unwrap_or(&0) != line[6].parse().unwrap() {
                    let val = { *registers.get_mut(line[0]).unwrap_or(&mut 0) };
                    match line[1] {
                        "inc" => {
                            registers.insert(line[0], val + line[2].parse::<i64>().unwrap());
                        },
                        "dec" => {
                            registers.insert(line[0], val - line[2].parse::<i64>().unwrap());
                        },
                        _ => {
                            assert!(false);
                        }
                    }
                    if registers[line[0]] > all_time_max {
                        all_time_max = registers[line[0]];
                    }
                }
            }
            _ => {
                assert!(false);
            }
        }
    }
    all_time_max
}

struct Program {
    weight: u64,
    children: Vec<String>
}

pub fn recursive_circus_part_one(input: &str) -> String {
    let mut programs: HashMap<&str, Program> = HashMap::new();
    for line in input.lines() {
        let line_elems: Vec<_> = line.split_whitespace().collect();
        let name = line_elems[0];
        let weight = {
            let foo = line_elems[1];
            let sub_foo = &foo[1..foo.len()-1];
            sub_foo.parse().expect("Failed to parse input")
        };
        let mut children = Vec::new();
        if let Some(&val) = line_elems.get(2) {
            assert_eq!(val, "->");
            let mut n = 3;
            while let Some(&child) = line_elems.get(n) {
                let sub_child = if child.bytes().rev().nth(0).unwrap() == b',' {
                    &child[..child.len()-1]
                } else {
                    child
                };
                children.push(String::from(sub_child));
                n += 1;
            }
        }
        programs.insert(name, Program {
            weight: weight,
            children: children
        });
    }
    let mut potential_roots: HashSet<&str> = programs.keys().map(|x| *x).collect();
    for elem in programs.values() {
        for child in elem.children.iter() {
            let val = potential_roots.remove(child.as_str());
        }
    }
    assert_eq!(potential_roots.len(), 1);
    String::from(potential_roots.into_iter().nth(0).unwrap())
}

pub fn recursive_circus_part_two(input: &str) -> u64 {
    let mut programs: HashMap<&str, Program> = HashMap::new();
    for line in input.lines() {
        let line_elems: Vec<_> = line.split_whitespace().collect();
        let name = line_elems[0];
        let weight = {
            let foo = line_elems[1];
            let sub_foo = &foo[1..foo.len()-1];
            sub_foo.parse().expect("Failed to parse input")
        };
        let mut children = Vec::new();
        if let Some(&val) = line_elems.get(2) {
            assert_eq!(val, "->");
            let mut n = 3;
            while let Some(&child) = line_elems.get(n) {
                let sub_child = if child.bytes().rev().nth(0).unwrap() == b',' {
                    &child[..child.len()-1]
                } else {
                    child
                };
                children.push(String::from(sub_child));
                n += 1;
            }
        }
        programs.insert(name, Program {
            weight: weight,
            children: children
        });
    }
    let mut potential_roots: HashSet<&str> = programs.keys().map(|x| *x).collect();
    for elem in programs.values() {
        for child in elem.children.iter() {
            let val = potential_roots.remove(child.as_str());
        }
    }
    assert_eq!(potential_roots.len(), 1);
    let root = potential_roots.into_iter().nth(0).unwrap();
    let mut to_check: HashSet<String> = programs[root].children.clone().into_iter().collect();
    let mut last_to_check = to_check.clone();
    let mut last_good_weight = 0;
    loop {
        let weights: Vec<_> = to_check.clone().into_iter().map(|child| (child.clone(), sum_weight(&child, &programs))).collect();
        let mut good_weight = 0;
        for ((child, weight), (other_child, other_weight)) in weights.into_iter().tuple_combinations() {
            if weight == other_weight {
                to_check.remove(&child);
                to_check.remove(&other_child);
                good_weight = weight;
            }
        }
        if to_check.is_empty() {
            break;
        }
        last_good_weight = good_weight;
        last_to_check = to_check.clone();
        to_check.clear();
        for name in last_to_check.iter() {
            for child in programs[name.as_str()].children.iter() {
                to_check.insert(child.clone());
            }
        }
    }
    assert_eq!(last_to_check.len(), 1);
    let bad_node = last_to_check.iter().nth(0).unwrap();
    let bad_weight = programs[bad_node.as_str()].weight;
    let children_weight = sum_weight(bad_node, &programs) - bad_weight;
    last_good_weight - children_weight
}

fn sum_weight(node_name: &str, programs: &HashMap<&str, Program>) -> u64 {
    let mut sum = 0;
    let node = &programs[node_name];
    sum += node.weight;
    for child in node.children.iter() {
        sum += sum_weight(child, programs);
    }
    sum
}

pub fn memory_reallocation_part_one(input: &str) -> i64 {
    let mut items: Vec<u64> = Vec::new();
    let mut biggest_val = 0;
    let mut index_big_val = 0;
    for (index, item) in input.split_whitespace().enumerate() {
        let val = item.parse().expect("Failed to parse input");
        if val > biggest_val {
            biggest_val = val;
            index_big_val = index;
        }
        items.push(val);
    }
    let mut position_table = HashSet::new();
    position_table.insert(items.clone());
    let mut cycles = 0;
    loop {
        let mut spread = items[index_big_val];
        items[index_big_val] = 0;
        let mut index = index_big_val + 1;
        while spread > 0 {
            if index >= items.len() {
                index = 0;
            }
            items[index] += 1;
            spread -= 1;
            index += 1;
        }
        cycles += 1;
        if position_table.contains(&items) {
            break;
        }
        position_table.insert(items.clone());
        biggest_val = 0;
        index_big_val = 0;
        for (i, val) in items.iter().enumerate() {
            if *val > biggest_val {
                biggest_val = *val;
                index_big_val = i;
            }
        }
    }
    cycles
}

pub fn memory_reallocation_part_two(input: &str) -> i64 {
    let mut items: Vec<u64> = Vec::new();
    let mut biggest_val = 0;
    let mut index_big_val = 0;
    for (index, item) in input.split_whitespace().enumerate() {
        let val = item.parse().expect("Failed to parse input");
        if val > biggest_val {
            biggest_val = val;
            index_big_val = index;
        }
        items.push(val);
    }
    let mut position_table = HashMap::new();
    position_table.insert(items.clone(), 0);
    let mut cycles = 0;
    loop {
        let mut spread = items[index_big_val];
        items[index_big_val] = 0;
        let mut index = index_big_val + 1;
        while spread > 0 {
            if index >= items.len() {
                index = 0;
            }
            items[index] += 1;
            spread -= 1;
            index += 1;
        }
        cycles += 1;
        if position_table.contains_key(&items) {
            break cycles - position_table[&items];
        }
        position_table.insert(items.clone(), cycles);
        biggest_val = 0;
        index_big_val = 0;
        for (i, val) in items.iter().enumerate() {
            if *val > biggest_val {
                biggest_val = *val;
                index_big_val = i;
            }
        }
    }
}

pub fn maze_twisty_trampolines_part_one(input: &str) -> i64 {
    let mut list: Vec<i64> = Vec::new();
    for line in input.lines() {
        list.push(line.parse().expect("Failed to parse input"))
    }
    let mut index: i64 = 0;
    let mut steps = 0;
    loop {
        if index >= list.len() as i64 || index < 0 {
            break;
        }
        let val = list[index as usize];
        list[index as usize] += 1;
        index += val;
        steps += 1;
    }
    steps
}

pub fn maze_twisty_trampolines_part_two(input: &str) -> i64 {
    let mut list: Vec<i64> = Vec::new();
    for line in input.lines() {
        list.push(line.parse().expect("Failed to parse input"))
    }
    let mut index: i64 = 0;
    let mut steps = 0;
    loop {
        if index >= list.len() as i64 || index < 0 {
            break;
        }
        let val = list[index as usize];
        if val >= 3 {
            list[index as usize] -= 1;
        } else {
            list[index as usize] += 1;
        }
        index += val;
        steps += 1;
    }
    steps
}

pub fn high_entropy_passphrases_part_one(input: &str) -> u64 {
    let mut sum = 0;
    for line in input.lines() {
        let mut add = true;
        for (x, y) in line.split_whitespace().tuple_combinations() {
            if x == y {
                add = false;
                break;
            }
        }
        if add {
            sum += 1;
        }
    }
    sum
}

pub fn high_entropy_passphrases_part_two(input: &str) -> u64 {
    let mut sum = 0;
    for line in input.lines() {
        let mut add = true;
        for (x, y) in line.split_whitespace().tuple_combinations() {
            let mut x_chars: Vec<u8> = x.bytes().collect();
            x_chars.sort_unstable();
            let mut y_chars: Vec<u8> = y.bytes().collect();
            y_chars.sort_unstable();
            if x_chars == y_chars {
                add = false;
                break;
            }
        }
        if add {
            sum += 1;
        }
    }
    sum
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
    const LENGTH: usize = 100;
    const CENTER: usize = LENGTH / 2;
    let mut grid = [[0; 100]; 100];
    grid[CENTER][CENTER] = 1;
    let mut n = 1;
    let mut i = CENTER;
    let mut j = CENTER + 1;
    loop {
        // UP N, UP N
        for _ in 0..(n * 2) - 1 {
            let sum_surroundings = {
                grid[i - 1][j] + grid[i - 1][j - 1] + grid[i - 1][j + 1] + grid[i][j + 1]
                    + grid[i][j - 1] + grid[i + 1][j + 1] + grid[i + 1][j]
                    + grid[i + 1][j - 1]
            };
            if sum_surroundings > num {
                return sum_surroundings;
            }
            grid[i][j] = sum_surroundings;
            i -= 1;
        }
        // LEFT N, LEFT N
        for _ in 0..(n * 2) {
            let sum_surroundings = {
                grid[i - 1][j] + grid[i - 1][j - 1] + grid[i - 1][j + 1] + grid[i][j + 1]
                    + grid[i][j - 1] + grid[i + 1][j + 1] + grid[i + 1][j]
                    + grid[i + 1][j - 1]
            };
            if sum_surroundings > num {
                return sum_surroundings;
            }
            grid[i][j] = sum_surroundings;
            j -= 1;
        }
        // DOWN N, DOWN N
        for _ in 0..(n * 2) {
            let sum_surroundings = {
                grid[i - 1][j] + grid[i - 1][j - 1] + grid[i - 1][j + 1] + grid[i][j + 1]
                    + grid[i][j - 1] + grid[i + 1][j + 1] + grid[i + 1][j]
                    + grid[i + 1][j - 1]
            };
            if sum_surroundings > num {
                return sum_surroundings;
            }
            grid[i][j] = sum_surroundings;
            i += 1;
        }
        // RIGHT N, RIGHT N
        for _ in 0..(n * 2) + 1 {
            let sum_surroundings = {
                grid[i - 1][j] + grid[i - 1][j - 1] + grid[i - 1][j + 1] + grid[i][j + 1]
                    + grid[i][j - 1] + grid[i + 1][j + 1] + grid[i + 1][j]
                    + grid[i + 1][j - 1]
            };
            if sum_surroundings > num {
                return sum_surroundings;
            }
            grid[i][j] = sum_surroundings;
            j += 1;
        }
        n += 1;
    }
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

    #[test]
    fn test_maze_twisty_trampolines_part_one() {
        let inp = "0
3
0
1
-3";
        assert_eq!(maze_twisty_trampolines_part_one(inp), 5);
    }

    #[test]
    fn test_maze_twisty_trampolines_part_two() {
        let inp = "0
3
0
1
-3";
        assert_eq!(maze_twisty_trampolines_part_two(inp), 10);
    }

    #[test]
    fn test_memory_reallocation_part_one() {
        assert_eq!(memory_reallocation_part_one("0 2 7 0"), 5);
    }

    #[test]
    fn test_memory_reallocation_part_two() {
        assert_eq!(memory_reallocation_part_two("0 2 7 0"), 4);
    }

    #[test]
    fn test_recursive_circus_part_one() {
        let input = "pbga (66)
xhth (57)
ebii (61)
havc (66)
ktlj (57)
fwft (72) -> ktlj, cntj, xhth
qoyq (66)
padx (45) -> pbga, havc, qoyq
tknk (41) -> ugml, padx, fwft
jptl (61)
ugml (68) -> gyxo, ebii, jptl
gyxo (61)
cntj (57)";
        assert_eq!(recursive_circus_part_one(input), String::from("tknk"));
    }

    #[test]
    fn test_recursive_circus_part_two() {
        let input = "pbga (66)
xhth (57)
ebii (61)
havc (66)
ktlj (57)
fwft (72) -> ktlj, cntj, xhth
qoyq (66)
padx (45) -> pbga, havc, qoyq
tknk (41) -> ugml, padx, fwft
jptl (61)
ugml (68) -> gyxo, ebii, jptl
gyxo (61)
cntj (57)";
        assert_eq!(recursive_circus_part_two(input), 60);
    }

    #[test]
    fn test_registers_part_one() {
        let input = "b inc 5 if a > 1
a inc 1 if b < 5
c dec -10 if a >= 1
c inc -20 if c == 10";
        assert_eq!(registers_part_one(input), 1);
    }

    #[test]
    fn test_registers_part_two() {
        let input = "b inc 5 if a > 1
a inc 1 if b < 5
c dec -10 if a >= 1
c inc -20 if c == 10";
        assert_eq!(registers_part_two(input), 10);
    }

    #[test]
    fn test_stream_processing_part_one() {
        assert_eq!(stream_processing_part_one("{}"), 1);
        assert_eq!(stream_processing_part_one("{{{}}}"), 6);
        assert_eq!(stream_processing_part_one("{{},{}}"), 5);
        assert_eq!(stream_processing_part_one("{{{},{},{{}}}}"), 16);
        assert_eq!(stream_processing_part_one("{<a>,<a>,<a>,<a>}"), 1);
        assert_eq!(stream_processing_part_one("{{<ab>},{<ab>},{<ab>},{<ab>}}"), 9);
        assert_eq!(stream_processing_part_one("{{<!!>},{<!!>},{<!!>},{<!!>}}"), 9);
        assert_eq!(stream_processing_part_one("{{<a!>},{<a!>},{<a!>},{<ab>}}"), 3);
    }

    #[test]
    fn test_stream_processing_part_two() {
        assert_eq!(stream_processing_part_two("<>"), 0);
        assert_eq!(stream_processing_part_two("<random characters>"), 17);
        assert_eq!(stream_processing_part_two("<<<<>"), 3);
        assert_eq!(stream_processing_part_two("<{!>}>"), 2);
        assert_eq!(stream_processing_part_two("<!!>"), 0);
        assert_eq!(stream_processing_part_two("<!!!>>"), 0);
        assert_eq!(stream_processing_part_two("<{o\"i!a,<{i<a>"), 10);
    }
}
