use std::fs::File;
use std::path::Path;
use std::io::BufReader;
use std::io::prelude::*;

const TARGET: i32 = 2020;

fn main() {
    let mut ls = ints_from_file("./day-1-input.txt");
    ls.sort();
    'i: for i in 0..ls.len() {
        'j: for j in i..ls.len() {
            let l1 = ls[i];
            let l2 = ls[j];
            let sum = ls[i] + ls[j];
            println!("{}, {}", l1, l2);
            if sum == TARGET {
                println!("{} * {} = {}", l1, l2, l1 * l2);
                break 'i;
            } else if sum > TARGET {
                break 'j;
            }
        }
    }
}

fn ints_from_file(filename: impl AsRef<Path>) -> Vec<i32> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .map(|x| x.parse::<i32>().unwrap())
        .collect()
}

