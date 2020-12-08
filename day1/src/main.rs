use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind};
// Load csv
// sort ascending
// binary search for entry such that x + y = 2020
fn main() {
    let mut vs: Vec<i32> = File::open("input.txt").and_then(|f| read_input(f)).unwrap();
    vs.sort();
    match sum3(2020, &vs) {
        Some((x, y, z)) => println!("{}", x * y * z),
        _ => println!("Can't be done"),
    }
}

fn sum3(tgt: i32, vs: &[i32]) -> Option<(i32, i32, i32)> {
    let mut res = None;
    for (xidx, x) in vs.iter().enumerate() {
        if let Some((y, z)) = sum2(tgt - x, &&vs[xidx + 1..]) {
            res = Some((*x, y, z));
            break;
        }
    }
    res
}

fn sum2(tgt: i32, vs: &[i32]) -> Option<(i32, i32)> {
    let mut res = None;
    for (xidx, x) in vs.iter().enumerate() {
        let sl = &vs[xidx + 1..];
        if let Some(yidx) = sl.binary_search(&(tgt - x)).ok() {
            res = Some((*x, sl[yidx]));
            break;
        }
    }
    res
}

fn read_input(file: File) -> Result<Vec<i32>, Error> {
    BufReader::new(file)
        .lines()
        .map(|line| line.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e))))
        .collect()
}
