use helpers;

pub fn run () -> () {
    let target = 2020;
    let mut ls = helpers::ints_from_file("./day-1-input.txt");
    ls.sort();
    'i: for i in 0..ls.len() {
        let a = ls[i];
        for j in i..ls.len() {
            let b = ls[j];
            'k: for k in j..ls.len() {
                let c = ls[k];
                let sum = a + b + c;

                if sum == target {
                    println!("Day 1, part 2: {} * {} * {} = {}", a, b, c, a * b * c);
                    break 'i;
                } else if sum > target {
                    break 'k;
                }
            }
        }
    }
}