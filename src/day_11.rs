use std::time::Instant;

use crate::common::Solution;

pub fn solve_day_11() -> Solution {
    let start = Instant::now();
    let part_a = day_11_a();
    let part_b = day_11_b();
    let duration = start.elapsed();
    Solution {
        part_a,
        part_b,
        duration,
    }
}

fn day_11_a() -> String {
    let v = 1234;
    format!("day_11_a = {}", v)
}

fn day_11_b() -> String {
    let v = 1234;
    format!("day_11_b = {}", v)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_test() {
        let result = day_11_a();
        println!("{}", &result);
        assert_eq!("day_11_a = 1234".to_string(), result);
    }

    #[test]
    fn b_test() {
        let result = day_11_b();
        println!("{}", &result);
        assert_eq!("day_11_b = 1234".to_string(), result);
    }
}
