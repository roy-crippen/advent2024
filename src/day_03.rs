use crate::common::Solution;
use regex::Regex;
use std::time::Instant;

static INPUT: &str = include_str!("../data/day_03.txt");

pub fn solve_day_03() -> Solution {
    let start = Instant::now();
    let part_a = day_03_a(INPUT); // 169021493
    let part_b = day_03_b(INPUT); // 111762583
    let duration = start.elapsed();
    Solution {
        part_a,
        part_b,
        duration,
    }
}

fn eval_mul(s: &str) -> usize {
    let binding = s.replace("mul(", "").replace(")", "");
    let ss: Vec<&str> = binding.split(",").collect();
    ss[0].parse::<usize>().unwrap() * ss[1].parse::<usize>().unwrap()
}

fn capture<'a>(re: &'a Regex, s: &'a str) -> Vec<&'a str> {
    re.captures_iter(s)
        .flat_map(|cap| cap.iter().map(|m| m.unwrap().as_str()).collect::<Vec<_>>())
        .collect()
}

fn day_03_a(s: &str) -> String {
    let re = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap();
    let xs: Vec<&str> = capture(&re, s);
    let v: usize = xs.iter().map(|x| eval_mul(x)).sum();
    format!("day_03_a = {}", v)
}

fn day_03_b(s: &str) -> String {
    let re = Regex::new(r"do\(\)|don't\(\)|mul\(\d{1,3},\d{1,3}\)").unwrap();
    let xs: Vec<&str> = capture(&re, s);

    let mut v = 0;
    let mut dont = false;
    for x in xs {
        let sub = &x[..3];
        if sub == "do(" {
            dont = false;
            continue;
        }
        if sub == "don" {
            dont = true;
            continue;
        }
        if !dont && sub == "mul" {
            v += eval_mul(x);
        }
    }

    format!("day_03_b = {}", v)
}

#[cfg(test)]
mod tests {
    use super::*;

    static STR_A: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    static STR_B: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn a_test() {
        let result = day_03_a(STR_A);
        println!("{}", &result);
        assert_eq!("day_03_a = 161".to_string(), result);
    }

    #[test]
    fn b_test() {
        let result = day_03_b(STR_B);
        println!("{}", &result);
        assert_eq!("day_03_b = 48".to_string(), result);
    }
}
