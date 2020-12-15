use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
fn main() {
    let f = File::open("input.txt").unwrap();
    let mut adapters = BufReader::new(f)
        .lines()
        .map(|str_res| str_res.map(|str| str.parse().unwrap()))
        .collect::<io::Result<Vec<usize>>>()
        .unwrap();
    adapters.sort();
    let dist = greedy_all(&adapters);
    let mut cache: HashMap<usize, usize> = HashMap::new();
    let max_jolts = adapters[adapters.len() - 1];
    let n = arrangements(&adapters, &mut cache, max_jolts, 0);
    println!("1: {}, 2: {}, 3: {}", dist[0], dist[1], dist[2]);
    println!("{}", dist[0] * dist[2]);
    println!("{}", n)
}

fn greedy_all(adapters: &[usize]) -> [usize; 3] {
    let mut dist: [usize; 3] = [0, 0, 1];
    let mut cur_jolt = &0;
    for jolt in adapters {
        let delta = jolt - cur_jolt - 1;

        dist[delta] = dist[delta] + 1;

        cur_jolt = jolt;
    }
    dist
}

fn arrangements(
    adapters: &[usize],
    cache: &mut HashMap<usize, usize>,
    max_jolts: usize,
    base: usize,
) -> usize {
    if let Some(n) = cache.get(&base) {
        *n
    } else {
        let n = adapters
            .iter()
            .filter(|x| if **x > base { **x - base <= 3 } else { false })
            .fold(if base == max_jolts { 1 } else { 0 }, |acc, n| {
                acc + arrangements(adapters, cache, max_jolts, *n)
            });

        cache.insert(base, n);
        n
    }
}
