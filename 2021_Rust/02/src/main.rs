use std::fs;

/// TYPES /////////////////////////////////////////////////////////////////////

/// FUNCTIONS /////////////////////////////////////////////////////////////////

/// MAIN and PARSE ////////////////////////////////////////////////////////////
fn main() {
    let file = "input.txt";

    let (f1, d1) = do_move(&file);
    let (f2, d2) = do_move_aimed(&file);

    println!("\nanswer1={}, answer2={}", f1 * d1, f2 * d2);
}

fn do_move(path: &str) -> (u32, u32) {
    let txt = fs::read_to_string(path).unwrap();

    let mut d = 0;
    let mut f = 0;

    for l in txt.lines() {
        let tokens : Vec<&str> = l.split(' ').collect();
        let val = tokens[1].parse::<u32>().unwrap();
        match tokens[0] {
            "forward" => {f += val; },
            "down" => {d += val;},
            "up" => {d -= val;}
            _ => {}
        }
    }
    (f, d)
}


fn do_move_aimed(path: &str) -> (u32, u32) {
    let txt = fs::read_to_string(path).unwrap();

    let mut d = 0;
    let mut f = 0;
    let mut aim = 0;

    for l in txt.lines() {
        let tokens : Vec<&str> = l.split(' ').collect();
        let val = tokens[1].parse::<u32>().unwrap();
        match tokens[0] {
            "forward" => {f += val; d += aim * val;},
            "down" => {aim += val;},
            "up" => {aim -= val;}
            _ => {}
        }
    }
    (f, d)
}
