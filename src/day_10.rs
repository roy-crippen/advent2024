use crate::common::Solution;
use std::time::Instant;

pub fn solve_day_10() -> Solution {
    let start = Instant::now();
    let part_a = day_10_a();
    let part_b = day_10_b();
    let duration = start.elapsed();
    Solution {
        part_a,
        part_b,
        duration,
    }
}

fn day_10_a() -> String {
    let v = 1234;
    format!("day_10_a = {}", v)
}

fn day_10_b() -> String {
    let v = 1234;
    format!("day_10_b = {}", v)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_test() {
        let result = day_10_a();
        println!("{}", &result);
        assert_eq!("day_10_a = 1234".to_string(), result);
    }

    #[test]
    fn b_test() {
        let result = day_10_b();
        println!("{}", &result);
        assert_eq!("day_10_b = 1234".to_string(), result);
    }
}
