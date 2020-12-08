use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
fn main() {
    let f = File::open("input.txt").unwrap();
    let rdr = BufReader::new(f);
    let mut h: HashSet<char> = HashSet::new();
    let mut g: HashSet<char> = HashSet::new();
    let mut first_line = true;
    let mut line_total;
    let mut ttl = 0;

    for res in rdr.lines() {
        let str = res.unwrap();

        if str == "" {
            line_total = h.len();
            println!("total {} ", line_total);
            h.clear();
            first_line = true;
            ttl = ttl + line_total
        } else {
            if first_line {
                for c in str.chars() {
                    h.insert(c);
                }
                first_line = false;
                println!("first {} : {}", str, h.len())
            } else {
                g.clear();
                for c in str.chars() {
                    g.insert(c);
                }
                h = h.intersection(&g).map(|c| *c).collect();
                println!("not first {} : {}, intersect: {}", str, g.len(), h.len())
            }
        }
    }
    h.intersection(&g);
    println!("{}", ttl + h.len());
}
