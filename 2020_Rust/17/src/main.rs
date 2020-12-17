use std::fs;
use std::time::Instant;
use Cell::{Inactive, Active};

// C LE JEU 2 LA VI ENKOR

/// TYPES /////////////////////////////////////////////////////////////////////
#[derive(Clone, Debug, PartialEq)]
enum Cell { Inactive, Active }

#[derive(Clone, Debug)]
struct Planete {
    cells: Vec<Cell>,
    dim: (usize, usize, usize, usize),
}

/// FUNCTIONS /////////////////////////////////////////////////////////////////
fn xyz_to_idx(coords: (usize, usize, usize, usize), dim: (usize, usize, usize, usize)) -> usize {
    return coords.0 + coords.1 * dim.0 + coords.2 * dim.0 * dim.1 + coords.3 * dim.0 * dim.1 * dim.2;
}

#[allow(dead_code)]
fn print_cells(planete: &Planete, z: usize, w: usize) {
    for y in 0..planete.dim.1 {
        for x in 0..planete.dim.0 {
            print!("{}", match planete.cells[xyz_to_idx((x,y,z,w), planete.dim)] { Active => '#', Inactive => '.'});
        }
        println!();
    }
    println!("{:?}", planete.dim);
}

fn count_neighbors(world: &Planete, coord: (usize, usize, usize, usize)) -> usize {
    let mut count = 0;
    for dx in -1..=1 {
        let x = (coord.0 as i64) + dx;
        if x < 0 || x >= world.dim.0 as i64 { 
            continue; 
        }
        let x = x as usize;

        for dy in -1..=1 {
            let y = (coord.1 as i64) + dy;
            if y < 0 || y >= world.dim.1 as i64 { 
                continue; 
            }
            let y = y as usize;

            for dz in -1..=1 {
                let z = (coord.2 as i64) + dz;
                if z < 0 || z >= world.dim.2 as i64 { 
                    continue; 
                }
                let z = z as usize;

                for dw in -1..=1 {
                    let w = (coord.3 as i64) + dw;
                    if w < 0 || w >= world.dim.3 as i64 { 
                        continue; 
                    }
                    let w = w as usize;


                    if x == coord.0 && y == coord.1 && z == coord.2 && w == coord.3{
                        continue;
                    }
                    
                    // Actually count
                    match world.cells[xyz_to_idx((x, y, z, w), world.dim)] {
                        Active => { count += 1; },
                        Inactive => {}
                    }
                }
            }
        }
    }
    count
}

fn live(planete: &mut Planete) {
    let current = planete.clone();
    for x in 0..planete.dim.0 {
        for y in 0..planete.dim.1 {
            for z in 0..planete.dim.2 {
                for w in 0..planete.dim.3 {
                    let cnt = count_neighbors(&current, (x, y, z, w));
                    let idx = xyz_to_idx((x, y, z, w), planete.dim);
                    match planete.cells[idx] {
                        Active   => { if cnt != 2 && cnt != 3 { planete.cells[idx] = Inactive; } },
                        Inactive => { if cnt == 3 { planete.cells[idx] = Active; } }
                    }
                }
            }
        }
    }
}

/// MAIN and PARSE ////////////////////////////////////////////////////////////
fn main() {
    let iter_cnt = 10;
    let start = Instant::now();
    let mut answer1 = 0;
    let mut answer2 : u64 = 0;
    for _ in 0..iter_cnt {
///////////////////////////////////////
        //let file = "test.txt";
        let file = "input.txt";

        let nb_cycles = 6;

        let mut planete = read_input(&file, nb_cycles);
        for i in 0..nb_cycles {
            live(&mut planete);
            //print_cells(&planete, 6, 6);
        }

        answer1 = planete.cells.iter().filter(|&c| *c == Active).count();

///////////////////////////////////////
    }
    println!("\nanswer1={}, answer2={}", answer1, answer2);
    println!("[Time] Finished after {:?} (mean on {} iterations)\n", start.elapsed() / iter_cnt, iter_cnt);
}


fn read_input(path: &str, nb_ite: usize) -> Planete { 
    let f = fs::read_to_string(path).unwrap();
    let nb_lines = f.lines().count();
    let size_line = f.lines().next().unwrap().len();
    let w = size_line + 2 * nb_ite;
    let h = nb_lines + 2 * nb_ite;
    let p = 1 + 2 * nb_ite;
    let pp = p;

    let mut cells : Vec<Cell> = vec![Inactive; w * h * p * pp];
    let z = nb_ite;
    let zz = nb_ite;
    let mut y = nb_ite;
    for l in f.lines() {
        let mut x = nb_ite;
        for c in l.chars() {
            cells[xyz_to_idx((x,y,z, zz), (w,h,p, pp))] = match c { '#' => Active, _ => Inactive }; 
            x += 1;
        }
        y += 1;
    }
    Planete{cells, dim: (w, h, p, pp)}
}
