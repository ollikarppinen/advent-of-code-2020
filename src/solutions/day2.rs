use helpers;

pub fn run () -> () {
    let part_1_solution = helpers::lines_from_file("./day-2-input.txt").iter().map(parse_line).filter(password_valid_part_1).count();
    println!("Day 2, Part 1 - valid password count: {:#?}", part_1_solution);
    let part_2_solution = helpers::lines_from_file("./day-2-input.txt").iter().map(parse_line).filter(password_valid_part_2).count();
    println!("Day 2, Part 2 - valid password count: {:#?}", part_2_solution);
}

fn password_valid_part_1(passwords_and_restrictions: &PasswordAndRestrictions) -> bool {
    let substring_count = passwords_and_restrictions.password.matches(&passwords_and_restrictions.substring).count();
    substring_count >= passwords_and_restrictions.range_start_int && substring_count <= passwords_and_restrictions.range_end_int
}

fn password_valid_part_2(passwords_and_restrictions: &PasswordAndRestrictions) -> bool {
    let range_start_index = passwords_and_restrictions.range_start_int - 1;
    let range_end_index = passwords_and_restrictions.range_end_int - 1;
    let password_len = passwords_and_restrictions.password.len();
    if range_start_index > password_len || range_end_index > password_len { return false }

    let password_chars: Vec<char> = passwords_and_restrictions.password.chars().collect();
    let substring_char = passwords_and_restrictions.substring.chars().nth(0).unwrap();

    (
        password_chars[range_start_index] == substring_char && password_chars[range_end_index] != substring_char
    ) || (
        password_chars[range_start_index] != substring_char && password_chars[range_end_index] == substring_char
    )
}

#[derive(Debug)]
struct PasswordAndRestrictions {
    password: String,
    substring: String,
    range_start_int: usize,
    range_end_int: usize
}

fn parse_line(line: &String) -> PasswordAndRestrictions {
    let words: Vec<&str> = line.split_whitespace().collect();
    let min_max: Vec<usize> = words[0].split("-").collect::<Vec<&str>>().iter().map(|line| line.parse::<usize>().unwrap()).collect();
    let substring = &words[1][0..1];
    let password = words[2];
    PasswordAndRestrictions { password: password.to_string(), substring: substring.to_string(), range_start_int: min_max[0], range_end_int: min_max[1] }
}