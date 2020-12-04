use regex::Regex;
use hex;
use std::fs;
use std::time::Instant;

// OMG using those regex is so slow => 500 ms for whole input file
#[allow(dead_code)]
fn value_valid_regex(field_type: &str, value: &str) -> bool {
    match field_type {
        "byr" => match value.parse::<u32>() {
            Ok(year) => (year >= 1920 && year <= 2002),
            Err(_) => false,
        },
        "iyr" => match value.parse::<u32>() {
            Ok(year) => (year >= 2010 && year <= 2020),
            Err(_) => false,
        },
        "eyr" => match value.parse::<u32>() {
            Ok(year) => (year >= 2020 && year <= 2030),
            Err(_) => false,
        },
        "hgt" => {
            let re_cm = Regex::new(r"^[0-9]{3}cm$").unwrap();
            let re_in = Regex::new(r"^[0-9]{2}in$").unwrap();
            if re_cm.is_match(value) {
                match value[0..3].parse::<u32>() {
                    Ok(size) => (size >= 150 && size <= 193),
                    Err(_) => false,
                }
            } else if re_in.is_match(value) {
                match value[0..2].parse::<u32>() {
                    Ok(size) => (size >= 59 && size <= 76),
                    Err(_) => false,
                }
            } else {
                false
            }
        },
        "hcl" => Regex::new(r"^#[0-9a-f]{6}$").unwrap().is_match(value),
        "ecl" => Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap().is_match(value),
        "pid" => Regex::new(r"^[0-9]{9}$").unwrap().is_match(value),
        "cid" => true,
        _ => false,
    }
}

fn value_valid(field_type: &str, value: &str) -> bool {
    match field_type {
        "byr" => match value.parse::<u32>() {
            Ok(year) => (year >= 1920 && year <= 2002),
            Err(_) => false,
        },
        "iyr" => match value.parse::<u32>() {
            Ok(year) => (year >= 2010 && year <= 2020),
            Err(_) => false,
        },
        "eyr" => match value.parse::<u32>() {
            Ok(year) => (year >= 2020 && year <= 2030),
            Err(_) => false,
        },
        "hgt" => {
            if value.chars().count() == 4 && value.ends_with("in") {
                match value[0..2].parse::<u32>() {
                    Ok(size) => (size >= 59 && size <= 76),
                    Err(_) => false,
                }
            } else if value.chars().count() == 5 && value.ends_with("cm") {
                match value[0..3].parse::<u32>() {
                    Ok(size) => (size >= 150 && size <= 193),
                    Err(_) => false,
                }
            } else { 
                false 
            }
        },
        "hcl" => 
            if value.chars().count() == 7 && value.starts_with('#') { 
                hex::decode(&value[1..7]).is_ok() 
            } else { 
                false 
            },
        "ecl" => match value {
            "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => true,
            _ => false,
        }
        "pid" => 
            if value.chars().count() == 9 {
                value.parse::<u32>().is_ok()
            } else {
                false
            }
        "cid" => true,
        _ => false,
    }
}

fn count_valids(filename: &str) -> Result<u32, std::io::Error> {
    let fields        = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid", "cid"];
    let accepted_miss = vec!["cid"];

    let mut ok_count = 0;
    let input     = fs::read_to_string(filename)?;
    let passports = input.split("\r\n\r\n");
    for passport in passports {
        let mut fields_needed = fields.clone();
        let mut valid = true;

        // Remove not needed ones
        for ok_miss in &accepted_miss {
            fields_needed.retain(|&x| &x != ok_miss);
        }

        // Remove read ones
        let fields_read  = passport.split_whitespace();
        for field in fields_read {
            let mut splitted = field.split(":");
            let type_read    = splitted.next().unwrap();
            let value_read   = splitted.next().unwrap();
            fields_needed.retain(|&x| x != type_read);
            // valid &= value_valid_regex(type_read, value_read);
            valid &= value_valid(type_read, value_read);
        }

        if fields_needed.len() == 0 && valid {
            ok_count += 1;
        }
    }
    Ok(ok_count)
}

fn main() -> Result<(), std::io::Error> {
    let iter_cnt = 5000;
    let start = Instant::now();
    for i in 0..iter_cnt {
///////////////////////////////////////
        // let valids = count_valids("test.txt")?;
        let valids = count_valids("input.txt")?;

        if i == 0 {
            println!("{} valids", valids);
        }
///////////////////////////////////////
    }
    let duration = start.elapsed();
    println!("Finished after {:?} (mean on {} iterations)", duration / iter_cnt, iter_cnt);
    Ok(())
}