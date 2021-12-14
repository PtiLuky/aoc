use std::fs;
use std::cmp::Ordering;

/// TYPES /////////////////////////////////////////////////////////////////////

/// FUNCTIONS /////////////////////////////////////////////////////////////////

/// MAIN and PARSE ////////////////////////////////////////////////////////////
fn main() {
    let file = "input.txt";
    let vals = read_input(&file);

    let c1 = best_pos(&vals);
    let c2 =best_pos2(&vals);
   
    println!("\nanswer1={}, answer2={}", c1, c2);
}

fn read_input(path: &str) -> Vec<u32> {
    let txt = fs::read_to_string(path).unwrap();
    txt.lines()
        .next()
        .unwrap()
        .split(",")
        .map(|v| v.parse::<u32>().unwrap())
        .collect()
}

// https://rust-lang-nursery.github.io/rust-cookbook/science/mathematics/statistics.html

fn partition(data: &[u32]) -> Option<(Vec<u32>, u32, Vec<u32>)> {
    match data.len() {
        0 => None,
        _ => {
            let (pivot_slice, tail) = data.split_at(1);
            let pivot = pivot_slice[0];
            let (left, right) = tail.iter().fold((vec![], vec![]), |mut splits, next| {
                {
                    let (ref mut left, ref mut right) = &mut splits;
                    if next < &pivot {
                        left.push(*next);
                    } else {
                        right.push(*next);
                    }
                }
                splits
            });

            Some((left, pivot, right))
        }
    }
}

fn select(data: &[u32], k: usize) -> Option<u32> {
    let part = partition(data);

    match part {
        None => None,
        Some((left, pivot, right)) => {
            let pivot_idx = left.len();

            match pivot_idx.cmp(&k) {
                Ordering::Equal => Some(pivot),
                Ordering::Greater => select(&left, k),
                Ordering::Less => select(&right, k - (pivot_idx + 1)),
            }
        }
    }
}

fn median(data: &[u32]) -> Option<u32> {
    let size = data.len();

    match size {
        even if even % 2 == 0 => {
            let fst_med = select(data, (even / 2) - 1);
            let snd_med = select(data, even / 2);

            match (fst_med, snd_med) {
                (Some(fst), Some(snd)) => Some((fst + snd) / 2),
                _ => None,
            }
        }
        odd => select(data, odd / 2),
    }
}

fn mean(numbers: &[u32]) -> u32 {
    (numbers.iter().sum::<u32>() as f32 / numbers.len() as f32).round() as u32
}

fn best_pos(vals: &Vec<u32>) -> u64 {
    let pos = median(vals).unwrap();
    let mut diff = 0;
    for v in vals.iter() {
        diff += (*v as i64 - pos as i64).abs() as u64;
    }
    diff
}

fn best_pos2(vals: &Vec<u32>) -> u64 {
    let pos = mean(vals) - 1;
    let mut fuel = 0;
    for v in vals.iter() {
        let diff = (*v as i64 - pos as i64).abs() as u64;
        fuel += diff * (1 + diff) / 2;
    }
    fuel
}
