use crate::common::Solution;
use rayon::prelude::*;
use std::time::Instant;

static INPUT: &str = include_str!("../data/day_07.txt");

#[derive(Clone, Debug)]
pub struct Op {
    pub desired: usize,
    pub values: Vec<usize>,
}

pub fn solve_day_07() -> Solution {
    let start = Instant::now();
    let ops = parse(INPUT);
    let part_a = day_07_a(&ops); // 5837374519342
    let part_b = day_07_b(&ops); // 492383931650959
    let duration = start.elapsed();
    Solution {
        part_a,
        part_b,
        duration,
    }
}

fn day_07_a(ops: &[Op]) -> String {
    let v: usize = ops.iter().fold(0usize, |acc, op| {
        if eval_rtl_a(op.desired, op.values.clone()) {
            op.desired + acc
        } else {
            acc
        }
    });

    format!("day_07_a = {}", v)
}

fn day_07_b(ops: &[Op]) -> String {
    let v: usize = ops
        .par_iter()
        .fold(
            || 0usize,
            |acc, op| {
                if eval_rtl_b(op.desired, op.values.clone()) {
                    op.desired + acc
                } else {
                    acc
                }
            },
        )
        .sum();

    format!("day_07_b = {}", v)
}

pub fn eval_rtl_a(desired: usize, values: Vec<usize>) -> bool {
    match values.len() {
        0 | 1 => false,
        2 => values[0] + values[1] == desired || values[0] * values[1] == desired,
        _ => {
            let mut new_values = values.clone();
            let value = new_values.pop().unwrap();

            let is_mul = desired % value == 0 && eval_rtl_a(desired / value, new_values.clone());
            let is_add = desired >= value && eval_rtl_a(desired - value, new_values);
            is_mul || is_add
        }
    }
}

pub fn eval_rtl_b(desired: usize, values: Vec<usize>) -> bool {
    match values.len() {
        0 => false,
        1 => values[0] == desired,
        2 => {
            values[0] + values[1] == desired
                || values[0] * values[1] == desired
                || concat_usize(values[0], values[1]) == desired
        }
        _ => {
            let mut new_values = values.clone();
            let value = new_values.pop().unwrap();

            let is_mul = desired % value == 0 && eval_rtl_b(desired / value, new_values.clone());
            let is_add = desired >= value && eval_rtl_b(desired - value, new_values.clone());
            let mut is_concat = false;
            if let Some(lhs) = un_concat_usize(desired, value) {
                is_concat = eval_rtl_b(lhs, new_values);
            }
            is_mul || is_add || is_concat
        }
    }
}

fn concat_usize(a: usize, b: usize) -> usize {
    let mut base = 1;
    let mut t: usize = b;
    while t > 0 {
        base *= 10;
        t /= 10;
    }
    a * base + b
}

fn un_concat_usize(joined: usize, right_side: usize) -> Option<usize> {
    let mut divisor = 10;
    let mut check = right_side;
    while check >= 10 {
        divisor *= 10;
        check /= 10;
    }

    // let rhs = joined % divisor;
    // println!("joined % divisor: {}", rhs);
    if joined % divisor != right_side {
        return None;
    }

    Some(joined / divisor)
}

fn parse(input: &str) -> Vec<Op> {
    let mut evals: Vec<Op> = Vec::new();
    for line in input.lines() {
        let parts: Vec<&str> = line.split(":").collect();
        let desired: usize = parts[0].parse().unwrap();
        let s = parts[1].trim();
        let values: Vec<usize> = s.split(" ").map(|x| x.parse().unwrap()).collect();
        evals.push(Op { desired, values });
    }

    evals
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::perms::Perms;

    static STR: &str = r"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    #[test]
    fn a_test() {
        let ops = parse(STR);
        let result = day_07_a(&ops);
        println!("{}", &result);
        assert_eq!("day_07_a = 3749".to_string(), result);
    }

    #[test]
    fn b_test() {
        let ops = parse(STR);
        let result = day_07_b(&ops);
        println!("{}", &result);
        assert_eq!("day_07_b = 11387".to_string(), result);
    }

    #[test]
    fn eval_rtl_a_test() {
        let desired = 3267;
        let values = vec![81, 40, 27];
        let result = eval_rtl_a(desired, values);
        assert!(result);

        let desired = 21037;
        let values = vec![9, 7, 18, 13];
        let result = eval_rtl_a(desired, values);
        assert!(!result);
    }

    #[test]
    fn eval_rtl_b_test() {
        let desired = 7290;
        let values = vec![6, 8, 6, 15];
        let result = eval_rtl_b(desired, values);
        assert!(result);

        let desired = 192;
        let values = vec![17, 8, 14];
        let result = eval_rtl_b(desired, values);
        assert!(result);

        let desired = 21037;
        let values = vec![9, 7, 18, 13];
        let result = eval_rtl_b(desired, values);
        assert!(!result);
    }

    #[test]
    fn perms_char_test() {
        let perms = Perms::new(2, vec!['a', 'b']);
        for perm in perms {
            println!("{:?}", perm);
        }
    }

    #[test]
    fn concat_usize_test() {
        let v = concat_usize(15, 6);
        assert_eq!(v, 156);
        let v = concat_usize(1234, 5678);
        assert_eq!(v, 12345678);

        let lhs = un_concat_usize(12345678, 5678);
        assert!(lhs.is_some());
        assert_eq!(lhs.unwrap(), 1234);
    }
}
