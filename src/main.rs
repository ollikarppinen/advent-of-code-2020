use std::fs::File;
use std::path::Path;
use std::io::BufReader;
use std::io::prelude::*;

fn main() {
    // day1();
    // day2();
    day3();
}

fn day1 () -> () {
    let target = 2020;
    let mut ls = ints_from_file("./day-1-input.txt");
    ls.sort();
    'i: for i in 0..ls.len() {
        let a = ls[i];
        for j in i..ls.len() {
            let b = ls[j];
            'k: for k in j..ls.len() {
                let c = ls[k];
                let sum = a + b + c;
                if sum == target {
                    println!("{} * {} * {} = {}", a, b, c, a * b * c);
                    break 'i;
                } else if sum > target {
                    break 'k;
                }
            }
        }
    }
}

fn day2 () -> () {
    let part_1_solution = lines_from_file("./day-2-input.txt").iter().map(day_2_parse_line).filter(password_valid_part_1).count();
    println!("Part 1 - valid password count: {:#?}", part_1_solution);
    let part_2_solution = lines_from_file("./day-2-input.txt").iter().map(day_2_parse_line).filter(password_valid_part_2).count();
    println!("Part 2 - valid password count: {:#?}", part_2_solution);
}

fn day3 () -> () {
    let lines = lines_from_file("./day-3-input.txt");
    let mut x = 0;
    let mut count = 0;
    for line in lines {
        if line.chars().nth(x % line.len()).unwrap() == '#' { count += 1 }
        x += 3;
    }
    println!("Day 3, part 1: {}", count);
}

fn password_valid_part_1(passwords_and_restrictions: &PasswordAndRestrictions) -> bool {
    let substring_count = passwords_and_restrictions.password.matches(&passwords_and_restrictions.substring).count();
    substring_count >= passwords_and_restrictions.range_start_int && substring_count <= passwords_and_restrictions.range_end_int
}

fn password_valid_part_2(passwords_and_restrictions: &PasswordAndRestrictions) -> bool {
    let range_start_index = passwords_and_restrictions.range_start_int - 1;
    let range_end_index = passwords_and_restrictions.range_end_int - 1;
    let password_len = passwords_and_restrictions.password.len();
    if range_start_index > password_len || range_end_index > password_len { return false }

    let password_chars: Vec<char> = passwords_and_restrictions.password.chars().collect();
    let substring_char = passwords_and_restrictions.substring.chars().nth(0).unwrap();

    (
        password_chars[range_start_index] == substring_char && password_chars[range_end_index] != substring_char
    ) || (
        password_chars[range_start_index] != substring_char && password_chars[range_end_index] == substring_char
    )
}

#[derive(Debug)]
struct PasswordAndRestrictions {
    password: String,
    substring: String,
    range_start_int: usize,
    range_end_int: usize
}

fn day_2_parse_line(line: &String) -> PasswordAndRestrictions {
    let words: Vec<&str> = line.split_whitespace().collect();
    let min_max: Vec<usize> = words[0].split("-").collect::<Vec<&str>>().iter().map(|line| line.parse::<usize>().unwrap()).collect();
    let substring = &words[1][0..1];
    let password = words[2];
    PasswordAndRestrictions { password: password.to_string(), substring: substring.to_string(), range_start_int: min_max[0], range_end_int: min_max[1] }
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn ints_from_file(filename: impl AsRef<Path>) -> Vec<i32> {
   lines_from_file(filename)
    .iter()
    .map(|line| line.parse::<i32>().unwrap())
    .collect()
}
