use std::fs;
use std::time::Instant;

#[allow(dead_code)]
fn count_group_or(grp_answers: &str) -> usize {
    let mut letters = String::new();
    
    for line in grp_answers.lines() {
        let mut new_letters = String::from(line);
        new_letters.retain(|c| !letters.contains(c));
        letters.push_str(new_letters.as_str());
    }

    letters.chars().count()
}


fn count_group_and(grp_answers: &str) -> usize {
    let mut letters = String::from("abcdefghijklmnopqrstuvwxyz");
    for line in grp_answers.lines() {
        letters.retain(|c| line.contains(c));
    }

    letters.chars().count()

}

fn main() -> Result<(), std::io::Error> {
    let iter_cnt = 500;
    let start = Instant::now();
    for i in 0..iter_cnt {
///////////////////////////////////////
        //let input     = fs::read_to_string("test.txt")?;
        let input     = fs::read_to_string("input.txt")?;
        let answers   = input.split("\r\n\r\n");

        let mut tot = 0;
        for grp in answers {
            tot += count_group_or(grp);
        }

        if i == 0 {
            println!("{}", tot);
        }
///////////////////////////////////////
    }
    let duration = start.elapsed();
    println!("Finished after {:?} (mean on {} iterations)", duration / iter_cnt, iter_cnt);
    Ok(())
}