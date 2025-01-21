use crate::common::Solution;
use std::time::Instant;

static INPUT: &str = include_str!("../data/day_02.txt");

pub fn solve_day_02() -> Solution {
    let start = Instant::now();
    let xss = parse_input(INPUT);
    let part_a = day_02_a(&xss); // 390
    let part_b = day_02_b(&xss); // 439
    let duration = start.elapsed();
    Solution {
        part_a,
        part_b,
        duration,
    }
}

fn day_02_a(xs: &[Vec<i16>]) -> String {
    let v = xs
        .iter()
        .fold(0, |acc, v| acc + if is_safe(v) { 1 } else { 0 });
    format!("day_02_a = {}", v)
}

fn day_02_b(xss: &[Vec<i16>]) -> String {
    let mut score = 0;
    for xs in xss {
        if is_safe(xs) {
            score += 1;
        } else {
            for i in 0..xs.len() {
                let mut ys = xs.clone();
                ys.remove(i);
                if is_safe(&ys) {
                    score += 1;
                    break;
                }
            }
        }
    }
    format!("day_02_b = {}", score)
}

fn is_safe(xs: &[i16]) -> bool {
    let increasing = xs[0] < xs[1];
    let decreasing = xs[0] > xs[1];
    let mut it = xs.iter().peekable();
    while let Some(x) = it.next() {
        if let Some(_next_x) = it.peek() {
            let next_x = *_next_x;
            let same_direction = increasing && next_x > x || decreasing && next_x < x;
            let diff = (x - next_x).abs();
            let adjacent = diff > 0 && diff < 4;
            // println!(
            //     "x: {}, next_x: {}, adjacent: {}, same_direction: {}",
            //     x, next_x, adjacent, same_direction
            // );
            if !same_direction || !adjacent {
                return false;
            }
        }
    }
    true
}

fn parse_input(input: &str) -> Vec<Vec<i16>> {
    input
        .split("\n")
        .filter(|s| !s.is_empty())
        .map(|s| {
            s.split(" ")
                .filter(|s| !s.is_empty())
                .map(|s| s.parse::<i16>().unwrap())
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    static STR: &str = r"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    #[test]
    fn a_test() {
        let xss = parse_input(STR);
        println!("{:?}", &xss);

        let result = day_02_a(&xss);
        println!("{}", &result);
        assert_eq!("day_02_a = 2".to_string(), result);
    }

    #[test]
    fn b_test() {
        let xss = parse_input(STR);

        let result = day_02_b(&xss);
        println!("{}", &result);
        assert_eq!("day_02_b = 4".to_string(), result);
    }
}
