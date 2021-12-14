use std::collections::{HashMap, HashSet};
use std::fs;

/// TYPES /////////////////////////////////////////////////////////////////////
type Links = HashMap<String, Vec<String>>;

/// FUNCTIONS /////////////////////////////////////////////////////////////////

/// MAIN and PARSE ////////////////////////////////////////////////////////////
fn main() {
    let file = "input.txt";
    let (nodes, links) = read_input(&file);

    let c1 = explore(&nodes, &links, &"start".to_string());
    let c2 = explore_twice(&nodes, &links, &"start".to_string(), false, &"".to_string()).len();

    println!("\nanswer1={}, answer2={}", c1, c2);
}

fn read_input(path: &str) -> (HashSet<String>, Links) {
    let txt = fs::read_to_string(path).unwrap();

    let mut nodes = HashSet::new();
    let mut links: Links = HashMap::new();

    for l in txt.lines() {
        let mut s = l.split("-");
        let n1 = s.next().unwrap().to_string();
        let n2 = s.next().unwrap().to_string();

        links.entry(n1.clone()).or_default().push(n2.clone());
        links.entry(n2.clone()).or_default().push(n1.clone());

        nodes.insert(n1);
        nodes.insert(n2);
    }

    (nodes, links)
}

fn explore(nodes: &HashSet<String>, links: &Links, curr: &String) -> usize {
    let mut left_nodes = nodes.clone();
    let mut tot_paths = 0;

    if curr == "end" {
        return 1;
    }

    if curr != &curr.to_uppercase() {
        left_nodes.remove(curr);
    }

    for n in &links[curr] {
        if nodes.contains(n) {
            tot_paths += explore(&left_nodes, &links, n);
        }
    }

    tot_paths
}


// Too lazw to filter doubled path that don't use doubling, so are redundantes
fn explore_twice(nodes: &HashSet<String>, links: &Links, curr: &String, doubled:  bool, path: &String) -> HashSet<String> {
    let mut left_nodes = nodes.clone();
    let mut found_paths = HashSet::new();

    if curr == "end" {
        found_paths.insert(path.clone());
        return found_paths;
    }

    if curr != &curr.to_uppercase() {
        if !doubled && curr != "start" {
            // Choose to double this one
            for n in &links[curr] {
                if nodes.contains(n) {
                    let paths = explore_twice(&left_nodes, &links, n, true, &(path.clone() + curr));
                    found_paths.extend(paths);
                }
            }
        }

        left_nodes.remove(curr);
    }

    for n in &links[curr] {
        if nodes.contains(n) {
            let paths = explore_twice(&left_nodes, &links, n, doubled, &(path.clone() + curr));
            found_paths.extend(paths);
        }
    }
    found_paths
}
