use std::fs;

/// TYPES /////////////////////////////////////////////////////////////////////

/// FUNCTIONS /////////////////////////////////////////////////////////////////

/// MAIN and PARSE ////////////////////////////////////////////////////////////
fn main() {
    let file = "input.txt";

    let c1 = count_obvious(&file);
    let c2 = sum_all(&file);

    println!("\nanswer1={}, answer2={}", c1, c2);
}

fn count_obvious(path: &str) -> u32 {
    let txt = fs::read_to_string(path).unwrap();
    let mut count = 0;

    for l in txt.lines() {
        let mut parts = l.split(" | ");
        let _left = parts.next().unwrap();
        let right = parts.next().unwrap();

        for r in right.split_whitespace() {
            match r.len() {
                2 | 4 | 3 | 7 => count += 1,
                _ => {}
            }
        }
    }

    count
}

fn nb_letter_common(s1: &str, s2: &str) -> usize {
    s1.chars().filter(|&c| s2.contains(c)).count()
}

fn is(s1: &str, s2: &str) -> bool {
    s1.len() == s2.len() && nb_letter_common(s1, s2) == s1.len()
}

fn decode_line(txt: &str) -> u32 {
    let mut parts = txt.split(" | ");
    let left = parts.next().unwrap();
    let right = parts.next().unwrap();
    let mut left: Vec<&str> = left.split_whitespace().collect();

    // Find uniques: 1, 4, 7, 8
    let v1 = left.iter().find(|&v| v.len() == 2).unwrap().to_string();
    let v4 = left.iter().find(|&v| v.len() == 4).unwrap().to_string();
    let v7 = left.iter().find(|&v| v.len() == 3).unwrap().to_string();
    let v8 = left.iter().find(|&v| v.len() == 7).unwrap().to_string();
    left.retain(|v| v != &v1 && v != &v4 && v != &v7 && v != &v8);

    let mut top_left_mid = v4.to_string();
    top_left_mid.retain(|v| !v1.contains(v));

    // Find 6 segments: 0, 6, 9
    let v6 = left
        .iter()
        .find(|&v| v.len() == 6 && nb_letter_common(v, &v1) == 1)
        .unwrap()
        .to_string();
    left.retain(|v| v != &v6);
    let v0 = left
        .iter()
        .find(|&v| v.len() == 6 && nb_letter_common(v, &top_left_mid) == 1)
        .unwrap()
        .to_string();
    left.retain(|v| v != &v0);
    let v9 = left.iter().find(|&v| v.len() == 6).unwrap().to_string();
    left.retain(|v| v != &v9);

    // find 5 segments: 2, 3, 5
    let v5 = left
        .iter()
        .find(|&v| nb_letter_common(v, &top_left_mid) == 2)
        .unwrap()
        .to_string();
    left.retain(|v| v != &v5);
    let v3 = left
        .iter()
        .find(|&v| nb_letter_common(v, &v1) == 2)
        .unwrap()
        .to_string();
    left.retain(|v| v != &v3);
    let v2 = left.first().unwrap().to_string();

    let mut tot = 0;
    for r in right.split_whitespace() {
        let mut val = 0;
        if is(r, &v1) {
            val = 1;
        } else if is(r, &v2) {
            val = 2;
        } else if is(r, &v3) {
            val = 3;
        } else if is(r, &v4) {
            val = 4;
        } else if is(r, &v5) {
            val = 5;
        } else if is(r, &v6) {
            val = 6;
        } else if is(r, &v7) {
            val = 7;
        } else if is(r, &v8) {
            val = 8;
        } else if is(r, &v9) {
            val = 9;
        }
        tot = tot * 10 + val;
    }
    tot
}

fn sum_all(path: &str) -> u32 {
    let txt = fs::read_to_string(path).unwrap();
    let mut sum = 0;

    for l in txt.lines() {
        let tot = decode_line(l);
        sum += tot;
    }

    sum
}
