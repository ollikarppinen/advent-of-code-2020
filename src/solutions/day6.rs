use helpers;
use itertools::Itertools;

pub fn run () -> () {
    let lines = helpers::lines_from_file("./day-6-input.txt");
    let mut line_iter = lines.iter();
    let mut line: Option<&String>;
    let mut grouped_lines: Vec<String> = vec![];
    let mut answers = 0;
    loop {
        line = line_iter.next();
        if line.is_none() || line.unwrap().len() == 0 {
            let unique_answers = grouped_lines.join("").chars().collect::<Vec<char>>().iter().unique().count();
            answers += unique_answers;
            grouped_lines = vec![];
        } else {
            grouped_lines.push(line.unwrap().to_string());
        }
        if line.is_none() {
            break;
        }
    }
    println!("Day 6, part 1, unique asnwers: {}", answers);
}