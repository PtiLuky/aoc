use std::fs;
use std::time::Instant;
use std::collections::HashMap;

/// TYPES /////////////////////////////////////////////////////////////////////

/// FUNCTIONS /////////////////////////////////////////////////////////////////
fn play(nums: &Vec<usize>, last_turn: usize) -> usize {
    let max = nums.iter().max().unwrap() + 1;
    let mut ages_log_previous : HashMap<usize, usize> = HashMap::new();
    let mut ages_log_current  : HashMap<usize, usize> = HashMap::new();
    let mut last_said = 0;

    for i in 0..nums.len() {
        ages_log_current.insert(nums[i], i);
        last_said = nums[i];
    }

    for i in nums.len()..last_turn {
        let next = match ages_log_previous.get(&last_said) {
            Some(val) => ages_log_current[&last_said] - *val,
            None => 0
        };

        match ages_log_current.get(&next) {
            Some(val) => { ages_log_previous.insert(next, *val); },
            None => {},
        };
        ;
        ages_log_current.insert(next, i);
        last_said = next;
    }

    last_said
}

/// MAIN and PARSE ////////////////////////////////////////////////////////////
fn main() {
    let iter_cnt = 1;
    let start = Instant::now();
    let mut answer1 = 0;
    let mut answer2 = 0;
    for _ in 0..iter_cnt {
///////////////////////////////////////
        //let file = "test.txt";
        let file = "input.txt";

        let nums = read_input(&file);
        answer1 = play(&nums, 2020);
        answer2 = play(&nums, 30000000);

///////////////////////////////////////
    }
    println!("\nanswer1={}, answer2={}", answer1, answer2);
    println!("[Time] Finished after {:?} (mean on {} iterations)\n", start.elapsed() / iter_cnt, iter_cnt);
}


fn read_input(path: &str) -> Vec<usize> { 
    let mut nums = Vec::<usize>::new();
    let f = fs::read_to_string(path).unwrap();

    for n in f.split(",") {
        nums.push(n.parse().unwrap());
    }
    nums
}
