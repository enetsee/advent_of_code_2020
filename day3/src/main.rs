use std::fs::File;
use std::io::{BufRead, BufReader, Error};

fn main() {
    let grid = File::open("input.txt")
        .and_then(|f| read_lines(BufReader::new(f)))
        .unwrap();

    let grid_size = (grid[0].len(), grid.len());
    let trees = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .iter()
        .fold(1, |ttl, step_size| {
            ttl * trees_encountered(&grid, grid_size, *step_size)
        });
    println!("{}", trees);
}

fn trees_hit(grid: &Vec<Vec<bool>>, step_size: (usize, usize)) -> usize {
    grid.iter()
        .step_by(step_size.1)
        .fold((0, 0), |(x, ttl), row| {
            let x = (x + step_size.0) % row.len();
            if row[x] {
                (x, ttl + 1)
            } else {
                (x, ttl)
            }
        })
        .1
}

fn trees_encountered(
    grid: &Vec<Vec<bool>>,
    grid_size: (usize, usize),
    step_size: (usize, usize),
) -> usize {
    let mut trees = 0;
    let mut cur_pos = (0, 0);
    while let Some(next_pos) = step(grid_size, step_size, cur_pos) {
        if grid[next_pos.1][next_pos.0] {
            trees = trees + 1
        }
        cur_pos = next_pos
    }
    trees
}

fn step(
    grid_size: (usize, usize),
    step_size: (usize, usize),
    cur_pos: (usize, usize),
) -> Option<(usize, usize)> {
    if cur_pos.1 + step_size.1 >= grid_size.1 {
        None
    } else {
        Some((
            (cur_pos.0 + step_size.0) % grid_size.0,
            cur_pos.1 + step_size.1,
        ))
    }
}

fn read_lines<R: BufRead>(read: R) -> Result<Vec<Vec<bool>>, Error> {
    read.lines()
        .map(|ln| ln.map(|f| f.chars().map(|c| c == '#').collect()))
        .collect()
}

fn read_flat<R: BufRead>(rdr: R) -> Result<Vec<bool>, Error> {
    rdr.lines()
        .map(|res| res.map(|ln| ln.chars().map(|c| c == '#').collect()))
        .collect::<Result<Vec<Vec<bool>>, Error>>()
        .map(|xss| xss.concat())
}
