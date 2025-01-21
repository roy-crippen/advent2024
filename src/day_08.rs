use crate::common::Solution;
use std::time::Instant;

pub fn solve_day_08() -> Solution {
    let start = Instant::now();
    let part_a = day_08_a();
    let part_b = day_08_b();
    let duration = start.elapsed();
    Solution {
        part_a,
        part_b,
        duration,
    }
}

fn day_08_a() -> String {
    let v = 1234;
    format!("day_08_a = {}", v)
}

fn day_08_b() -> String {
    let v = 1234;
    format!("day_08_b = {}", v)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_test() {
        let result = day_08_a();
        println!("{}", &result);
        assert_eq!("day_08_a = 1234".to_string(), result);
    }

    #[test]
    fn b_test() {
        let result = day_08_b();
        println!("{}", &result);
        assert_eq!("day_08_b = 1234".to_string(), result);
    }
}
