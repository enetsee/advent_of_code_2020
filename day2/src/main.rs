use std::fs::File;
use std::io::{self, BufRead, Error, ErrorKind};

fn main() {
    let x = File::open("input.txt")
        .and_then(|f| load(io::BufReader::new(f)))
        .unwrap()
        .iter()
        .fold(0, |ttl, (policy, pw)| {
            if is_valid_alt(policy, pw) {
                ttl + 1
            } else {
                ttl
            }
        });
    println!("{}", x)
}

#[derive(Debug, PartialEq)]
struct Policy {
    min_use: usize,
    max_use: usize,
    use_char: char,
}

fn is_valid(policy: &Policy, pw: &String) -> bool {
    let uses = pw
        .chars()
        .fold(0, |ttl, c| if policy.use_char == c { ttl + 1 } else { ttl });
    uses >= policy.min_use && uses <= policy.max_use
}
fn is_valid_alt(policy: &Policy, pw: &String) -> bool {
    let cs: Vec<_> = pw.chars().collect();
    let c = policy.use_char;
    let a = cs[policy.min_use];
    let b = cs[policy.max_use];
    (a == c) != (b == c)
}

#[cfg(test)]
mod validation {

    #[test]
    fn parse_policy() {
        let policy1 = super::Policy {
            use_char: 'a',
            min_use: 1,
            max_use: 3,
        };
        let policy2 = super::Policy {
            use_char: 'b',
            min_use: 1,
            max_use: 3,
        };
        let policy3 = super::Policy {
            use_char: 'c',
            min_use: 2,
            max_use: 9,
        };
        assert_eq!(super::parse_policy("1-3 a".to_string()).unwrap(), policy1);
        assert_eq!(super::parse_policy("1-3 b".to_string()).unwrap(), policy2);
        assert_eq!(super::parse_policy("2-9 c".to_string()).unwrap(), policy3);
    }

    #[test]
    fn is_valid_alt() {
        let policy1 = super::Policy {
            use_char: 'a',
            min_use: 1,
            max_use: 3,
        };
        let policy2 = super::Policy {
            use_char: 'b',
            min_use: 1,
            max_use: 3,
        };
        let policy3 = super::Policy {
            use_char: 'c',
            min_use: 2,
            max_use: 9,
        };
        assert_eq!(super::is_valid_alt(&policy1, &"abcde".to_string()), true);
        assert_eq!(super::is_valid_alt(&policy2, &"cdefg".to_string()), false);
        assert_eq!(
            super::is_valid_alt(&policy3, &"ccccccccc".to_string()),
            false
        );
    }
}

fn load<R>(read: R) -> io::Result<Vec<(Policy, String)>>
where
    R: BufRead,
{
    read.lines()
        .map(|lnres| lnres.and_then(|ln| parse_ln(ln)))
        .collect::<io::Result<Vec<(Policy, String)>>>()
}

fn split_once(on: char, in_string: String) -> Option<(String, String)> {
    let mut splitter = in_string.split(on);
    splitter
        .next()
        .map(|fst| (fst.to_string(), splitter.fold("".to_string(), |a, b| a + b)))
}

fn parse_policy(str: String) -> Option<Policy> {
    let (fst, snd) = split_once(' ', str)?;
    let use_char = snd.chars().next()?;
    let (low, high) = split_once('-', fst)?;
    let min_use = low.parse().ok()?;
    let max_use = high.parse().ok()?;
    Some(Policy {
        use_char,
        min_use,
        max_use,
    })
}
fn parse_ln(ln: String) -> Result<(Policy, String), Error> {
    match split_once(':', ln) {
        Some((policy_str, pw)) => parse_policy(policy_str)
            .map(|policy| (policy, pw))
            .ok_or(Error::new(ErrorKind::InvalidData, "")),
        _ => Err(Error::new(ErrorKind::InvalidData, "")),
    }
}
