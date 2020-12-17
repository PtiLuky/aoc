use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;
use std::time::Instant;
use std::collections::HashMap;
use Instr::{Mask, Write};

/// TYPES /////////////////////////////////////////////////////////////////////
#[derive(Clone, Debug)]
enum Instr {
    Mask(String), // mask with 0s, Xs and 1s
    Write(u64, u64), // address, number
}

/// FUNCTIONS /////////////////////////////////////////////////////////////////
fn count_val_in_mem(mem: &HashMap<u64, u64>) -> u64 {
    let mut sum = 0;
    for (_, val) in mem.iter() {
        sum += val;
    }
    sum
}

fn execute_v1(instr: &Vec<Instr>) -> u64 {
    let mut memory = HashMap::<u64, u64>::new();
    let mut curr_mask_0 = u64::MAX;
    let mut curr_mask_1 = 0;

    for i in instr {
        match i {
            Mask(mask) => {
                let mask_0 = mask.replace("X", "1");
                let mask_0 = u64::from_str_radix(&mask_0[..], 2).unwrap();
                curr_mask_0 = mask_0; 

                let mask_1 = mask.replace("X", "0");
                let mask_1 = u64::from_str_radix(&mask_1[..], 2).unwrap();
                curr_mask_1 = mask_1; 
            }
            Write(addr, val) => {
                let val = val & curr_mask_0;
                let val = val | curr_mask_1;
                memory.insert(*addr, val);
            }
        }
    }

    count_val_in_mem(&memory)
}

fn float_adresses(out_addresses: &mut Vec<u64>, mask: &String, addr: u64) {
    match mask.find("X") {
        Some(pos) => { 
            let mask = mask.replacen("X", "-", 1);
            let shift = mask.chars().count() - 1 - pos;
            let addr_0 = addr & (!(1 << shift));
            let addr_1 = addr | (1 << shift);
            float_adresses(out_addresses, &mask, addr_0);
            float_adresses(out_addresses, &mask, addr_1);
        },
        None => { out_addresses.push(addr); }
    }
}

fn addresses(mask: &String, addr: u64) -> Vec<u64>{
    // Force 1s
    let mask_1 = mask.replace("X", "0");
    let mask_1 = u64::from_str_radix(&mask_1[..], 2).unwrap();
    let addr = addr | mask_1;

    // Generate floatings addresses
    let mut addresses = Vec::<u64>::new();
    float_adresses(&mut addresses, mask, addr);

    addresses
}

fn execute_v2(instr: &Vec<Instr>) -> u64 {
    let mut memory = HashMap::<u64, u64>::new();
    let mut curr_mask = String::new();

    for i in instr {
        match i {
            Mask(mask) => {
                curr_mask = mask.clone();
            }
            Write(addr, val) => {
                for add in addresses(&curr_mask, *addr) {
                    memory.insert(add, *val);
                }
            }
        }
    }

    count_val_in_mem(&memory)
}

/// MAIN and PARSE ////////////////////////////////////////////////////////////
fn main() {
    let iter_cnt = 10;
    let start = Instant::now();
    let mut answer1 = 0;
    let mut answer2 = 0;
    for _ in 0..iter_cnt {
///////////////////////////////////////
        //let file = "test.txt";
        let file = "input.txt";

        let instrs = read_input(&file);
        answer1 = execute_v1(&instrs);
        answer2 = execute_v2(&instrs);

///////////////////////////////////////
    }
    println!("\nanswer1={}, answer2={}", answer1, answer2);
    println!("[Time] Finished after {:?} (mean on {} iterations)\n", start.elapsed() / iter_cnt, iter_cnt);
}


fn read_input(path: &str) -> Vec<Instr> { 
    let mut instrs = Vec::<Instr>::new();
    let f = File::open(path).unwrap();
    let f = BufReader::new(f);

    for l in f.lines() {
        let l = l.unwrap();
        instrs.push(match &l[0..4] {
            "mask" => {
                let mut ite = l.split(" = ");
                ite.next();
                let mask = ite.next().unwrap();
                Mask(mask.to_string())
            },
            "mem[" => {
                let addr = l[4..].split("]").next().unwrap().parse::<u64>().unwrap();
                let mut ite = l.split(" = ");
                ite.next();
                let val = ite.next().unwrap().parse::<u64>().unwrap();
                Write(addr, val)
            },
            _ => { panic!("CANT READ!"); }
        });
    }
    instrs
}
