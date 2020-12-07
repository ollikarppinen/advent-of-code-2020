use helpers;
use std::path::Path;
use regex::Regex;

pub fn run () -> () {
    println!("Regex match: {}", matches_regex(r"^\d{4}$", "2014"));
    let passports = parse_passports("./day-4-example-input.txt");
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
    byr_valid(passport.byr.as_ref()) &&
    iyr_valid(passport.iyr.as_ref()) &&
    eyr_valid(passport.eyr.as_ref()) &&
    hgt_valid(passport.hgt.as_ref()) &&
    hcl_valid(passport.hcl.as_ref()) &&
    ecl_valid(passport.ecl.as_ref()) &&
    pid_valid(passport.pid.as_ref()) &&
    cid_valid(passport.cid.as_ref())
}

fn byr_valid(byr: Option<&String>) -> bool {
    if byr.is_none() || !matches_regex(r"^\d{4}$", byr.unwrap()) { return false }
    let byr_int = byr.unwrap().parse::<i32>().unwrap();
    return byr_int >= 1920 && byr_int <= 2002;
}

fn iyr_valid(iyr: Option<&String>) -> bool {
    if iyr.is_none() || !matches_regex(r"^\d{4}$", iyr.unwrap()) { return false }
    let iyr_int = iyr.unwrap().parse::<i32>().unwrap();
    return iyr_int >= 2010 && iyr_int <= 2020;
}

fn eyr_valid(eyr: Option<&String>) -> bool {
    if eyr.is_none() || !matches_regex(r"^\d{4}$", eyr.unwrap()) { return false }
    let eyr_int = eyr.unwrap().parse::<i32>().unwrap();
    return eyr_int >= 2020 && eyr_int <= 2030;
}

fn hgt_valid(hgt: Option<&String>) -> bool {
    if hgt.is_none() { return false };

    if matches_regex(r"^\d{3}cm$", hgt.unwrap()) {
        let hgt_int = hgt.unwrap().replace("cm", "").parse::<i32>().unwrap();
        return hgt_int >= 150 && hgt_int <= 193;
    } else if matches_regex(r"^\d{2}in$", hgt.unwrap()) {
        let hgt_int = hgt.unwrap().replace("in", "").parse::<i32>().unwrap();
        return hgt_int >= 59 && hgt_int <= 76;
    }

    return false;
}

fn hcl_valid(hcl: Option<&String>) -> bool {
    if hcl.is_none() || !matches_regex(r"^#[0-9a-z]{6}$", hcl.unwrap()) { return false }
    return true;
}

fn ecl_valid(ecl: Option<&String>) -> bool {
    if ecl.is_none() { return false }
    match ecl.unwrap().as_str() {
       "amb" => return true,
       "blu" => return true,
       "brn" => return true,
       "gry" => return true,
       "grn" => return true,
       "hzl" => return true,
       "oth" => return true,
       _ => return false
    }
}

fn pid_valid(pid: Option<&String>) -> bool {
    return !pid.is_none()
}

fn cid_valid(_cid: Option<&String>) -> bool {
    return true
}

fn matches_regex(pattern: &str, string: &str) -> bool {
    let re = Regex::new(pattern).unwrap();
    return re.is_match(string);
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
