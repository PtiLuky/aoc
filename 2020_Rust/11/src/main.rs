use std::fs;
use std::time::Instant;

// C LE GEU 2 LA VIE

/// TYPES /////////////////////////////////////////////////////////////////////


/// FUNCTIONS /////////////////////////////////////////////////////////////////
fn count_occupied_neig1(layout: &Vec<u8>, i: usize, w: usize) -> usize {
    let h = layout.len() / w;
    let mut count = 0;
    let y = i / w;
    let x = i % w;
    if y >= 1 && x >= 1   && layout[w * (y-1) + (x-1)] == 2 { count += 1; }
    if y >= 1             && layout[w * (y-1) + (x  )] == 2 { count += 1; }
    if y >= 1 && x < w-1  && layout[w * (y-1) + (x+1)] == 2 { count += 1; }
    if           x >= 1   && layout[w * (y  ) + (x-1)] == 2 { count += 1; }
    if           x < w-1  && layout[w * (y  ) + (x+1)] == 2 { count += 1; }
    if y < h-1 && x >= 1  && layout[w * (y+1) + (x-1)] == 2 { count += 1; }
    if y < h-1            && layout[w * (y+1) + (x  )] == 2 { count += 1; }
    if y < h-1 && x < w-1 && layout[w * (y+1) + (x+1)] == 2 { count += 1; }
    count
}
fn count_occupied_neig2(layout: &Vec<u8>, i: usize, w: usize) -> usize {
    let mut count = 0;
    if is_someone_in_direction(layout, -1, -1, i, w) { count += 1; }
    if is_someone_in_direction(layout, -1,  0, i, w) { count += 1; }
    if is_someone_in_direction(layout, -1,  1, i, w) { count += 1; }
    if is_someone_in_direction(layout,  0, -1, i, w) { count += 1; }
    if is_someone_in_direction(layout,  0,  1, i, w) { count += 1; }
    if is_someone_in_direction(layout,  1, -1, i, w) { count += 1; }
    if is_someone_in_direction(layout,  1,  0, i, w) { count += 1; }
    if is_someone_in_direction(layout,  1,  1, i, w) { count += 1; }
    count
}
fn is_someone_in_direction(layout: &Vec<u8>, dx: i32, dy: i32, i: usize, w: usize) -> bool {
    let h = (layout.len() / w) as i32;
    let w = w as i32;
    let mut y = i as i32 / w;
    let mut x = i as i32 % w;
    x += dx;
    y += dy;
    while x >= 0 && y >= 0 && x < w && y < h {
        if layout[(y * w + x) as usize] == 1 {
            return false
        } else if layout[(y * w + x) as usize] == 2 {
            return true
        }
        x += dx;
        y += dy;
    }

    false
}

fn populate1(layout: &mut Vec<u8>, w: usize) -> usize {
    let nb_occupy_to_occupy = 0; // 0  neighbor  => occupy seat
    let nb_occupy_to_empty  = 4; // 4+ neighbors => leave empty
    let current_layout = layout.clone();
    let mut changes = 0;
    for i in 0..layout.len() {
        let neigs_occupied = count_occupied_neig1(&current_layout, i, w);
        if current_layout[i] == 1 && neigs_occupied <= nb_occupy_to_occupy {
            layout[i] = 2;
            changes += 1;
        } 
        if current_layout[i] == 2 && neigs_occupied >= nb_occupy_to_empty {
            layout[i] = 1;
            changes += 1;
        } 
    }
    changes
}
fn populate2(layout: &mut Vec<u8>, w: usize) -> usize {
    let nb_occupy_to_occupy = 0; // 0  neighbor  => occupy seat
    let nb_occupy_to_empty  = 5; // 5+ neighbors => leave empty
    let current_layout = layout.clone();
    let mut changes = 0;
    for i in 0..layout.len() {
        let neigs_occupied = count_occupied_neig2(&current_layout, i, w);
        if current_layout[i] == 1 && neigs_occupied <= nb_occupy_to_occupy {
            layout[i] = 2;
            changes += 1;
        } 
        if current_layout[i] == 2 && neigs_occupied >= nb_occupy_to_empty {
            layout[i] = 1;
            changes += 1;
        } 
    }
    changes
}

#[allow(dead_code)]
fn print_layout(layout: &Vec<u8>, w: usize) {
    for y in 0..layout.len() / w {
        for x in 0..w {
            match layout[x + y * w] {
                0 => print!("."),
                1 => print!("L"),
                2 => print!("#"),
                _ => print!("?"),
            }
        }
        println!();
    }
    println!();
}

fn count_occupied1(layout: &Vec<u8>, w: usize) -> usize {
    let mut layout = layout.clone();
    while populate1(&mut layout, w) != 0 {
        //print_layout(&layout, w);
    }
    layout.iter().filter(|&s| *s == 2).count()
}
fn count_occupied2(layout: &Vec<u8>, w: usize) -> usize {
    let mut layout = layout.clone();
    while populate2(&mut layout, w) != 0 {
        //print_layout(&layout, w);
    }
    layout.iter().filter(|&s| *s == 2).count()
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
        
        let (layout, width) = read_input(&file);

        answer1 = count_occupied1(&layout, width);
        answer2 = count_occupied2(&layout, width);
///////////////////////////////////////
    }
    println!("\nOccupied1={}, occupied2={}", answer1, answer2);
    println!("[Time] Finished after {:?} (mean on {} iterations)\n", start.elapsed() / iter_cnt, iter_cnt);
}


fn read_input(path: &str) -> (Vec<u8>, usize) { 
    let input = fs::read_to_string(path).unwrap();
    let w = input.find('\n').unwrap() - 1;

    let mut nums = Vec::<u8>::with_capacity(input.chars().count());
    for c in input.chars() {
        match c {
            '.' => nums.push(0),
            'L' => nums.push(1),
            '#' => nums.push(2),
            _ => {}
        }
    }
    (nums, w)
}
