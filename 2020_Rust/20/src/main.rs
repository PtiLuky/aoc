use std::fs;
use std::time::Instant;
/*
Explanation: 
For each tile, we keep only id + a hash per side + an inversed hash per side.

Let's have the tile : 
A ... B
. ... .   for the rest we skip the "...":    AB
C ... D                                      CD


Let's not the hash of (A...B) as ab and the inversed hash of (A...B) = ba
We define the hash array as :
    [(ab, ba), // top,   clockwise then counter-clockwise
    (bd, db),  // right, clockwise then counter-clockwise
    (dc, cd),  // bot,   clockwise then counter-clockwise
    (ca, ac)]  // left,  clockwise then counter-clockwise}


We define the orientation as n => n * 90 counterclockwise
We define the flip as true => "flip horizontally AFTER orientation"
Rota 0 :         Rota 0 flipped:
   AB                   BA
   CD                   DC
Rota 1 :         Rota 1 flipped:
   BD                   DB
   AC                   CA
Rota 2 :         Rota 2 flipped:
   DC                   CD
   BA                   AB
Rota 2 :         Rota 3 flipped:
   CA                   AC
   DB                   BD

So we have : 
top_hash     = hash[orientation].0 
top_hash_inv = hash[orientation].1

right_hash     = hash[orientation + 1].0 
right_hash_inv = hash[orientation + 3].1

bot_hash     = hash[orientation + 2].1
bot_hash_inv = hash[orientation + 2].0

left_hash       = hash[orientation + 3].1
left_hash_inv   = hash[orientation + 1].0


*/
/// TYPES /////////////////////////////////////////////////////////////////////
struct Tile {
    id: u32,
    hash: [(u16, u16); 4],
    text: String
}
type Tiles = Vec<Tile>;
#[derive(Clone, Copy, Debug)]
struct Cell {
    tile: Option<u32>,
    orientation: usize, // in [0;4[
    flipped: bool,
}
type Solution = Vec<Cell>;

/// FUNCTIONS /////////////////////////////////////////////////////////////////
fn get_hash(cell: &Cell, tiles: &Tiles, orientation: usize) -> Option<u16> {
    if cell.tile.is_none() { return None }
    for t in tiles {
        if cell.tile == Some(t.id) {
            if cell.flipped {
                if orientation == 0 {
                    return Some(t.hash[(cell.orientation + 0) % 4].1);
                } else if orientation == 1 {
                    return Some(t.hash[(cell.orientation + 3) % 4].1);
                } else if orientation == 2 {
                    return Some(t.hash[(cell.orientation + 2) % 4].0);
                } else if orientation == 3 {
                    return Some(t.hash[(cell.orientation + 1) % 4].0);
                }
            } else {
                if orientation == 0 {
                    return Some(t.hash[(cell.orientation + 0) % 4].0);
                } else if orientation == 1 {
                    return Some(t.hash[(cell.orientation + 1) % 4].0);
                } else if orientation == 2 {
                    return Some(t.hash[(cell.orientation + 2) % 4].1);
                } else if orientation == 3 {
                    return Some(t.hash[(cell.orientation + 3) % 4].1);
                }
            }
        }
    }
    None
}

fn can_be_placed(cell: &Cell, tiles: &Tiles, t: &Option<u16>, r: &Option<u16>, b: &Option<u16>, l: &Option<u16>) -> bool {
    if t.is_some() && t != &get_hash(cell, tiles, 0) {
        return false;
    }
    if r.is_some() && r != &get_hash(cell, tiles, 1) {
        return false;
    }
    if b.is_some() && b != &get_hash(cell, tiles, 2) {
        return false;
    }
    if l.is_some() && l != &get_hash(cell, tiles, 3) {
        return false;
    }
    true
}

fn candidates(idx: usize, sol: &Solution, not_placed: &Vec<u32>, tiles: &Tiles, w: usize) -> Vec<Cell> {
    let x = idx % w;
    let y = idx / w;
    let mut t = Option::<u16>::None;
    let mut r = Option::<u16>::None;
    let mut b = Option::<u16>::None;
    let mut l = Option::<u16>::None;
    if x > 0 {
        let neig_idx = (x - 1) + y * w;
        l = get_hash(&sol[neig_idx], tiles, 1);
    }
    if y > 0 {
        let neig_idx = x + (y - 1) * w;
        t = get_hash(&sol[neig_idx], tiles, 2);
    }
    if x < w - 1 {
        let neig_idx = (x + 1) + y * w;
        r = get_hash(&sol[neig_idx], tiles, 3);
    }
    if y < w - 1 {
        let neig_idx = x + (y + 1) * w;
        b = get_hash(&sol[neig_idx], tiles, 0);
    }

    let mut candi = Vec::<Cell>::new();
    if t.is_some() || r.is_some() || b.is_some() || l.is_some() {
        for id in not_placed {
            for o in 0..4 {
                for f in &[false, true] {
                    let c = Cell { tile: Some(*id), orientation: o, flipped: *f};
                    if can_be_placed(&c, tiles, &t, &r, &b, &l) {
                        candi.push(c);
                    }
                }
            }
        }
    }
    candi
}

fn finished(sol: &Solution, w: usize, side_square: usize) -> usize {
    let mut first_x : Option<usize> = None;
    let mut first_y : Option<usize> = None;
    let mut last_x : Option<usize> = None;
    let mut last_y : Option<usize> = None;
    for x in 0..w {
        for y in 0..w {
            if sol[x + y * w].tile.is_some() {
                if first_x.is_none() { first_x = Some(x); }
                if first_y.is_none() { first_y = Some(y); }
                last_x = Some(x);
                last_y = Some(y);
            }
        }
    }
    if first_x.is_some() && first_y.is_some() && last_x.is_some() && last_y.is_some() {
        let first_x = first_x.unwrap();
        let first_y = first_y.unwrap();
        let last_x  = last_x.unwrap();
        let last_y  = last_y.unwrap();
        if (last_x - first_x + 1 ) == side_square && (last_y - first_y + 1 ) == side_square {
            return sol[first_x + first_y * w].tile.unwrap() as usize 
            * sol[first_x + last_y * w].tile.unwrap() as usize 
            * sol[last_x  + first_y * w].tile.unwrap() as usize 
            * sol[last_x + last_y * w].tile.unwrap() as usize;
        }
    }
    0
}

fn search_solution(sol: Solution, w: usize, not_placed: Vec<u32>, tiles: &Tiles) -> Option<Solution> {
    for i in 0..(w*w) {
        if sol[i].tile.is_none() {
            let candi = candidates(i, &sol, &not_placed, &tiles, w);
            for c in candi {
                let mut next_not_placed = not_placed.clone();
                next_not_placed.retain(|x| Some(*x) != c.tile);

                let mut next_sol = sol.clone();
                next_sol[i] = c;

                let res = search_solution(next_sol, w, next_not_placed, tiles);
                if res.is_some() {
                    return res;
                }
            }
        }
    }
    if finished(&sol, w, (w+1)/2) != 0 {
        return Some(sol)
    } else {
        None
    }
}

fn find_solution(tiles: &Tiles) -> (usize, Solution) {
    let nb_tiles = tiles.len();
    let side_square = (nb_tiles as f32).sqrt() as usize;
    let w = side_square * 2 - 1 ;

    let mut sol : Solution = vec![Cell{tile: None, orientation: 0, flipped: false}; w*w];
    let mut not_placed = Vec::<u32>::new();

    for t in tiles {
        not_placed.push(t.id);
    }

    sol[(side_square - 1) * (w + 1)].tile = not_placed.pop();

    let sol = search_solution(sol, w, not_placed, tiles).unwrap();

    (finished(&sol, w, side_square), sol)
}



fn global_string(sol: &Solution, tiles: &Tiles) -> String {
    let side_square = (tiles.len() as f32).sqrt() as usize;
    let w = side_square * 2 - 1 ;

    let mut lines : Vec<String> = vec!["".to_string(); side_square * 8];
    let mut first_x : Option<usize> = None;
    for x in 0..w {
        for y in 0..w {
            let sol = &sol[x + y * w];
            match sol.tile {
                Some(idx) => {
                    println!("{:?}",sol);
                    if first_x.is_none() { first_x = Some(x) };
                    for t in tiles {
                        if t.id == idx {
                            let txt : Vec<&str> = t.text.lines().collect();
                            let txt_w = txt.len();
                            for i_y in 1..(txt_w-1) {
                                // per line
                                for i_x in 1..(txt_w-1) {
                                    // per char
                                    let mut original_x = i_x;//match sol.orientation { 0 => i_x, 1 => i_y, 2 => txt_w - i_x, _ => txt_w - i_y};
                                    let original_y = i_y;//match sol.orientation { 3 => i_x, 0 => i_y, 1 => txt_w - i_x, _ => txt_w - i_y};
                                    //if sol.flipped { original_x = txt_w - original_x; }
                                    let substr = &txt[original_y][original_x..(original_x+1)];
                                    lines[(x - first_x.unwrap()) * 8 + i_y - 1].push_str(substr);
                                }
                            }
                        }
                    }
                },
                None => {}
            }
        }
    }
    for l in lines {
        println!("{}", l);
    }
    "".to_string()
}

/// MAIN and PARSE ////////////////////////////////////////////////////////////
fn main() {
    let iter_cnt = 1;
    let start = Instant::now();
    let mut answer1 = 0;
    for _ in 0..iter_cnt {
///////////////////////////////////////
        let file = "test.txt";
        //let file = "input.txt";

        let tiles = read_input(&file);
        let (product, sol) = find_solution(&tiles);
        answer1 = product;
        global_string(&sol, &tiles);

///////////////////////////////////////
    }
    println!("\nanswer1={}", answer1);
    println!("[Time] Finished after {:?} (mean on {} iterations)\n", start.elapsed() / iter_cnt, iter_cnt);
}


fn read_input(path: &str) -> Tiles{ 
    let input = fs::read_to_string(path).unwrap().replace("\r", "");

    let mut tiles = Tiles::new();

    for l in input.split("\n\n") {
        let id = l[5..l.find(":").unwrap()].parse::<u32>().unwrap();
        let l = &l[l.find("\n").unwrap() + 1..l.len()];
        let w = l.find("\n").unwrap(); // don't forget to use w+1 for the \n
        let mut t0 = 0;
        let mut t1 = 0;
        let mut r0 = 0;
        let mut r1 = 0;
        let mut b0 = 0;
        let mut b1 = 0;
        let mut l0 = 0;
        let mut l1 = 0;
        for i in 0..w {
            // top hashes
            let idx = i;
            let bit = match &l[idx..idx+1] { "#" => 1, _ => 0};
            t0 = (t0 << 1) + bit;
            t1 += bit << i;

            // right hashed
            let idx = (w + 1) * (i + 1) - 2;
            let bit = match &l[idx..idx+1] { "#" => 1, _ => 0};
            r0 = (r0 << 1) + bit;
            r1 += bit << i;

            // bottom hashes
            let idx = (w * (w + 1)) - 2 - i;
            let bit = match &l[idx..idx+1] { "#" => 1, _ => 0};
            b0 = (b0 << 1) + bit;
            b1 += bit << i;

            // left hashes
            let idx = (w + 1) * i;
            let bit = match &l[idx..idx+1] { "#" => 1, _ => 0};
            l1 = (l1 << 1) + bit;
            l0  += bit << i;

        }
        tiles.push(Tile{ id, hash: [(t0, t1), (r0, r1), (b0, b1), (l0, l1)], text: l.to_string()});
    }

    tiles
}
