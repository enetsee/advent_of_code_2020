use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead, BufReader, ErrorKind};
fn main() {
    let f = File::open("input.txt").unwrap();
    let ops = BufReader::new(f)
        .lines()
        .map(|res| res.and_then(|str| parse_op(&str)))
        .collect::<io::Result<Vec<Op>>>()
        .unwrap();

    let original_result = run(&ops);
    match original_result {
        ProgResult::TerminatedCorrectly(n) => {
            println!("Original program terminated correctly: {}", n)
        }
        ProgResult::TerminatedIncorrectly(n) => {
            println!("Original program terminated incorrectly: {}", n)
        }
        ProgResult::InfiniteLoop(n) => println!("Original program did not terminate: {}", n),
    }
    let corrected_result = run_corrected(&ops).unwrap();

    println!(
        "Corrected program terminated correctly: {}",
        corrected_result
    )
}

#[derive(Debug, Eq, PartialEq)]
enum Op {
    NOP(i32),
    ACC(i32),
    JMP(i32),
}

#[derive(Debug, Eq, PartialEq)]
enum ProgResult {
    TerminatedCorrectly(i32),
    TerminatedIncorrectly(i32),
    InfiniteLoop(i32),
}

fn run_corrected(ops: &[Op]) -> Option<i32> {
    let mut cur_op = 0;
    let mut acc = 0;
    let mut out = None;

    while out.is_none() {
        match ops[cur_op] {
            Op::NOP(0) => cur_op = cur_op + 1,
            Op::NOP(n) => {
                if let ProgResult::TerminatedCorrectly(x) = run_from(add(cur_op, n), acc, ops) {
                    out = Some(x);
                } else {
                    cur_op = cur_op + 1
                }
            }
            Op::JMP(n) => {
                if let ProgResult::TerminatedCorrectly(x) = run_from(cur_op + 1, acc, ops) {
                    out = Some(x);
                } else {
                    cur_op = add(cur_op, n)
                }
            }
            Op::ACC(n) => {
                cur_op = cur_op + 1;
                acc = acc + n
            }
        }
    }

    out
}

fn add(c: usize, n: i32) -> usize {
    if n.is_negative() {
        c - n.wrapping_abs() as u32 as usize
    } else {
        c + n as usize
    }
}

fn run(ops: &[Op]) -> ProgResult {
    run_from(0, 0, ops)
}

fn run_from(pos: usize, acc_in: i32, ops: &[Op]) -> ProgResult {
    let final_op = ops.len();
    let mut cur_op = pos;
    let mut acc = acc_in;
    let mut seen: HashSet<usize> = HashSet::new();
    while !seen.contains(&cur_op) && cur_op < final_op {
        seen.insert(cur_op);
        match ops[cur_op] {
            Op::NOP(_) => cur_op = cur_op + 1,
            Op::ACC(n) => {
                cur_op = cur_op + 1;
                acc = acc + n;
            }
            Op::JMP(n) => cur_op = add(cur_op, n),
        }
    }
    if cur_op == final_op {
        ProgResult::TerminatedCorrectly(acc)
    } else if seen.contains(&cur_op) {
        ProgResult::InfiniteLoop(acc)
    } else {
        ProgResult::TerminatedIncorrectly(acc)
    }
}

fn parse_op(str: &str) -> io::Result<Op> {
    let parts: Vec<&str> = str.splitn(2, ' ').collect();
    let n: i32 = parts[1]
        .parse()
        .map_err(|_| io::Error::new(ErrorKind::InvalidData, parts[1]))?;
    match parts[0] {
        "nop" => Ok(Op::NOP(n)),
        "acc" => Ok(Op::ACC(n)),
        "jmp" => Ok(Op::JMP(n)),
        otherwise => Err(io::Error::new(ErrorKind::InvalidData, otherwise)),
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn parse_op() {
        use super::Op;

        assert_eq!(super::parse_op("jmp +6").unwrap(), Op::JMP(6));
        assert_eq!(super::parse_op("nop +1").unwrap(), (Op::NOP(1)));
        assert_eq!(super::parse_op("jmp -6").unwrap(), (Op::JMP(-6)));
        assert_eq!(super::parse_op("acc -6").unwrap(), (Op::ACC(-6)));
        // assert_eq!(super::parse_op("jop +6"), Err(ParseErr::NotAnOp));
        // assert_eq!(super::parse_op("jmp y"), Err(ParseErr::NotAnInt));
    }
}
