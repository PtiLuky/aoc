use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;
use std::time::Instant;
use std::collections::BTreeSet;

/// TYPES /////////////////////////////////////////////////////////////////////

/// FUNCTIONS /////////////////////////////////////////////////////////////////
fn count_permuts(adapters: &BTreeSet<u32>, target: u32) -> u64 {
    // in vec permuts: permuts[i] is the number of permuts allowing to have the joltage i
    let mut permuts: Vec<u64> = vec![0; (target - 2) as usize];

    permuts[0] = 1; // first input is 1 : one way to have it
    for i in adapters {
        let idx = *i as usize;
        if idx >= 3 {
            permuts[idx] += permuts[idx - 3];
        } 
        if idx >= 2 {
            permuts[idx] += permuts[idx - 2];
        }
        permuts[idx] += permuts[idx - 1];
    }
    *permuts.last().unwrap()
}


fn count_d1_d3(adapters: &BTreeSet<u32>) -> (u32, u32) {
    let mut delta1 = 1; // why start counters with 1 instead of 0? Good question...
    let mut delta3 = 1; // why start counters with 1 instead of 0? Good question...

    let mut iter = adapters.iter();
    let mut previous = iter.next().unwrap();
    for next in iter {
        if next - previous == 1 {
            delta1 += 1;
        } else if next - previous == 3 {
            delta3 += 1;
        }
        previous = next;
    }
    (delta1, delta3)
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
        
        let adapters = read_input(&file);
        let device = adapters.iter().max().unwrap() + 3;
        let (d1, d3) = count_d1_d3(&adapters);
        answer1 = d1 * d3;
        answer2 = count_permuts(&adapters, device);
///////////////////////////////////////
    }
    let duration = start.elapsed();
    
    println!("\nd1*d3={}, permuts={}", answer1, answer2);
    println!("[Time] Finished after {:?} (mean on {} iterations)\n", duration / iter_cnt, iter_cnt);
}


fn read_input(path: &str) -> BTreeSet<u32> { 
    let mut nums = BTreeSet::<u32>::new();
    let f = File::open(path).unwrap();
    let f = BufReader::new(f);
    for line in f.lines() {
        nums.insert(line.unwrap().parse::<u32>().unwrap());
    }
    nums
}
