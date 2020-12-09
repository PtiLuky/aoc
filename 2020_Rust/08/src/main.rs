use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;
use std::time::Instant;

/// TYPES /////////////////////////////////////////////////////////////////////
#[derive(Debug)]
enum Instr {
    Nope(i32), // No Op
    Acc(i32),  // Increase/decrease global var Accumulator 
    Jump(i32), // Relative jump
}

type Program = Vec<Instr>;

struct Cpu {
    prog : Program,  // List of instructions
    cir : usize,     // Current instruction (index)
    accumulator: i32 // Accumulator global variable
}

/// FUNCTIONS /////////////////////////////////////////////////////////////////
fn exec_instr(cpu: &mut Cpu) -> usize {
    match cpu.prog[cpu.cir] {
        Instr::Nope(_) => { cpu.cir += 1; },
        Instr::Acc(v)  => { cpu.accumulator += v; cpu.cir += 1; },
        Instr::Jump(v) => { cpu.cir = (cpu.cir as i32 + v) as usize; }
    }
    cpu.cir
}

fn run_stop_on_repeat(cpu: &mut Cpu) -> (i32, bool) {
    let mut instr_exec : Vec<bool> = vec![false; cpu.prog.len()];
    let mut prev_acc = 0;
    let mut finished = false;
    while !finished && !instr_exec[cpu.cir] {
        prev_acc = cpu.accumulator;
        instr_exec[cpu.cir]  = true;
        exec_instr(cpu);
        finished = cpu.cir >= cpu.prog.len();
    }
    (prev_acc, finished)
}

fn permut_jump_nop(prog: &mut Program, i: usize) {
    match prog[i] {
        Instr::Nope(v) => {prog[i] = Instr::Jump(v);},
        Instr::Jump(v) => {prog[i] = Instr::Nope(v);},
        _ => {}
    }
}

fn find_corruption(cpu: &mut Cpu) -> i32 {
    for i in 0..(cpu.prog.len()) {
        cpu.accumulator = 0;
        cpu.cir = 0;
        permut_jump_nop(&mut cpu.prog, i);
        let (_, finished) = run_stop_on_repeat(cpu);
        permut_jump_nop(&mut cpu.prog, i);
        if finished {
            break;
        }
    }
    cpu.accumulator
}

/// MAIN and PARSE ////////////////////////////////////////////////////////////
fn main() {
    let iter_cnt = 1000;
    let start = Instant::now();
    let mut acc_before_loop = 0;
    let mut acc_end         = 0;
    for _ in 0..iter_cnt {
///////////////////////////////////////
        //let file = "test.txt";
        let file = "input.txt";
        
        let prog        = read_input(&file);
        let mut process = Cpu {prog, cir : 0, accumulator : 0};

        acc_before_loop = run_stop_on_repeat(&mut process).0;
        acc_end = find_corruption(&mut process);

///////////////////////////////////////
    }
    let duration = start.elapsed();
    
    println!("\nLoop after acc={}, End after acc={}", acc_before_loop, acc_end);
    println!("[Time] Finished after {:?} (mean on {} iterations)\n", duration / iter_cnt, iter_cnt);
}

fn read_input(path: &str) -> Program {
    let mut prog = Program::new();
    
    let f = File::open(path).unwrap();
    let f = BufReader::new(f);
    for line in f.lines() {
        let line = line.unwrap();
        let mut ite_line = line.split(" ");
        let instr_type   = ite_line.next().unwrap();
        let val1         = ite_line.next().unwrap().parse::<i32>().unwrap();
        let instr = match instr_type {
            "jmp" => Instr::Jump(val1),
            "acc" => Instr::Acc(val1),
            _ => Instr::Nope(val1)
        };
        prog.push(instr);
    }
    prog
}