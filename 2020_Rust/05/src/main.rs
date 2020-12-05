use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;
use std::time::Instant;

fn compute_id(seat_name: &str) -> usize {
    let mut id = 0;
    id += if seat_name.chars().nth(0) == Some('B') { 1 << 9} else { 0 } ;
    id += if seat_name.chars().nth(1) == Some('B') { 1 << 8} else { 0 } ;
    id += if seat_name.chars().nth(2) == Some('B') { 1 << 7} else { 0 } ;
    id += if seat_name.chars().nth(3) == Some('B') { 1 << 6} else { 0 } ;
    id += if seat_name.chars().nth(4) == Some('B') { 1 << 5} else { 0 } ;
    id += if seat_name.chars().nth(5) == Some('B') { 1 << 4} else { 0 } ;
    id += if seat_name.chars().nth(6) == Some('B') { 1 << 3} else { 0 } ;
    id += if seat_name.chars().nth(7) == Some('R') { 1 << 2} else { 0 } ;
    id += if seat_name.chars().nth(8) == Some('R') { 1 << 1} else { 0 } ;
    id += if seat_name.chars().nth(9) == Some('R') { 1 << 0} else { 0 } ;
    id
}

#[allow(dead_code)]
fn find_highest(path: &str) -> usize {
    let f = File::open(path).unwrap();
    let f = BufReader::new(f);

    let mut highest = 0;
    for line in f.lines() {
        let line = line.unwrap();
        let seat_id = compute_id(&line[..]);
        if seat_id > highest {
            highest = seat_id;
        }
    }

    highest
}

fn find_my_place(path: &str) -> usize {
    let f = File::open(path).unwrap();
    let f = BufReader::new(f);
    let mut found_seats : Vec<bool> = vec![false; 1023];
    
    for line in f.lines() {
        let seat_id = compute_id(&line.unwrap()[..]);
        found_seats[seat_id] = true;
    }

    for i in 1..1024 {
        if found_seats[i - 1] && !found_seats[i] && found_seats[i + 1] {
            return i
        }
    }

    0
}

fn main() -> Result<(), std::io::Error> {
    let iter_cnt = 1000;
    let start = Instant::now();
    for i in 0..iter_cnt {
///////////////////////////////////////
        // let file = "test.txt";
        let file = "input.txt";
        
        let my_place = find_my_place(&file);
        let highest = find_highest(&file);

        if i == 0 {
            println!("Highest: {}", highest);
            println!("My place: {}", my_place);
        }
///////////////////////////////////////
    }
    let duration = start.elapsed();
    println!("Finished after {:?} (mean on {} iterations)", duration / iter_cnt, iter_cnt);
    Ok(())
}