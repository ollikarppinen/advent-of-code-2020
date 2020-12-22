use helpers;
use std::collections::HashSet;

pub fn run () -> () {
    let lines = helpers::lines_from_file("./day-8-input.txt");
    let mut i = 0;
    let mut mutation = 0;
    let mut mutation_i = 0;
    let mut acc = 0;
    let mut is: HashSet::<i32> = HashSet::new();
    let line_count = lines.len();
    let mut mutated = false;
    loop {
        let splits = lines[i as usize].split(" ").collect::<Vec<&str>>();
        let mut cmd = splits[0];
        if !mutated && cmd == "jmp" || cmd == "nop" {
            if mutation == mutation_i {
                println!("Mutation: {} / {}", mutation, line_count);
                println!("i: {}", i);
                cmd = if cmd == "jmp" { "nop" } else { "jmp" };
                mutated = true;
            }
            mutation_i += 1;
        }
        let arg = splits[1].parse::<i32>().unwrap();
        if i == lines.len() as i32 - 1 {
            println!("Last line!");
            acc += if cmd == "acc" { arg } else { 1 };
            break;
        }
        i += if cmd == "jmp" { arg } else { 1 };
        if is.contains(&i) {
            is = HashSet::new();
            mutated = false;
            mutation += 1;
            mutation_i = 0;
            i = 0;
            acc = 0;
        } else {
            is.insert(i);
        }
        if i >= lines.len() as i32 {
            println!("i out of bounds!");
            break;
        }
        if cmd == "acc" { acc += arg; }
    }
    println!("Day 8, part 2: {}", acc);
}
