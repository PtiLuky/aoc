use std::collections::HashMap;
use std::fs;

/// TYPES /////////////////////////////////////////////////////////////////////
type Rules = HashMap<(char, char), char>;

/// FUNCTIONS /////////////////////////////////////////////////////////////////

/// MAIN and PARSE ////////////////////////////////////////////////////////////
fn main() {
    let file = "input.txt";
    let (input, rules) = read_input(&file);

    let s = run_sim_dumb(&input, &rules, 10);
    let (a, b) = count_min_max(&s);
    let c1 = b - a;

    let s = run_sim_smart(&input, &rules, 40);
    let (a, b) = count_min_max_smart(&s);
    let c2 = b - a;

    println!("\nanswer1={}, answer2={}", c1, c2);
}

fn read_input(path: &str) -> (Vec<char>, Rules) {
    let txt = fs::read_to_string(path).unwrap();

    let mut input = Vec::new();
    let mut rules = HashMap::new();

    for (i, l) in txt.lines().enumerate() {
        if i == 0 {
            input = l.chars().collect();
        } else if i == 1 {
        } else {
            let a = l.chars().nth(0).unwrap();
            let b = l.chars().nth(1).unwrap();
            let c = l.chars().nth(6).unwrap();
            rules.insert((a, b), c);
        }
    }

    (input, rules)
}

fn run_sim_smart(input: &Vec<char>, rules: &Rules, ite: u32) -> HashMap<(char, char), usize> {
    let mut counts = HashMap::new();

    for idx in 0..(input.len() - 1) {
        *counts.entry((input[idx], input[idx + 1])).or_default() += 1;
    }

    for it in 0..ite {
        let mut counts_next = HashMap::new();
        for ((a, b), cnt) in counts.iter() {
            let (a, b) = (*a, *b);
            let c = *rules.get(&(a, b)).unwrap();
            *counts_next.entry((a, c)).or_default() += cnt;
            *counts_next.entry((c, b)).or_default() += cnt;
        }
        counts = counts_next;
    }
    counts
}

fn count_min_max_smart(input: &HashMap<(char, char), usize>) -> (usize, usize) {
    let mut counts = HashMap::new();
    for ((a, b), cnt) in input {
        *counts.entry(a).or_default() += cnt;
        *counts.entry(b).or_default() += cnt;
    }

    let mut least = 9999999999999;
    let mut most = 0;
    for (_, cnt) in counts.iter() {
        let cnt = *cnt;
        if cnt < least {
            least = cnt;
        }
        if cnt > most {
            most = cnt;
        }
    }
    println!("{}, {}", least, most);
    let least = ((least as f64) / 2.).ceil() as usize;
    let most = ((most as f64) / 2.).ceil() as usize;
    (least, most)
}

fn run_sim_dumb(input: &Vec<char>, rules: &Rules, ite: u32) -> Vec<char> {
    let mut data = input.clone();
    for it in 0..ite {
        let mut i = 0;
        while i < data.len() - 1 {
            match rules.get(&(data[i], data[i + 1])) {
                Some(c) => {
                    data.insert(i + 1, *c);
                    i += 1;
                },
                None => {}
            }
            i += 1;
        }
    }
    data
}

fn count_min_max(input: &Vec<char>) -> (usize, usize) {
    let mut counts = HashMap::new();
    for c in input {
        *counts.entry(c).or_default() += 1;
    }

    let mut least = 9999999999999;
    let mut most = 0;
    for (_, cnt) in counts.iter() {
        let cnt = *cnt;
        if cnt < least {
            least = cnt;
        }
        if cnt > most {
            most = cnt;
        }
    }
    (least, most)
}