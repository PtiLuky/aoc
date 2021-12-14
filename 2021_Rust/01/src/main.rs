use std::fs;

/// TYPES /////////////////////////////////////////////////////////////////////

/// FUNCTIONS /////////////////////////////////////////////////////////////////

/// MAIN and PARSE ////////////////////////////////////////////////////////////
fn main() {
    let file = "input.txt";

    let answer1 = count_increase(&file);
    let answer2 = count_increase_slide(&file);

    println!("\nanswer1={}, answer2={}", answer1, answer2);
}

fn count_increase(path: &str) -> u64 {
    let txt = fs::read_to_string(path).unwrap();

    let mut prev = -1;
    let mut tot = 0;

    for l in txt.lines() {
        let num = l.parse::<i64>().unwrap();
        if prev != -1 && prev < num {
            tot += 1;
        }
        prev = num;
    }
    tot
}

fn count_increase_slide(path: &str) -> u64 {
    let txt = fs::read_to_string(path).unwrap();

    let mut prev1 = -1;
    let mut prev2 = -1;
    let mut prev_win = -1;
    let mut tot = 0;

    for l in txt.lines() {
        let num = l.parse::<i64>().unwrap();
        if prev1 != -1 && prev2 != -1 {
            let sum = num + prev1 + prev2;
            if prev_win != -1 && sum > prev_win {
                tot += 1;
            }
            prev_win = sum;
        }
        prev2 = prev1;
        prev1 = num;
    }
    tot
}
