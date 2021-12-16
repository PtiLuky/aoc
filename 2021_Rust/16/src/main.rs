use bitreader::BitReader;
use std::fs;
extern crate hex;

/// TYPES /////////////////////////////////////////////////////////////////////

/// FUNCTIONS /////////////////////////////////////////////////////////////////

/// MAIN and PARSE ////////////////////////////////////////////////////////////
fn main() {
    let file = "input.txt";
    let data = read_input(&file);
    let mut reader = BitReader::new(&data[..]);
    let (_, _, val, tot) = read_packet(&mut reader).unwrap();
    let c1 = tot;
    let c2 = val;

    println!("\nanswer1={}, answer2={}", c1, c2);
}

fn read_input(path: &str) -> Vec<u8> {
    let txt = fs::read_to_string(path).unwrap();
    hex::decode(txt).unwrap()
}

fn read_packet(
    mut reader: &mut BitReader,
) -> Result<(u8, u8, u64, usize), bitreader::BitReaderError> {
    let version = reader.read_u8(3)?;
    let tpe = reader.read_u8(3)?;

    let mut tot_version = version as usize;

    let mut val = 0;

    if tpe == 4 {
        let mut keep_going = true;
        while keep_going {
            keep_going = reader.read_bool()?;
            val = (val << 4) + reader.read_u64(4)?;
        }
    } else {
        let length_type = reader.read_bool()?;
        let mut values = Vec::new();

        if length_type {
            let length_nb = reader.read_u64(11)?;
            for _ in 0..length_nb {
                let (_, _, val, tot) = read_packet(&mut reader)?;
                values.push(val);
                tot_version += tot;
            }
        } else {
            let length_bits = reader.read_u64(15)?;
            let will_be_left = reader.remaining() - length_bits;
            while reader.remaining() > will_be_left {
                let (_, _, val, tot) = read_packet(&mut reader)?;
                values.push(val);
                tot_version += tot;
            }
        }
        val = compute(tpe, &values)
    }

    Ok((version, tpe, val, tot_version))
}

fn compute(tpe: u8, vals: &Vec<u64>) -> u64 {
    match tpe {
        0 => vals.iter().sum(),
        1 => vals.iter().product(),
        2 => *vals.iter().min().unwrap(),
        3 => *vals.iter().max().unwrap(),
        5 => if vals[0] > vals[1] { 1 } else { 0 }
        6 => if vals[0] < vals[1] { 1 } else { 0 }
        7 => if vals[0] == vals[1] { 1 } else { 0 }
        _ => 0,
    }
}
