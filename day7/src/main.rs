#[macro_use]
extern crate lazy_static;
extern crate regex;
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

type Edges = HashMap<String, Vec<(usize, String)>>;

fn main() {
    let f = File::open("input.txt").unwrap();
    let edges: Edges = BufReader::new(f)
        .lines()
        .map(|res| res.map(|rule_str| parse_rule(rule_str)))
        .collect::<io::Result<HashMap<String, Vec<_>>>>()
        .unwrap();
    let tgt = "shiny gold".to_string();
    let n = edges
        .iter()
        .filter(|(key, _)| eventually_contains(&edges, key, &tgt))
        .count();
    let contained = bags_contained(&edges, &tgt);
    println!("eventually contain: {}, contained within: {}", n, contained)
}

fn bags_contained(edges: &Edges, bag_color: &String) -> usize {
    match edges.get(bag_color) {
        Some(xs) => xs.iter().fold(0, |ttl, (n, c)| {
            let x = bags_contained(edges, c);
            ttl + n + n * x
        }),
        _ => 0,
    }
}

fn eventually_contains(edges: &Edges, src_color: &String, dest_color: &String) -> bool {
    edges
        .get(src_color)
        .map(|dests| {
            dests
                .iter()
                .any(|(_, c)| c.eq(dest_color) || eventually_contains(edges, c, dest_color))
        })
        .unwrap_or(false)
}

fn parse_rule(rule_str: String) -> (String, Vec<(usize, String)>) {
    lazy_static! {
        static ref SRC_RE: Regex =
            Regex::new(r"^(?P<color>[a-z]+\s[a-z]+)(?:\sbags contain)").unwrap();
    }
    lazy_static! {
        static ref DEST_RE: Regex = Regex::new(r"(?P<num>\d+)\s(?P<color>[a-z]+\s[a-z]+)").unwrap();
    }

    let src = SRC_RE
        .captures(&rule_str)
        .unwrap()
        .name("color")
        .unwrap()
        .as_str()
        .to_owned();

    let dests = DEST_RE
        .captures_iter(&rule_str)
        .map(|caps| {
            (
                caps.name("num").unwrap().as_str().parse().unwrap(),
                caps.name("color").unwrap().as_str().to_owned(),
            )
        })
        .collect();

    (src, dests)
}

#[cfg(test)]
mod test {
    use std::vec;

    #[test]
    fn parse_rule() {
        let str1 = "dotted black bags contain 3 striped turquoise bags, 4 dark tan bags, 4 vibrant lavender bags.";
        assert_eq!(
            super::parse_rule(str1.to_string()),
            (
                "dotted black".to_string(),
                vec![
                    (3, "striped turquoise".to_string()),
                    (4, "dark tan".to_string()),
                    (4, "vibrant lavender".to_string())
                ]
            )
        );
        let str2 =
            "faded blue bags contain 4 clear salmon bags, 1 light cyan bag, 5 wavy tomato bags.";
        assert_eq!(
            super::parse_rule(str2.to_string()),
            (
                "faded blue".to_string(),
                vec![
                    (4, "clear salmon".to_string()),
                    (1, "light cyan".to_string()),
                    (5, "wavy tomato".to_string())
                ]
            )
        );
    }
}
