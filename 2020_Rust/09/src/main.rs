use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;
use std::time::Instant;

/// TYPES /////////////////////////////////////////////////////////////////////

/// FUNCTIONS /////////////////////////////////////////////////////////////////
fn find_error(list: &Vec::<u64>, window_size: usize) -> u64 {
    let w = window_size;
    for i in w..list.len() {
        let mut found = false;
        for i1 in (i-w)..i {
            for i2 in (i1+1)..i {
                if list[i1] + list[i2] == list[i] {
                    found = true;
                    break;
                }
                if found { break; }
            }
        }
        if !found { return list[i] }
    }
    0
}

fn seq_sum_to(list: &Vec::<u64>, target: u64) -> &[u64] {
    let size = list.len();
    for i in (1..size).rev() {
        let mut sum = list[i];
        for j in (0..i).rev() {
            sum += list[j];
            if sum == target {
                return &list[j..i];
            } else if sum > target {
                break;
            }
        }
    }
    &list[0..0]
}

/// MAIN and PARSE ////////////////////////////////////////////////////////////
fn main() {
    let iter_cnt = 100;
    let start = Instant::now();
    let mut answer1 = 0;
    let mut answer2 = 0;
    for _ in 0..iter_cnt {
///////////////////////////////////////
        //let file = "test.txt";
        let file = "input.txt";
        
        let nums = read_input(&file);
        //answer = find_error(&nums, 5);
        answer1 = find_error(&nums, 25);
        let seq = seq_sum_to(&nums, answer1);
        answer2 = seq.iter().min().unwrap() + seq.iter().max().unwrap();

///////////////////////////////////////
    }
    let duration = start.elapsed();
    
    println!("\n{} ; {}", answer1, answer2);
    println!("[Time] Finished after {:?} (mean on {} iterations)\n", duration / iter_cnt, iter_cnt);
}

fn read_input(path: &str) -> Vec<u64> { 
    let mut nums = Vec::<u64>::new();
    let f = File::open(path).unwrap();
    let f = BufReader::new(f);
    for line in f.lines() {
        nums.push(line.unwrap().parse::<u64>().unwrap());
    }
    nums
}
