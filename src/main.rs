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
    let lines = lines_from_file("./day-2-input.txt");
    for line in lines {
        let words: Vec<&str> = line.split_whitespace().collect();
        let min_max: Vec<i32> = words[0].split("-").collect::<Vec<&str>>().iter().map(|line| line.parse::<i32>().unwrap()).collect();
        let chr = &words[1][0..1];
        let pw = words[2];
        println!("min_max: {:?}, chr: {}, pw: {}", min_max, chr, pw);
    }
}

fn 

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
