use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;
use std::time::Instant;

struct Map {
    w: usize,
    h: usize,
    trees: Vec<bool>,
}

// File to vec<bool>
fn read_map(filename: &str) -> Result<Map, std::io::Error> {
    let mut w = 0;
    let mut h = 0;
    let mut trees : Vec<bool> = Vec::new();

    let f = File::open(filename)?;
    let f = BufReader::new(f);
    for line in f.lines() {
        let line = line?;
        h += 1;
        w = line.chars().count();
        for c in line.chars() {
            trees.push(c == '#');
        }
    }

    Ok(Map{w, h, trees})
}

// Count trees on slope
fn test_slope(map : &Map, dx: usize, dy: usize) -> usize {
    let mut x = 0;
    let mut y = 0;
    let mut cnt_trees = 0;

    while y < map.h {
        if map.trees[x + y * map.w] {
            cnt_trees += 1;
        }
        x = (x + dx) % map.w;
        y = y + dy;
    }
    cnt_trees
}

fn main() -> Result<(), std::io::Error> {
    let start = Instant::now();
    for i in 0..1000 {
///////////////////////////////////////
        let mut prod = 1;
        //let map = read_map("test.txt")?;
        let map = read_map("input.txt")?;
    
        prod *= test_slope(&map, 1, 1);        
        prod *= test_slope(&map, 3, 1);
        prod *= test_slope(&map, 5, 1);
        prod *= test_slope(&map, 7, 1);
        prod *= test_slope(&map, 1, 2);

        if i == 0 {
            println!("{}", prod);
        }
///////////////////////////////////////
    }
    let duration = start.elapsed();
    println!("Finished after {:?} (mean on 1000 iterations)", duration / 1000);
    Ok(())
}