use helpers;
use std::collections::HashSet;

pub fn run () -> () {
    let lines = helpers::lines_from_file("./day-6-input.txt");
    let mut line_iter = lines.iter();
    let mut line: Option<&String>;
    let mut unique_answers: Option<HashSet::<char>> = None;
    let mut shared_answers: Option<HashSet::<char>> = None;
    let mut unique_answer_count = 0;
    let mut shared_answer_count = 0;
    loop {
        line = line_iter.next();
        if line.is_none() || line.unwrap().len() == 0 {
            unique_answer_count += unique_answers.unwrap().len();
            shared_answer_count += shared_answers.unwrap().len();
            shared_answers = None;
            unique_answers = None;
        } else {
            if shared_answers.is_none() || unique_answers.is_none() {
                shared_answers = Some(line.unwrap().chars().collect());
                unique_answers = Some(line.unwrap().chars().collect());
            } else {
                let answers = line.unwrap().chars().collect::<HashSet<char>>();
                shared_answers = Some(shared_answers.unwrap().intersection(&answers).cloned().collect());
                unique_answers = Some(unique_answers.unwrap().union(&answers).cloned().collect());
            }
        }
        if line.is_none() { break; }
    }
    println!("Day 6, part 1, unique answers: {}", unique_answer_count);
    println!("Day 6, part 2, shared answers: {}", shared_answer_count);
}