use crate::common::Solution;
use std::time::Instant;

static INPUT: &str = include_str!("../data/day_01.txt");

pub fn solve_day_01() -> Solution {
    let start = Instant::now();
    let (ls, rs) = parse_input(INPUT);
    let part_a = day_01_a(&ls, &rs); // 2086478
    let part_b = day_01_b(&ls, &rs); // 24941624
    let duration = start.elapsed();
    Solution {
        part_a,
        part_b,
        duration,
    }
}

fn day_01_a(_ls: &[usize], _rs: &[usize]) -> String {
    let mut ls = _ls.to_vec();
    ls.sort();
    let mut rs = _rs.to_vec();
    rs.sort();
    let v = ls
        .iter()
        .zip(rs)
        .map(|(l, r)| if *l > r { *l - r } else { r - *l })
        .sum::<usize>();
    format!("day_01_a = {}", v)
}

fn day_01_b(ls: &[usize], rs: &[usize]) -> String {
    let mut score = 0;
    for l in ls {
        score += rs.iter().filter(|r| **r == *l).sum::<usize>();
    }
    format!("day_01_b = {}", score)
}

fn parse_input(input: &str) -> (Vec<usize>, Vec<usize>) {
    input
        .split("\n")
        .filter(|s| !s.is_empty())
        .map(|s| {
            let mut ss = s.split(" ").filter(|s| !s.is_empty());
            let x = ss.next().unwrap().parse::<usize>().unwrap();
            let y = ss.next().unwrap().parse::<usize>().unwrap();
            (x, y)
        })
        .unzip()
}

#[cfg(test)]
mod tests {
    use super::*;

    static STR: &str = r"3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn a_test() {
        let (ls, rs) = parse_input(STR);
        println!("{:?}", &ls);
        println!("{:?}", &rs);

        let result = day_01_a(&ls, &rs);
        println!("{}", &result);
        assert_eq!("day_01_a = 11".to_string(), result);
    }

    #[test]
    fn b_test() {
        let (ls, rs) = parse_input(STR);
        let result = day_01_b(&ls, &rs);
        println!("{}", &result);
        assert_eq!("day_01_b = 31".to_string(), result);
    }
}
