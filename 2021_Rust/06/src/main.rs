use std::fs;

/// TYPES /////////////////////////////////////////////////////////////////////

/// FUNCTIONS /////////////////////////////////////////////////////////////////

/// MAIN and PARSE ////////////////////////////////////////////////////////////
fn main() {
    let file = "input.txt";
    let vals = read_input(&file);

    let c1 = simulate_stats(&vals, 80);
    let c2 = simulate_stats(&vals, 256);

    println!("\nanswer1={}, answer2={}", c1, c2);
}

fn read_input(path: &str) -> Vec<u32> {
    let txt = fs::read_to_string(path).unwrap();
    txt.lines().next().unwrap().split(",").map(|v| v.parse::<u32>().unwrap()).collect()
}

fn simulate_dumb(mut vals: Vec<u32>, iterations: usize) -> Vec<u32>{
    for _ in 0..iterations {
        let size = vals.len();

        for idx in 0..size {
            let v = &mut vals[idx];
            if *v == 0 {
                *v = 6;
                vals.push(8);
            } else {
                *v -= 1;
            }
        }
    }
    vals
}

fn simulate_stats(vals: &Vec<u32>, iterations: usize) -> u64 {
    let mut stats = vec![0 as u64; 9];
    for v in vals.iter() {
        stats[*v as usize] += 1;
    }
    for i in 0..iterations {
        let nb_new = stats[0];
        for i in 0..8 {
            stats[i] = stats[i + 1];
        }
        stats[6] += nb_new;
        stats[8] = nb_new;
    }
    stats.iter().sum()
}