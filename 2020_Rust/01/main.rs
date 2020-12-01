use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;

fn main() -> io::Result<()> {
    let f = File::open("input.txt")?;
    let f = BufReader::new(f);

    let mut vector: Vec<i32> = vec![];

    for line in f.lines() {
        match line {
            Ok(string) => match string.trim().parse::<i32>() {
                Err(err)   => panic!("{:?}", err),
                Ok(number) => vector.push(number)
            },
            Err(err)   => panic!("{:?}", err),
        }
    }

    for x in &vector {
        for y in &vector {
            for z in &vector {
                if x + y + z == 2020 {
                    println!("{}", x * y * z);
                }
            }
        }
    }
    
    Ok(())
}