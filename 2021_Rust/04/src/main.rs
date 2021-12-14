use std::cmp::max;
use std::fs;

/// TYPES /////////////////////////////////////////////////////////////////////
const W: u32 = 5;

/// FUNCTIONS /////////////////////////////////////////////////////////////////

/// MAIN and PARSE ////////////////////////////////////////////////////////////
fn main() {
    let file = "input.txt";
    let (draws, boards) = read_input(&file);

    let best = play_best(&draws, &boards);
    let worst = play_worst(&draws, &boards);

    println!("\nanswer1={}, answer2={}", best, worst);
}

fn read_input(path: &str) -> (Vec<usize>, Vec<Vec<usize>>) {
    let txt = fs::read_to_string(path).unwrap();
    let mut ite = txt.lines();
    let draws = ite
        .next()
        .unwrap()
        .split(",")
        .map(|v| v.parse::<usize>().unwrap())
        .collect();
    let mut boards = Vec::new();

    while let Some(l) = ite.next() {
        if l.is_empty() {
            boards.push(Vec::new());
        } else {
            let mut board = l
                .split_whitespace()
                .map(|v| v.parse::<usize>().unwrap())
                .collect();
            boards.last_mut().unwrap().append(&mut board);
        }
    }
    (draws, boards)
}

fn count_sum(board: &Vec<usize>, order: &Vec<usize>, turn: usize) -> usize {
    let mut tot = 0;
    for i in 0..25 {
        if order[board[i]] > turn {
            tot += board[i];
        }
    }
    tot
}

fn play_best(draws: &Vec<usize>, boards: &Vec<Vec<usize>>) -> usize {
    let mut order = vec![99; 100];
    for i in 0..draws.len() {
        order[draws[i] as usize] = i;
    }

    let mut win_turn = 100;
    let mut win_sum = 0;
    // boards
    for b in boards.iter() {
        // lines
        for y in 0..5 {
            let mut latest = 0;
            for x in 0..5 {
                let val = b[x + y * 5];
                latest = max(latest, order[val]);
            }
            if latest < win_turn {
                win_turn = latest;
                win_sum = count_sum(&b, &order, latest);
            }
        }

        // colums
        for x in 0..5 {
            let mut latest = 0;
            for y in 0..5 {
                let val = b[x + y * 5];
                latest = max(latest, order[val]);
            }
            if latest < win_turn {
                win_turn = latest;
                win_sum = count_sum(&b, &order, latest);
            }
        }
    }

    win_sum * draws[win_turn]
}

fn play_worst(draws: &Vec<usize>, boards: &Vec<Vec<usize>>) -> usize {
    let mut order = vec![99; 100];
    for i in 0..draws.len() {
        order[draws[i] as usize] = i;
    }

    let mut worst_win_turn = 0;
    let mut worst_win_sum = 0;
    // boards
    for b in boards.iter() {
        let mut win_turn = 100;
        let mut win_sum = 0;
        // lines
        for y in 0..5 {
            let mut latest = 0;
            for x in 0..5 {
                let val = b[x + y * 5];
                latest = max(latest, order[val]);
            }
            if latest < win_turn {
                win_turn = latest;
                win_sum = count_sum(&b, &order, latest);
            }
        }

        // colums
        for x in 0..5 {
            let mut latest = 0;
            for y in 0..5 {
                let val = b[x + y * 5];
                latest = max(latest, order[val]);
            }
            if latest < win_turn {
                win_turn = latest;
                win_sum = count_sum(&b, &order, latest);
            }
        }

        if win_turn > worst_win_turn {
            worst_win_sum = win_sum;
            worst_win_turn = win_turn;
        }
    }

    worst_win_sum * draws[worst_win_turn]
}
