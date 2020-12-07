use helpers;

pub fn run () -> () {
    let lines = helpers::lines_from_file("./day-3-input.txt");
    let mut xs = [0, 0, 0, 0, 0];
    let mut counts = [0, 0, 0, 0, 0];
    let mut i = 0;
    for line in lines {
        let chars: Vec<char> = line.chars().collect();
        if chars[xs[0] % line.len()] == '#' { counts[0] += 1 }
        if chars[xs[1] % line.len()] == '#' { counts[1] += 1 }
        if chars[xs[2] % line.len()] == '#' { counts[2] += 1 }
        if chars[xs[3] % line.len()] == '#' { counts[3] += 1 }
        if i % 2 == 0 {
            if chars[xs[4] % line.len()] == '#' { counts[4] += 1 }
            xs[4] += 1;
        }
        xs[0] += 1;
        xs[1] += 3;
        xs[2] += 5;
        xs[3] += 7;
       
        i += 1;
    }
    println!("Day 3, part 1: {:?}", counts);
    println!("Day 3, part 1: {}", counts.iter().fold(1, |a, b| a * b));
}