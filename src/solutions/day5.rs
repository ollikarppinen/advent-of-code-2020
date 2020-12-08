use helpers;
use std::path::Path;

pub fn run () -> () {
    let boarding_passes = parse_boarding_passes("./day-5-input.txt");
    let mut seat_ids: Vec<i32> = boarding_passes.iter().map(calculate_seat_id).collect();
    let highest_seat_id = seat_ids.iter().max().unwrap();
    println!("Day 5, part 1, highest seat id: {}", highest_seat_id);
    seat_ids.sort();
    let missing_seat_id = find_missing_seat_id(seat_ids);
    println!("Day 5, part 2, missing seat id: {}", missing_seat_id);
}

fn median(a: i32, b: i32) -> i32 {
    a + (b - a) / 2
}

fn find_missing_seat_id(ids: Vec<i32>) -> i32 {
    let mut latest_id = None;
    let mut missing_id = 0;
    for id in ids {
        if !latest_id.is_none() && latest_id.unwrap() + 2 == id {
            missing_id = latest_id.unwrap() + 1;
            break;
        }
        latest_id = Some(id);
    }
    missing_id
}

fn parse_boarding_passes(path: impl AsRef<Path>) -> Vec<BoardingPass> {
    return helpers::lines_from_file(path).iter().map(parse_boarding_pass).collect();
}

fn calculate_seat_id(boarding_pass: &BoardingPass) -> i32 {
    boarding_pass.row * 8 + boarding_pass.column
}

fn parse_boarding_pass(raw: &String) -> BoardingPass {
    let mut row_upper_i = 127;
    let mut row_lower_i = 0;
    let mut column_upper_i = 7;
    let mut column_lower_i = 0;
    for c in raw.chars() {
        match c {
            'F' => row_upper_i = if (row_upper_i - row_lower_i) > 1 { median(row_lower_i, row_upper_i) } else { row_lower_i },
            'B' => row_lower_i = if (row_upper_i - row_lower_i) > 1 { median(row_lower_i, row_upper_i) + 1 } else { row_upper_i },
            'R' => column_lower_i = if (column_upper_i - column_lower_i) > 1 { median(column_lower_i, column_upper_i) + 1 } else { column_upper_i },
            'L' => column_upper_i = if (column_upper_i - column_lower_i) > 1 { median(column_lower_i, column_upper_i) } else { column_lower_i },
            _ => ()
        }
    }
    return BoardingPass { raw: raw.to_string(), row: row_upper_i, column: column_upper_i };
}

#[derive(Debug)]
struct BoardingPass {
    raw: String,
    row: i32,
    column: i32
}
