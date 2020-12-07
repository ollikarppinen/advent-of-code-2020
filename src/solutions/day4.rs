use helpers;
use std::path::Path;

pub fn run () -> () {
    let passports = parse_passports("./day-4-input.txt");
    let valid_passport_count = passports.iter().filter(&&valid_passport).count();
    println!("Day 4, part 1: valid passport count: {:?}", valid_passport_count);
}

#[derive(Debug)]
struct Passport {
    byr: Option<String>,
    iyr: Option<String>,
    eyr: Option<String>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<String>
}

fn valid_passport(passport: &&Passport) -> bool {
    !(
        passport.byr.is_none() ||
        passport.iyr.is_none() ||
        passport.eyr.is_none() ||
        passport.hgt.is_none() ||
        passport.hcl.is_none() ||
        passport.ecl.is_none() ||
        passport.pid.is_none()
    )
}

fn parse_passports(path: impl AsRef<Path>) -> Vec<Passport> {
    let lines = helpers::lines_from_file(path).join(" ");
    let passport_strs = lines.split("  ").collect::<Vec<&str>>();
    let mut passports: Vec<Passport> = Vec::new();
    for passport_str in passport_strs {
        let field_strs = passport_str.split(" ");
        let mut passport = Passport {
            byr: None,
            iyr: None,
            eyr: None,
            hgt: None,
            hcl: None,
            ecl: None,
            pid: None,
            cid: None
        };
        for field_str in field_strs {
            let field_value = field_str.split(":").collect::<Vec<&str>>();
            let field_name = field_value[0];
            let field_value = field_value[1].to_string();
            match field_name {
                "byr" => passport.byr = Some(field_value),
                "iyr" => passport.iyr = Some(field_value),
                "eyr" => passport.eyr = Some(field_value),
                "hgt" => passport.hgt = Some(field_value),
                "hcl" => passport.hcl = Some(field_value),
                "ecl" => passport.ecl = Some(field_value),
                "pid" => passport.pid = Some(field_value),
                "cid" => passport.cid = Some(field_value),
                _ => ()
            }
        }
        passports.push(passport);
    }
    return passports;
}
