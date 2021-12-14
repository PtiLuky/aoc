use std::cmp::{max, min};
use std::collections::HashSet;
use std::fs;

/// TYPES /////////////////////////////////////////////////////////////////////

/// FUNCTIONS /////////////////////////////////////////////////////////////////

/// MAIN and PARSE ////////////////////////////////////////////////////////////
fn main() {
    let file = "input.txt";
    let (lines, (w, h)) = read_input(&file);

    let double = count_doubles(&lines, w);

    println!("\nanswer1={}", double);
}

fn read_input(path: &str) -> (Vec<(i32, i32, i32, i32)>, (i32, i32)) {
    let txt = fs::read_to_string(path).unwrap();
    let mut lines = Vec::new();
    let mut w = 0;
    let mut h = 0;
    for l in txt.lines() {
        let mut pts = l.split(" -> ");
        let mut pt1 = pts.next().unwrap().split(",");
        let mut pt2 = pts.next().unwrap().split(",");

        let x1 = pt1.next().unwrap().parse::<i32>().unwrap();
        let y1 = pt1.next().unwrap().parse::<i32>().unwrap();
        let x2 = pt2.next().unwrap().parse::<i32>().unwrap();
        let y2 = pt2.next().unwrap().parse::<i32>().unwrap();

        w = max(w, max(x1, x2) + 1);
        h = max(h, max(y1, y2) + 1);

        lines.push((x1, y1, x2, y2));
    }
    (lines, (w, h))
}

fn count_doubles(coords: &Vec<(i32, i32, i32, i32)>, w: i32) -> usize {
    let mut occupied_cells = HashSet::new();
    let mut double_cells = HashSet::new();

    for (x1, y1, x2, y2) in coords.iter() {
        if x1 == x2 {
            for y in *min(y1, y2)..(max(y1, y2) + 1) {
                let i = x1 + w * y;
                if occupied_cells.contains(&i) {
                    double_cells.insert(i);
                }
                occupied_cells.insert(i);
            }
        } else if y1 == y2 {
            for x in *min(x1, x2)..(max(x1, x2) + 1) {
                let i = x + w * y1;
                if occupied_cells.contains(&i) {
                    double_cells.insert(i);
                }
                occupied_cells.insert(i);
            }
        } else {
            let coef_x = if x1 < x2 { 1 } else { -1 };
            let coef_y = if y1 < y2 { 1 } else { -1 };
            for i in 0..((x1 - x2).abs() + 1) {
                let x = x1 + coef_x * i;
                let y = y1 + coef_y * i;
                let i = x + w * y;
                if occupied_cells.contains(&i) {
                    double_cells.insert(i);
                }
                occupied_cells.insert(i);
            }
        }
    }

    double_cells.len()
}
