use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};
use std::fs;

/// TYPES /////////////////////////////////////////////////////////////////////
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Node {
    cost: u32,
    idx: usize,
}
impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.idx.cmp(&other.idx))
    }
}
impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// FUNCTIONS /////////////////////////////////////////////////////////////////

/// MAIN and PARSE ////////////////////////////////////////////////////////////
fn main() {
    let file = "input.txt";
    let (input, (w, h)) = read_input(&file);
    let (input2, (w2, h2)) = multiply_map(&input, w, h);

    let c1 = a_star(&input, w, h);
    let c2 = a_star(&input2, w2, h2);

    println!("\nanswer1={}, answer2={}", c1, c2);
}

fn a_star(board: &Vec<u8>, w: usize, h: usize) -> usize {
    let target = w * h - 1;

    let mut gcosts: Vec<u32> = vec![99999999; w * h];
    let mut prev: Vec<i64> = vec![-1; w * h];

    let mut queue = BinaryHeap::new();
    let mut seen = HashSet::new();

    gcosts[0] = 0;

    queue.push(Node {
        cost: h_cost(0, w, h),
        idx: 0,
    });

    while let Some(Node {cost, idx }) = queue.pop() {
        if idx == target {
            break;
        }
        seen.remove(&idx);

        for &n in get_neight(idx, w, h).iter() {
            let n = if n < 0 {
                continue;
            } else {
                n as usize
            };

            let alt_dist = gcosts[idx] + board[n] as u32;
            if alt_dist < gcosts[n] {
                gcosts[n] = alt_dist;
                prev[n] = idx as i64;
                if !seen.contains(&n) {
                    queue.push(Node {
                        cost: alt_dist + h_cost(n, w, h),
                        idx: n,
                    });
                    seen.insert(n);
                }
            }
        }
    }

    let mut tot = 0;
    let mut curr = target as i64;
    while curr > 0 {
        // we want to skip the first.
        tot += board[curr as usize] as usize;
        curr = prev[curr as usize];
    }
    tot
}

fn h_cost(idx: usize, w: usize, h: usize) -> u32 {
    // Minimize real possible score => go for exact solution but may be slower => using 0 is using simple dijkstra
    // Maximize real possible score => faster, but non exact solution
    let x = idx % w;
    let y = idx / w;
    ((w - 1 - x) * 1 + (h - 1 - y) * 1) as u32
}

fn get_neight(idx: usize, w: usize, h: usize) -> [i64; 4] {
    let x = idx % w;
    let y = idx / w;
    let l = if x == 0 { -1 } else { ((x - 1) + y * w) as i64 };
    let t = if y == 0 { -1 } else { (x + (y - 1) * w) as i64 };
    let r = if x == w - 1 {
        -1
    } else {
        ((x + 1) + y * w) as i64
    };
    let b = if y == h - 1 {
        -1
    } else {
        (x + (y + 1) * w) as i64
    };
    [l, t, r, b]
}

fn read_input(path: &str) -> (Vec<u8>, (usize, usize)) {
    let txt = fs::read_to_string(path).unwrap();

    let w = txt.lines().next().unwrap().len();
    let data: Vec<u8> = txt
        .chars()
        .filter(|c| c.is_numeric())
        .map(|c| c as u8 - '0' as u8)
        .collect();
    assert!(data.len() % w == 0, "size not divisable");
    let h = data.len() / w;

    (data, (w, h))
}

fn multiply_map(board: &Vec<u8>, w: usize, h: usize) -> (Vec<u8>, (usize, usize)) {
    let new_h = 5 * h;
    let new_w = 5 * w;
    let mut new_board = vec![0; new_w * new_h];

    for r_x in 0..5 {
        for r_y in 0..5 {
            for x in 0..w {
                for y in 0..h {
                    new_board[(x + w * r_x) + (y + h * r_y) * new_w] =
                        (board[x + y * w] + r_x as u8 + r_y as u8 - 1) % 9 + 1;
                }
            }
        }
    }

    (new_board, (new_w, new_h))
}