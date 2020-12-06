use std::fs::File;
use std::path::Path;
use std::io::BufReader;
use std::io::prelude::*;

fn main() {
    // day1();
    day2();
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
    let valid_password_count = lines_from_file("./day-2-input.txt").iter().map(day_2_parse_line).filter(password_valid).count();
    println!("Valid password count: {:#?}", valid_password_count);
}

fn password_valid(passwords_and_restrictions: &PasswordAndRestrictions) -> bool {
    let substring_count = passwords_and_restrictions.password.matches(&passwords_and_restrictions.substring).count();
    substring_count >= passwords_and_restrictions.min_substring as usize && substring_count <= passwords_and_restrictions.max_substring as usize
}

#[derive(Debug)]
struct PasswordAndRestrictions {
    password: String,
    substring: String,
    min_substring: i32,
    max_substring: i32
}

fn day_2_parse_line(line: &String) -> PasswordAndRestrictions {
    let words: Vec<&str> = line.split_whitespace().collect();
    let min_max: Vec<i32> = words[0].split("-").collect::<Vec<&str>>().iter().map(|line| line.parse::<i32>().unwrap()).collect();
    let substring = &words[1][0..1];
    let password = words[2];
    PasswordAndRestrictions { password: password.to_string(), substring: substring.to_string(), min_substring: min_max[0], max_substring: min_max[1] }
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
