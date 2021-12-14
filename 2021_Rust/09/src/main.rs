use std::fs;

/// TYPES /////////////////////////////////////////////////////////////////////

/// FUNCTIONS /////////////////////////////////////////////////////////////////

/// MAIN and PARSE ////////////////////////////////////////////////////////////
fn main() {
    let file = "input.txt";
    let (vals, w) = read_input(&file);

    let c1 = min_score(&vals, w);
    let c2 = 0;//best_pos2(&vals);

    println!("\nanswer1={}, answer2={}", c1, c2);
}

fn read_input(path: &str) -> (Vec<u8>, usize) {
    let txt = fs::read_to_string(path).unwrap();
    let w = txt.lines().next().unwrap().len();
    let vals = txt
        .chars()
        .filter(|c| c.is_numeric())
        .map(|c| c as u8 - '0' as u8)
        .collect();
    (vals, w)
}

fn min_score(vals: &Vec<u8>, w: usize) -> u32 {
    let h = vals.len() / w;
    let mut score = 0;
    for y in 0..h {
        for x in 0..w {
            if (y == 0 || vals[(y - 1) * w + x] > vals[y * w + x])
                && (y == h - 1 || vals[(y + 1) * w + x] > vals[y * w + x])
                && (x == 0 || vals[y * w + (x - 1)] > vals[y * w + x])
                && (x == w - 1 || vals[y * w + (x + 1)] > vals[y * w + x])
            {
                score += vals[y * w + x] as u32 + 1
            }
        }
    }
    score
}
