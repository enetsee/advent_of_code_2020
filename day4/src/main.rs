use regex::Regex;
use std::fs::read_to_string;
use std::ops::Range;
use std::{collections::HashMap, ops::RangeInclusive};

fn main() {
    let n = read_to_string("input.txt")
        .unwrap()
        .split("\n\n")
        .into_iter()
        .fold(0, |ttl, str| {
            let flds = parse_fields(str);

            if valid_fields(flds) {
                ttl + 1
            } else {
                ttl
            }
        });
    println!("{}", n)
}

fn parse_fields(dtls: &str) -> HashMap<&str, &str> {
    dtls.split_whitespace()
        .map(|pair| {
            let xs: Vec<&str> = pair.splitn(2, ':').collect();
            (xs[0], xs[1])
        })
        .collect()
}

const KEYS: [&str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

fn valid_int_in_range(range: RangeInclusive<usize>, val: &str) -> bool {
    val.parse::<usize>()
        .map(|n| if range.contains(&n) { true } else { false })
        .unwrap_or(false)
}

fn valid_height(s: &str) -> bool {
    s.strip_suffix("cm")
        .map(|h| valid_int_in_range(150..=193, h))
        .or_else(|| s.strip_suffix("in").map(|h| valid_int_in_range(59..=76, h)))
        .unwrap_or(false)
}

fn valid_hair_color(s: &str) -> bool {
    s.strip_prefix("#")
        .map(|ds| {
            ds.chars().all(|c| match c {
                '0'..='9' | 'a'..='f' => true,
                _ => false,
            })
        })
        .unwrap_or(false)
}
fn valid_eye_color(clr: &str) -> bool {
    match clr {
        "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => true,
        _ => false,
    }
}

fn valid_passport_id(s: &str) -> bool {
    let cs: Vec<char> = s.chars().collect();
    cs.len() == 9 && cs.iter().all(|c| c.is_numeric())
}

fn valid_fields(dtls: HashMap<&str, &str>) -> bool {
    dtls.get("byr")
        .map_or(false, |s| valid_int_in_range(1920..=2002, s))
        && dtls
            .get("iyr")
            .map_or(false, |s| valid_int_in_range(2010..=2020, s))
        && dtls
            .get("eyr")
            .map_or(false, |s| valid_int_in_range(2020..=2030, s))
        && dtls.get("hgt").map_or(false, |s| valid_height(s))
        && dtls.get("hcl").map_or(false, |s| valid_hair_color(s))
        && dtls.get("ecl").map_or(false, |s| valid_eye_color(s))
        && dtls.get("pid").map_or(false, |s| valid_passport_id(s))
}

fn valid_without_cid(dtls: HashMap<&str, &str>) -> bool {
    KEYS.iter().all(|k| dtls.contains_key(k))
}

#[cfg(test)]
mod test {
    #[test]
    fn split_blank_line() {
        let str = "a\n\n\n
        b
        c
        
        d"
        .to_string();
        let lns: Vec<&str> = str.split("\n\n").collect();
        assert_eq!(lns.len(), 3)
    }
}
