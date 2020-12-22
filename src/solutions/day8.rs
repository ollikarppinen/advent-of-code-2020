use helpers;
use std::collections::HashSet;

pub fn run () -> () {
    let lines = helpers::lines_from_file("./day-8-input.txt");
    let mut i = 0;
    let mut acc = 0;
    let mut is: HashSet::<i32> = HashSet::new();
    loop {
        is.insert(i);
        let splits = lines[i as usize].split(" ").collect::<Vec<&str>>();
        let cmd = splits[0];
        let arg = splits[1].parse::<i32>().unwrap();
        i += if cmd == "jmp" { arg } else { 1 };
        if is.contains(&i) {
            break;
        } else {
            is.insert(i);
        }
        if i >= lines.len() as i32 { break; }
        if cmd == "acc" { acc += arg; }
    }
    println!("Day 8, part 1: {}", acc);
}
