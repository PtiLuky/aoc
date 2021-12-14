use std::collections::{HashMap, HashSet};
use std::fs;

/// TYPES /////////////////////////////////////////////////////////////////////

/// FUNCTIONS /////////////////////////////////////////////////////////////////

/// MAIN and PARSE ////////////////////////////////////////////////////////////
fn main() {
    let file = "input.txt";
    let (pts, orders) = read_input(&file);

    let first_order = vec![orders[0]];
    let result = fold(&pts, &first_order);
    let c1 = result.len();

    let result = fold(&pts, &orders);
    print(&result, 50, 10);
    let c2 = 0;

    println!("\nanswer1={}, answer2={}", c1, c2);
}

fn print(pts: &HashSet<(u32, u32)>, w: usize, h: usize) {
    let mut arr = vec![' '; w * h];
    for (x, y) in pts {
        let (x, y) = (*x as usize, *y as usize);
        arr[y * w + x] = '#';
    }
    for y in 0..h {
        for x in 0..w {
            let (x, y) = (x as usize, y as usize);
            print!("{}", arr[y * w + x]);
        }
        println!("");
    }
}

fn read_input(path: &str) -> (HashSet<(u32, u32)>, Vec<(bool, u32)>) {
    let txt = fs::read_to_string(path).unwrap();

    let mut pts = HashSet::new();
    let mut orders = Vec::new();

    let mut first_part = true;

    for l in txt.lines() {
        if l.is_empty() {
            first_part = false;
        } else if first_part {
            let mut s = l.split(",");
            let x = s.next().unwrap().parse::<u32>().unwrap();
            let y = s.next().unwrap().parse::<u32>().unwrap();
            pts.insert((x, y));
        } else {
            let sentence_x = "fold along x=";
            let idx = l[sentence_x.len()..l.len()].parse::<u32>().unwrap();
            orders.push((l.starts_with(&sentence_x), idx));
        }
    }

    (pts, orders)
}

fn fold(points: &HashSet<(u32, u32)>, folds: &Vec<(bool, u32)>) -> HashSet<(u32, u32)> {
    let mut pts = points.clone();
    for &(is_x, coord) in folds {
        let mut new_pts = HashSet::new();
        for (x, y) in pts {
            let x = if is_x && x > coord { 2 * coord - x } else { x };
            let y = if is_x || y <= coord { y } else { 2 * coord - y };
            new_pts.insert((x, y));
        }
        pts = new_pts;
    }
    pts
}
