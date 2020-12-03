use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;
use std::time::Instant;

// Error handling policy : I trust the input file, so I unwrap everything with joy and recklessness :)

#[allow(dead_code)]
fn correctness_v1(pwd: &str, letter: &str, min: usize, max: usize) -> bool {
    let letter_count = pwd.matches(letter).count();
    letter_count >= min && letter_count <= max
}

#[allow(dead_code)]
fn correctness_v2(pwd: &str, letter: char, pos1: usize, pos2: usize) -> bool {
    let char1_ok = pwd.chars().nth(pos1).unwrap() == letter;
    let char2_ok = pwd.chars().nth(pos2).unwrap() == letter;
    char1_ok ^ char2_ok
}

fn is_line_correct(line : &str) -> bool {
    // this does not work if password contains ": "
    let mut vec = line.split(": ");
    let policy  = vec.next().unwrap();
    let pwd     = vec.next().unwrap();
    
    let mut policy = policy.split(" ");
    let bornes = policy.next().unwrap();
    let letter = policy.next().unwrap();

    let mut bornes = bornes.split("-");
    let min = bornes.next().unwrap().parse::<usize>().unwrap();
    let max = bornes.next().unwrap().parse::<usize>().unwrap();

    // correctness_v1(&pwd, &letter, min, max)
    correctness_v2(&pwd, letter.chars().nth(0).unwrap(), min - 1, max - 1)
}

fn main(){
    let start = Instant::now();

    let f = File::open("input.txt").unwrap();
    let f = BufReader::new(f);

    let mut pwd_ok_cnt = 0;
    for line in f.lines() {
        if is_line_correct(&line.unwrap()) {
            pwd_ok_cnt += 1;
        }
    }

    println!("{}", pwd_ok_cnt);
    
    let duration = start.elapsed();
    println!("Finished after {:?}", duration);
}