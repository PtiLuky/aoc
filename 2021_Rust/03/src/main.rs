use std::fs;

/// TYPES /////////////////////////////////////////////////////////////////////

/// FUNCTIONS /////////////////////////////////////////////////////////////////

/// MAIN and PARSE ////////////////////////////////////////////////////////////
fn main() {
    let file = "input.txt";
    let (vals, len) = read_input(&file);

    let (g, e) = gamma_espilon(&vals, len);
    let (o, co2) = oxygen_co2(&vals, len);

    println!("\nanswer1={}, answer2={} - {}, {}", g * e, o * co2, o, co2);
}

fn read_input(path: &str) -> (Vec<u32>, usize) {
    let txt = fs::read_to_string(path).unwrap();
    let len = txt.lines().next().unwrap().len();
    let data = txt
        .lines()
        .map(|v| u32::from_str_radix(&v, 2).unwrap())
        .collect();
    (data, len)
}

fn gamma_espilon(data: &Vec<u32>, len: usize) -> (u32, u32) {
    let mut counts = vec![0; len];

    for val in data.iter() {
        for i in 0..len {
            let n = len - 1 - i;
            if val & (1 << n) != 0 {
                counts[i] += 1;
            }
        }
    }

    let mut g = 0;
    let mut e = 0;
    for c in counts {
        g = g << 1;
        e = e << 1;
        if c * 2 >= data.len() {
            g += 1;
        } else {
            e += 1;
        }
    }
    (g, e)
}

fn oxygen_co2(data: &Vec<u32>, len: usize) -> (u32, u32) {
    // oxy
    let mut oxy_list= data.clone();
    for i in 0..len {
        if oxy_list.len() <= 1 {
            break;
        }

        let mut count = 0;
        let mask = 1 << (len - 1 - i);
        for v in oxy_list.iter() {
            if v & mask != 0 {
                count += 1;
            }
        }

        let l = oxy_list.len();
        oxy_list.retain(|&v| if count * 2 >= l { v & mask != 0 } else {v & mask == 0});
    }
    // co2
    let mut co2_list= data.clone();
    for i in 0..len {
        if co2_list.len() <= 1 {
            break;
        }

        let mut count = 0;
        let mask = 1 << (len - 1 - i);
        for v in co2_list.iter() {
            if v & mask != 0 {
                count += 1;
            }
        }

        let l = co2_list.len();
        co2_list.retain(|&v| if count * 2 < l { v & mask != 0 } else {v & mask == 0});
    }

    (oxy_list[0], co2_list[0])
}
