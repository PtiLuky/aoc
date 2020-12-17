use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;
use std::time::Instant;
use std::collections::HashMap;

/// TYPES /////////////////////////////////////////////////////////////////////
type Rules = HashMap<String, Vec<(u32, u32)>>;

struct Ticket {
    fields: Vec<u32>,
    valid: bool
}

/// FUNCTIONS /////////////////////////////////////////////////////////////////
fn is_field_valid(rules: &Rules, field: u32) -> bool {
    for (_, r) in rules.iter() {
        for range in r {
            if field >= range.0 && field <= range.1 { 
                return true
            }
        }
    }
    false
}

fn check_error_rate(rules: &Rules, tickets: &mut Vec<Ticket>) -> u32 {
    let mut err_rate = 0;
    for t in tickets {
        for f in &t.fields {
            if !is_field_valid(rules, *f){
                err_rate += f;
                t.valid = false;
            }
        }
    }
    err_rate
}

fn solution_unique(possibles: &Vec<Vec<String>>) -> bool {
    for p in possibles {
        if p.len() > 1 {
            return false
        }
    }
    true
}

fn is_rule_ok(ranges: &Vec<(u32, u32)>, val: u32) -> bool {
    for r in ranges {
        if val >= r.0 && val <= r.1 {
            return true
        }
    }
    false
}

fn find_unique_solution(mut possibles: Vec<Vec<String>>) -> Vec<String> {
    let nb_fields = possibles.len();
    let no_str = "".to_string();
    let mut sol : Vec::<String> = vec![no_str.clone(); nb_fields];

    while sol.contains(&no_str) {
        for i in 0..nb_fields {
            if sol[i] != no_str { continue; }

            if possibles[i].len() == 1 {
                sol[i] = possibles[i][0].clone();
                for vec in &mut possibles {
                    vec.retain(|name| name != &sol[i]);
                }
            }
        }
    }

    sol
}

fn find_categories_order(rules: &Rules, tickets: &Vec<Ticket>) -> Vec<String> {
    let nb_fields = tickets[0].fields.len();
    let mut possibles : Vec<Vec<String>> = vec![rules.keys().cloned().collect(); nb_fields];

    for t in tickets {
        if solution_unique(&possibles) {
            break;
        }

        if !t.valid {
            continue;
        }
            
        for i_field in 0..possibles.len() {
            possibles[i_field].retain(|name| is_rule_ok(&rules[name], t.fields[i_field]));
        }
    }

    find_unique_solution(possibles)
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

        let (rules, my_ticket, mut other_tickets) = read_input(&file);
        answer1 = check_error_rate(&rules, &mut other_tickets);
        let labels = find_categories_order(&rules, &mut other_tickets);
        answer2 = 1;
        for i in 0..labels.len() {
            if labels[i].starts_with("departure") {
                answer2 *= (my_ticket.fields[i] as u64);
            }
        }

///////////////////////////////////////
    }
    println!("\nanswer1={}, answer2={}", answer1, answer2);
    println!("[Time] Finished after {:?} (mean on {} iterations)\n", start.elapsed() / iter_cnt, iter_cnt);
}


fn read_input(path: &str) -> (Rules, Ticket, Vec<Ticket>) { 
    let mut rules = Rules::new();
    let mut my_ticket = Ticket{fields: Vec::<u32>::new(), valid: true};
    let mut tickets = Vec::<Ticket>::new();
    let f = File::open(path).unwrap();
    let f = BufReader::new(f);

    let mut rules_done = false;
    let mut my_ticket_done = false;

    for l in f.lines() {
        let l = l.unwrap();
        if l.is_empty() {
            if rules_done {  my_ticket_done = true; } 
            else          {  rules_done = true; }
            continue;
        }

        if !rules_done {
            let mut ite = l.split(": ");
            let name = ite.next().unwrap();
            let mut ranges = Vec::<(u32, u32)>::new();
            for range in ite.next().unwrap().split(" or ") {
                let mut ite = range.split("-");
                let i1 = ite.next().unwrap().parse::<u32>().unwrap();
                let i2 = ite.next().unwrap().parse::<u32>().unwrap();
                ranges.push((i1, i2));
            }
            rules.insert(name.to_string(), ranges);

        } else if !my_ticket_done {
            if l.contains(":") { continue; }

            for i in l.split(",") { my_ticket.fields.push(i.parse().unwrap()); }
        } else {
            if l.contains(":") { continue; }

            let mut fields = Vec::<u32>::new();
            for i in l.split(",") { fields.push(i.parse().unwrap()); }
            tickets.push(Ticket{fields, valid: true});
        }
    }
    (rules, my_ticket, tickets)
}
