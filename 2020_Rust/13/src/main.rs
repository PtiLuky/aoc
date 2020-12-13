use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;
use std::time::Instant;

/// TYPES /////////////////////////////////////////////////////////////////////

/// FUNCTIONS /////////////////////////////////////////////////////////////////
fn delay_before_start(hour: usize, bus_period: usize) -> usize {
    if bus_period == 0 { return 0; }

    (bus_period - hour % bus_period) % bus_period
}

fn find_consecutive_multiples(buses: &Vec<usize>) -> usize{
    // 1. for a couple (x, y), find a multiple a P*x + 1 = Q*y, progressing of 1.
    // 2. then, redo the step with (y, z), but progressing with Least common multiple (x,y)
    // here x and y are prime, use x*y.

    let mut step : usize = 1;
    let mut time_bus_0 : usize = 0; // time of the first bus
    let mut i = 0;
    while i < buses.len() - 1 {
        // find x
        if buses[i] == 0 {
            i += 1;
            continue; // jokers
        }

        // find y
        let mut delta = 1;
        while i + delta < buses.len() && buses[i + delta] == 0 {
            delta += 1;
        }
        let x = buses[i];
        let y = buses[i + delta];

        // use a step of multiples
        step *= x;
        while time_bus_0 == 0 ||
            (time_bus_0 + i) % x != 0 ||
            (time_bus_0 + i + delta) % y != 0 
        {
            time_bus_0 += step;
        }

        // jump to next x (knowing next x is current y...)
        i = i + delta;
    }
    time_bus_0
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

        let (hour, buses) = read_input(&file);
        let mut delay = hour;
        for b in &buses {
            let b = *b;
            if b > 0 {
                let bus_delay = delay_before_start(hour, b);
                if bus_delay < delay {
                    delay = bus_delay;
                    answer1 = delay * b;
                }
            }
        }

        answer2 = find_consecutive_multiples(&buses);

///////////////////////////////////////
    }
    println!("\nanswer1={}, answer2={}", answer1, answer2);
    println!("[Time] Finished after {:?} (mean on {} iterations)\n", start.elapsed() / iter_cnt, iter_cnt);
}


fn read_input(path: &str) -> (usize, Vec<usize>) { 
    let mut nums = Vec::<usize>::new();
    let f = File::open(path).unwrap();
    let f = BufReader::new(f);
    let mut ite = f.lines();

    let hour = ite.next().unwrap().unwrap().parse::<usize>().unwrap();
    for id in ite.next().unwrap().unwrap().split(",") {
        nums.push(match id.parse::<usize>() {
            Ok(num) => num,
            Err(_) => 0
        });
    }
    (hour, nums)
}
