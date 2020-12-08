use std::fs::File;
use std::io::{self, BufRead, BufReader, Error, ErrorKind};

fn main() {
    let mut f = File::open("input.txt")
        .and_then(|f| {
            BufReader::new(f)
                .lines()
                .map(|res| res.map(|str| unique_id(decode(&str))))
                .collect::<io::Result<Vec<usize>>>()
        })
        .unwrap();
    f.sort();

    // let n = f.into_iter().max().unwrap();
    let m = find_missing(&f);
    let n = f.iter().max().unwrap();
    println!("max seat: {}", n);
    println!("my seat: {}", m)
}

fn find_missing(xs: &Vec<usize>) -> usize {
    let lr = xs.windows(2).find(|arr| arr[1] - arr[0] > 1).unwrap();
    lr[0] + 1
}

fn decode(str: &str) -> (usize, usize) {
    let (row_str, col_str) = str.split_at(7);
    let mut row_low = 0;
    let mut row_high = 127;
    let mut col_low = 0;
    let mut col_high = 7;
    for r in row_str.chars() {
        if r == 'F' {
            row_high = row_low + ((row_high - row_low) / 2);
        } else {
            row_low = row_high - ((row_high - row_low) / 2);
        }
    }
    for c in col_str.chars() {
        if c == 'L' {
            col_high = col_low + ((col_high - col_low) / 2);
        } else {
            col_low = col_high - ((col_high - col_low) / 2);
        }
    }
    (row_low, col_low)
}

fn unique_id(seat: (usize, usize)) -> usize {
    seat.0 * 8 + seat.1
}

#[cfg(test)]
mod test {
    #[test]
    fn decode() {
        assert_eq!(super::decode("BFFFBBFRRR"), (70, 7));
        assert_eq!(super::decode("FFFBBBFRRR"), (14, 7));
        assert_eq!(super::decode("BBFFBBFRLL"), (102, 4));
    }
}
