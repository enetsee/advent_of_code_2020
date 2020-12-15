use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};
fn main() {
    let f = File::open("input.txt").unwrap();
    let mut nums: VecDeque<i64> = VecDeque::with_capacity(25);

    let (idx, not_sum) = BufReader::new(f)
        .lines()
        .map(|line| {
            let str = line.unwrap();
            let num: i64 = str.parse().unwrap();
            num
        })
        .enumerate()
        .find(|(idx, num)| {
            if idx < &25 {
                nums.push_back(*num);
                false
            } else if let Some(_) = sum_two(&nums, num) {
                nums.push_back(*num);
                nums.pop_front();
                false
            } else {
                println!("{} {} {:#?}", idx, num, nums);
                true
            }
        })
        .unwrap();

    let sl: Vec<i64> = BufReader::new(File::open("input.txt").unwrap())
        .lines()
        .take(idx)
        .map(|line| {
            let str = line.unwrap();
            let num: i64 = str.parse().unwrap();
            num
        })
        .collect();

    let (i, j) = conseq_sum(&sl, &not_sum).unwrap();

    let mut mn = None;
    let mut mx = None;
    for k in i..=j {
        if let Some(cur_mn) = mn {
            if sl[k] < cur_mn {
                mn = Some(sl[k])
            }
        } else {
            mn = Some(sl[k])
        }
        if let Some(cur_mx) = mx {
            if sl[k] > cur_mx {
                mx = Some(sl[k])
            }
        } else {
            mx = Some(sl[k])
        }
    }
    let weakness = mn.unwrap() + mx.unwrap();
    // println!("what??");
    println!("no sum for {}", not_sum);
    println!("weakness {}", weakness)
}

// [35,20,15,25,47 ,40 ,62 ]
//   1  2  3  4  5   6   7
// [35,55,70,95,142,182,244]
//                 -127
//                   55
fn conseq_sum(nums: &[i64], tgt: &i64) -> Option<(usize, usize)> {
    let mut ttl = 0;
    let mut pairs: HashMap<i64, usize> = HashMap::new();
    pairs.insert(0, 0);
    for (idx, num) in nums.iter().enumerate() {
        ttl = ttl + num;
        let key: i64 = ttl - tgt;

        if let Some(&jdx) = pairs.get(&key) {
            return Some((jdx, idx + 1));
        } else {
            pairs.insert(ttl, idx + 1)
        };
    }
    None
}

#[cfg(test)]
mod test {
    #[test]
    fn conseq_sum() {
        let input = [35, 20, 15, 25, 47, 40, 62];
        let ttl = 127;
        assert_eq!(Some((2, 6)), super::conseq_sum(&input, &ttl));
        // assert_eq!(1,2);
    }
}

fn sum_two(nums: &VecDeque<i64>, tgt: &i64) -> Option<(usize, usize)> {
    let mut pairs: HashMap<i64, usize> = HashMap::new();

    for (idx, num) in nums.iter().enumerate() {
        if let Some(&jdx) = pairs.get(num) {
            return Some((jdx, idx as usize));
        } else {
            pairs.insert(tgt - num, idx);
        };
    }
    None
}
