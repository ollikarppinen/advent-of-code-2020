use std::fs::File;
use std::path::Path;
use std::io::BufReader;
use std::io::prelude::*;


fn main() {
    day1();
}

fn ints_from_file(filename: impl AsRef<Path>) -> Vec<i32> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .map(|x| x.parse::<i32>().unwrap())
        .collect()
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

