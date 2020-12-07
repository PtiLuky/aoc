use std::collections::hash_map::{HashMap, Entry};
use std::collections::HashSet;
use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;
use std::time::Instant;

type Content = Vec<(usize, String)>;
type Rules   = HashMap<String, Content>;
type Parents = HashMap<String, Vec<String>>;

fn read_input(path: &str) -> Rules {
    let f = File::open(path).unwrap();
    let f = BufReader::new(f);
    let mut bags_content = Rules::new();

    for line in f.lines() {
        let line          = line.unwrap();
        let mut ite_split = line.split(" bags contain ");
        let color_out     = ite_split.next().unwrap().to_string();
        let ite_split = ite_split.next().unwrap().split(", ");
        let mut content : Content = Content::new();
        for in_bags in ite_split {
            let mut color = String::new();
            let mut ite   = in_bags.split(" ");
            let count     = match ite.next().unwrap().parse::<usize>() { Ok(x) => x, Err(_) => 0};
            color +=  ite.next().unwrap();
            for word in ite {
                if word != "bag" && word != "bags" && word != "bag." && word != "bags." {
                    color.push_str(" ");
                    color.push_str(word);
                }
            }
            if count > 0 {
                content.push((count, String::from(&color)));
            }
        }
        bags_content.insert(color_out, content);
    }
    bags_content
}

fn list_parents(rules : &Rules) -> Parents {
    let mut parents = Parents::new();
    for (color_out, content) in rules {
        for (_, color_in) in content {
            match parents.entry(color_in.clone()) {
                Entry::Occupied(mut entry) => {
                    entry.get_mut().push(color_out.clone());
                },
                Entry::Vacant(entry) => {
                    entry.insert(vec![color_out.clone()]);
                },
            } 
        }
    }

    parents
}

fn count_parents_of(color: &str, parents : &Parents) -> usize {
    let mut can_contain = HashSet::<String>::new();
    let mut treating : Vec::<String> = vec![color.to_string()];

    let mut curr = treating.pop();
    while curr != None {
        let elem = curr.unwrap();
        can_contain.insert(elem.clone());

        let parents_of_elem = parents.get(&elem);
        if parents_of_elem != None {
            for p in parents_of_elem.unwrap() {
                if !can_contain.contains(p) {
                    treating.push(p.clone());
                }
            }
        }
        curr = treating.pop();
    }
    can_contain.len() - 1 // remove itself
}

fn count_bags_in(color: &str, rules: &Rules) -> usize {
    let mut tot = 0;
    let mut treating : Vec::<(usize, String)> = vec![(1, color.to_string())];

    let mut curr = treating.pop();
    while curr != None {
        let elem = curr.unwrap();
        let factor = elem.0;
        let elem   = elem.1;

        tot += factor;

        for (nb, name) in rules.get(&elem).unwrap() {
            treating.push((factor * nb, name.clone()));
        }
        curr = treating.pop();
    }
    tot - 1
}

fn main() -> Result<(), std::io::Error> {
    let iter_cnt = 100;
    let start = Instant::now();
    for i in 0..iter_cnt {
///////////////////////////////////////
        //let file = "test.txt";
        let file = "input.txt";
        
        let rules   = read_input(&file);
        let parents = list_parents(&rules);

        let count = count_parents_of("shiny gold", &parents);
        let count2 = count_bags_in("shiny gold", &rules);

        if i == 0 {
            println!("parents : {}", count);
            println!("children : {}", count2);
        }
///////////////////////////////////////
    }
    let duration = start.elapsed();
    println!("Finished after {:?} (mean on {} iterations)", duration / iter_cnt, iter_cnt);
    Ok(())
}