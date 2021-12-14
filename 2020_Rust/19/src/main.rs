use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;
use std::time::Instant;
use std::collections::HashMap;

// Note : an "easy" solution would to build a corresponding regex : 
/*
0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: "a"
5: "b"

would become "a((aa|bb)|(ab|ba))b"

But here I want to do otherwise. I may explore this solution in another main.rs
*/

/// TYPES /////////////////////////////////////////////////////////////////////
enum Elem {
    Id(usize),
    Char(String)
}
type Seq = Vec<Elem>; // Sequence
type Rule = Vec<Seq>;
type Rules = HashMap<usize, Rule>;

/// FUNCTIONS /////////////////////////////////////////////////////////////////
// if match returns true and the number of characters consummed
fn can_match_total(msg: &str, rules: &Rules) -> bool {
    // Matches rule 0 and uses all characters
    for size in can_match_rule(msg, 0, rules) {
        if size == msg.len() {
            return true;
        }
    }
    false
}

// Returns a vector of sizes of the substrings msg[0..size] validating the rule
fn can_match_rule(msg: &str, rule_id: usize, rules: &Rules) -> Vec<usize> {
    let mut ok_sizes = Vec::<usize>::new();
    for s in &rules[&rule_id] {
        let sequence_result = can_match_seq(msg, s, rules);
        ok_sizes.extend(sequence_result.iter());
    }
    ok_sizes
}

// Returns a vector of sizes of the substrings msg[0..size] validating the sequence
fn can_match_seq(msg: &str, seq: &Seq, rules: &Rules) -> Vec<usize> {
    // Start with one offset : 0
    let mut ok_offsets : Vec<usize> = vec![0];
    let mut next_offset = Vec::<usize>::new();
    for e in seq {
        // Remove offsets going too far
        ok_offsets.retain(|&offset| offset < msg.len());
        if ok_offsets.is_empty() {
            return ok_offsets;
        }

        // Process next element for each possible offset
        next_offset.clear();
        for o in &ok_offsets {
            let o = *o;
            match e {
                Elem::Id(id) => {
                    let deltas = can_match_rule(&msg[o..], *id, rules);
                    for d in deltas {
                        next_offset.push(o + d);
                    }
                },
                Elem::Char(txt) => {
                    if txt[0..1] == msg[o..(o+1)] {
                        next_offset.push(o + 1);
                    }
                },
            }
        }
        std::mem::swap(&mut ok_offsets, &mut next_offset); // avoid realloc by swaping old and new
    }
    ok_offsets
}


/// MAIN and PARSE ////////////////////////////////////////////////////////////
fn main() {
    let iter_cnt = 10;
    let start = Instant::now();
    let mut answer1 = 0;
    for _ in 0..iter_cnt {
///////////////////////////////////////
        //let file = "test1.txt";
        //let file = "test2.txt";
        //let file = "input1.txt";
        let file = "input2.txt";

        let (rules, messages) = read_input(&file);
        let mut count = 0;
        for msg in messages {
            if can_match_total(&msg[..], &rules) {
                count += 1;
            }
        }
        answer1 = count;

///////////////////////////////////////
    }
    println!("\nanswer1={}", answer1);
    println!("[Time] Finished after {:?} (mean on {} iterations)\n", start.elapsed() / iter_cnt, iter_cnt);
}


fn read_input(path: &str) -> (Rules, Vec<String>) { 
    let mut rules = Rules::new();
    let mut messages = Vec::<String>::new();

    let f = File::open(path).unwrap();
    let f = BufReader::new(f);

    let mut rules_done = false;

    for l in f.lines() {
        let l = l.unwrap();
        if l.is_empty() {
            rules_done = true;
            continue;
        }

        if !rules_done {
            let mut ite = l.split(": ");
            let id = ite.next().unwrap().parse::<usize>().unwrap();
            let definition = ite.next().unwrap();
            let mut subrules = Vec::<Seq>::new();
            
            for rule in definition.split(" | ") {
                let mut seq = Seq::new();

                for elem in rule.split(" ") {
                    if &elem[0..1] == "\"" { 
                        seq.push(Elem::Char(elem[1..2].to_string()));
                    } else {
                        seq.push(Elem::Id(elem.parse().unwrap()));
                    }
                }
                subrules.push(seq);
            }
            rules.insert(id, subrules);
            
        } else {
            messages.push(l.to_string());
        }
    }
    (rules, messages)
}
