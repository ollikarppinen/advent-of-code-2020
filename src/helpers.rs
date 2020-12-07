use std::fs::File;
use std::path::Path;
use std::io::BufReader;
use std::io::prelude::*;

pub fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

pub fn ints_from_file(filename: impl AsRef<Path>) -> Vec<i32> {
    lines_from_file(filename)
        .iter()
        .map(|line| line.parse::<i32>().unwrap())
        .collect()
}
