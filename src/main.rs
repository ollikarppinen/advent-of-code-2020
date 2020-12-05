use std::fs::File;
use std::path::Path;
use std::io::BufReader;
use std::io::prelude::*;

fn main() {
    let ls = lines_from_file("./day-1-input.txt");
    'outer: for l in &ls {
        for ll in &ls {
            let l = l.parse::<i32>().unwrap();
            let ll =  ll.parse::<i32>().unwrap();
            if l + ll == 2020 {
                println!("{} + {} = 2020", l, ll);
                println!("{} * {} = {}", l, ll, l * ll);
                break 'outer;
            }
        }
    }
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

