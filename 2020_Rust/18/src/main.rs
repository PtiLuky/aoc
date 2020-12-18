use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;
use std::time::Instant;

/*
// Sad little unused Expression tree :'(
use Expression::{Sum, Product, Parenth, Num};

/// TYPES /////////////////////////////////////////////////////////////////////

type RefExpr = Box<Expression>;
enum Expression {
    Sum(RefExpr, RefExpr),
    Product(RefExpr, RefExpr),
    Parenth(RefExpr),
    Num(u64),
    None
}*/

/// FUNCTIONS /////////////////////////////////////////////////////////////////
fn compute_tokens1(tokens: &Vec<&str>, mut idx: &mut usize) -> u64 {
    let mut res = 0;
    let mut op = '=';   
    
    while idx < &mut tokens.len() {
        match &tokens[*idx][..] {
            ")" => { break; },
            "*" => { op = '*'; }, // since it has the lowest priority remember the operator and apply later
            "+" => { op = '+'; }, // since it has the lowest priority remember the operator and apply later
            expr => {
                let num;
                if expr == "(" {  // since it has the highest priority compute now
                    *idx = *idx + 1;
                    num = compute_tokens1(&tokens, &mut idx);
                } else {
                    num = expr.parse::<u64>().unwrap();
                }
                match op {
                    '=' => { res = num; },
                    '+' => { res += num },
                    '*' => { res *= num },
                    _ => { panic!("Unknown operator"); }
                }
            }
        }
        *idx = *idx + 1;
    }
    res
}

fn compute_line1(line: &str) -> u64 {
    let split: Vec<&str> = line.split(' ').collect();
    let mut idx = 0;
    let res = compute_tokens1(&split, &mut idx);
    res
}

fn compute_tokens2(tokens: &Vec<&str>, mut idx: &mut usize) -> u64 {
    let mut res = 0;
    let mut op = '=';   
    
    while idx < &mut tokens.len() {
        match &tokens[*idx][..] {
            ")" => { break; },
            "*" => {  // since it has the lowest priority just compute all the rest and apply at the complete end
                *idx = *idx + 1;
                let num = compute_tokens2(&tokens, &mut idx);
                res *= num;
                break; },
            "+" => { op = '+'; },  // since it has a low priority remember the operator and apply later
            expr => {
                let num;
                if expr == "(" {  // since it has the highest priority compute now
                    *idx = *idx + 1;
                    num = compute_tokens2(&tokens, &mut idx);
                } else {
                    num = expr.parse::<u64>().unwrap();
                }
                match op {
                    '=' => { res = num; },
                    '+' => { res += num },
                    '*' => { res *= num },
                    _ => { panic!("Unknown operator"); }
                }
            }
        }
        *idx = *idx + 1;
    }
    res
}

fn compute_line2(line: &str) -> u64 {
    let split: Vec<&str> = line.split(' ').collect();
    let mut idx = 0;
    let res = compute_tokens2(&split, &mut idx);
    res
}

/// MAIN and PARSE ////////////////////////////////////////////////////////////
fn main() {
    let iter_cnt = 1;
    let start = Instant::now();
    let mut answer1 = 0;
    let mut answer2 : u64 = 0;
    for _ in 0..iter_cnt {
///////////////////////////////////////
        //let file = "test.txt";
        let file = "input.txt";

        let totals = read_input(&file);
        answer1 = totals.0;
        answer2 = totals.1;

///////////////////////////////////////
    }
    println!("\nanswer1={}, answer2={}", answer1, answer2);
    println!("[Time] Finished after {:?} (mean on {} iterations)\n", start.elapsed() / iter_cnt, iter_cnt);
}


fn read_input(path: &str) -> (u64, u64) { 
    let f = File::open(path).unwrap();
    let f = BufReader::new(f);

    let mut tot1 = 0;
    let mut tot2 = 0;
    for l in f.lines() {
        let l = l.unwrap();
        let l = l.replace("(", "( ");
        let l = l.replace(")", " )");
        tot1 += compute_line1(&l);
        tot2 += compute_line2(&l);
    }

    (tot1, tot2)
}
