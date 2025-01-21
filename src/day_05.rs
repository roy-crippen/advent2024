use crate::common::Solution;
use std::cmp::Ordering;
use std::collections::{hash_map, HashMap};
use std::time::Instant;

static INPUT: &str = include_str!("../data/day_05.txt");

#[derive(Debug, Clone, Eq)]
struct Page {
    num: u32,
    successors: Vec<u32>,
    predecessors: Vec<u32>,
}

impl Ord for Page {
    fn cmp(&self, other: &Self) -> Ordering {
        other.num.cmp(&self.num)
    }
}

#[allow(clippy::non_canonical_partial_ord_impl)]
impl PartialOrd for Page {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(if self.successors.contains(&other.num) {
            Ordering::Greater
        } else if self.predecessors.contains(&other.num) {
            Ordering::Less
        } else {
            Ordering::Equal
        })
    }
}

impl PartialEq for Page {
    fn eq(&self, other: &Self) -> bool {
        self.num == other.num
    }
}

pub fn solve_day_05() -> Solution {
    let start = Instant::now();
    let xss = parse_input(INPUT);
    let part_a = day_05_a(&xss); // 6034
    let part_b = day_05_b(&xss); // 6305
    let duration = start.elapsed();
    Solution {
        part_a,
        part_b,
        duration,
    }
}

fn day_05_a(xss: &[Vec<Page>]) -> String {
    let mut tot = 0;
    for xs in xss {
        let mut copy = xs.clone();
        copy.sort();
        if *xs == copy {
            tot += xs[xs.len() / 2].num;
        }
    }

    format!("day_05_a = {}", tot)
}

fn day_05_b(xss: &[Vec<Page>]) -> String {
    let mut tot = 0;
    for xs in xss {
        let mut copy = xs.clone();
        copy.sort();
        if *xs != copy {
            tot += copy[copy.len() / 2].num;
        }
    }
    format!("day_05_b = {}", tot)
}

fn parse_input(input: &str) -> Vec<Vec<Page>> {
    let mut it = input.lines();
    let mut m: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut all_nums: Vec<u32> = vec![];
    // create a hashmap of rules
    loop {
        let line = it.next().unwrap();
        if line.is_empty() {
            break;
        }
        let ss: Vec<&str> = line.split("|").collect();
        let k = ss[0].parse::<u32>().unwrap();
        let v = ss[1].parse::<u32>().unwrap();
        all_nums.push(k);
        all_nums.push(v);
        if let hash_map::Entry::Vacant(e) = m.entry(k) {
            e.insert(vec![v]);
        } else {
            m.get_mut(&k).unwrap().push(v);
        }
    }

    // find and add the smallest value to the map
    all_nums.dedup();
    for k in all_nums.iter() {
        if !m.contains_key(k) {
            m.insert(*k, vec![]);
        }
    }

    // create the Page map
    let mut page_m: HashMap<u32, Page> = m
        .iter()
        .map(|(k, vs)| {
            (
                *k,
                Page {
                    num: *k,
                    successors: vec![],
                    predecessors: vs.clone(),
                },
            )
        })
        .collect();

    // fill predecessors
    for k in m.keys() {
        let page = page_m.get(k).unwrap().clone();
        for v in page.predecessors.iter() {
            let page_v = page_m.get_mut(v).unwrap();
            page_v.successors.push(page.num);
        }
    }

    // get puzzle data
    let xss = it
        .map(|s| {
            s.split(",")
                .map(|x| {
                    let k = x.parse::<u32>().unwrap();
                    page_m.get(&k).unwrap().clone()
                })
                .collect::<Vec<Page>>()
        })
        .collect::<Vec<Vec<Page>>>();

    xss
}

#[cfg(test)]
mod tests {
    use super::*;

    static STR: &str = r"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[test]
    fn a_test() {
        let xss = parse_input(STR);

        let result = day_05_a(&xss);
        println!("{}", &result);
        assert_eq!("day_05_a = 143".to_string(), result);
    }

    #[test]
    fn b_test() {
        let xss = parse_input(STR);
        let result = day_05_b(&xss);
        println!("{}", &result);
        assert_eq!("day_05_b = 123".to_string(), result);
    }
}
