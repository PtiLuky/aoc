use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;
use std::time::Instant;
use Instr::{North, South, East, West, Left, Right, Forward};

/// TYPES /////////////////////////////////////////////////////////////////////
#[derive(Clone)]
enum Instr {
    North(i32),
    South(i32),
    East(i32),
    West(i32),

    Left(i32),
    Right(i32),
    Forward(i32),
}

/// FUNCTIONS /////////////////////////////////////////////////////////////////
fn turn(face: &Instr, mut angle: i32) -> Instr{
    let mut face = face.clone();
    if angle < 0 {
        angle += 360;
    }
    while angle > 0 {
        match face {
            North(_) => { 
                face = East(0);  
            },
            South(_) => { 
                face = West(0); 
            },
            East(_)  => { 
                face = South(0); 
            },
            West(_)  => { 
                face = North(0);
            },
            _ => {}
        };
        angle -= 90;
    }
    face
}

fn travel1(directions: &Vec<Instr>) -> (i32, i32) {
    let mut x = 0;
    let mut y = 0;
    let mut face = Instr::East(0);

    for dir in directions {
        match dir {
            North(d) => { y += d; },
            South(d) => { y -= d; },
            East(d)  => { x += d; },
            West(d)  => { x -= d; },
            Left(d)  => { face = turn(&mut face, -*d); },
            Right(d) => { face = turn(&mut face, *d); },
            Forward(d) => {
                match face {
                    North(_) => y += d,
                    South(_) => y -= d,
                    East(_)  => x += d,
                    West(_)  => x -= d,
                    _ => {}
                };
            }
        };
    }
    (x, y)
}

fn turn_wp(mut wp: (i32, i32), mut angle: i32) -> (i32, i32) {
    if angle < 0 {
        angle += 360;
    }

    while angle > 0 {
        wp = (wp.1, -wp.0);
        angle -= 90;
    }
    wp
}

fn travel2(directions: &Vec<Instr>) -> (i32, i32) {
    let mut x = 0;
    let mut y = 0;

    let mut wp = (10, 1);

    for dir in directions {
        match dir {
            North(d) => { wp.1 += d; },
            South(d) => { wp.1 -= d; },
            East(d)  => { wp.0 += d; },
            West(d)  => { wp.0 -= d; },
            Left(d)  => { wp = turn_wp(wp, -*d); },
            Right(d) => { wp = turn_wp(wp, *d); },
            Forward(d) => {
                x += d * wp.0;
                y += d * wp.1;
            }
        };
    }
    (x, y)
}

/// MAIN and PARSE ////////////////////////////////////////////////////////////
fn main() {
    let iter_cnt = 100;
    let start = Instant::now();
    let mut answer1 = 0;
    let mut answer2 = 0;
    for _ in 0..iter_cnt {
///////////////////////////////////////
        //let file = "test.txt";
        let file = "input.txt";

        let directions = read_input(&file);

        let (x, y) = travel1(&directions);
        answer1 = x.abs() + y.abs();

        let (x, y) = travel2(&directions);
        answer2 = x.abs() + y.abs();

///////////////////////////////////////
    }
    println!("\nanswer1={}, answer2={}", answer1, answer2);
    println!("[Time] Finished after {:?} (mean on {} iterations)\n", start.elapsed() / iter_cnt, iter_cnt);
}


fn read_input(path: &str) -> Vec<Instr> { 
    let mut instr = Vec::<Instr>::new();
    let f = File::open(path).unwrap();
    let f = BufReader::new(f);
    for l in f.lines() {
        let l = l.unwrap();
        let num = l[1..].parse::<i32>().unwrap();
        instr.push(match &l[0..1] {
            "N" => Instr::North(num),
            "S" => Instr::South(num),
            "E" => Instr::East(num),
            "W" => Instr::West(num),
            "L" => Instr::Left(num),
            "R" => Instr::Right(num),
            "F" => Instr::Forward(num),
            _ => Instr::North(0),
        });
    }
    instr
}
