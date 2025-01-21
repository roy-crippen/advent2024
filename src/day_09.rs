use crate::common::Solution;
use std::collections::HashMap;
use std::time::Instant;

static INPUT: &str = include_str!("../data/day_09.txt");

pub fn solve_day_09() -> Solution {
    let start = Instant::now();
    let xs = make_block(INPUT);
    let part_a = day_09_a(&xs); // 6310675819476
    let part_b = day_09_b(&xs); // 6335972980679
    let duration = start.elapsed();
    Solution {
        part_a,
        part_b,
        duration,
    }
}

fn make_block(s: &str) -> Vec<usize> {
    let mut xs = Vec::new();
    let mut it_free = s.chars().skip(1).step_by(2);
    for (id, file) in s.chars().step_by(2).enumerate() {
        // write file ids
        for _ in 0..file.to_digit(10).unwrap() {
            xs.push(id);
        }

        // write free spots
        if let Some(free) = it_free.next() {
            for _ in 0..free.to_digit(10).unwrap() {
                xs.push(1_000_001usize + id);
            }
        }
    }
    // println!("{:?}", &xs);
    xs
}

fn split_block(xs: &[usize]) -> (HashMap<usize, (usize, usize)>, Vec<(usize, usize)>) {
    // let start = Instant::now();
    let mut m: HashMap<usize, (usize, usize)> = HashMap::new();
    let mut it = xs.iter().enumerate().peekable();
    let mut file_len = 0;
    while let Some((idx, x)) = it.next() {
        file_len += 1;
        if let Some((_, x_next)) = it.peek() {
            if *x != **x_next {
                m.insert(*x, (idx + 1 - file_len, file_len));
                file_len = 0;
            }
        } else {
            // last entry
            m.insert(*x, (idx + 1 - file_len, file_len));
        };
    }

    let mut gaps: Vec<(usize, usize)> = m
        .iter()
        .filter(|(k, _)| **k > 1_000_000)
        .map(|(_, v)| *v)
        .collect();
    gaps.sort();

    m = m
        .iter()
        .filter(|(k, _)| **k < 1_000_000)
        .map(|(k, v)| (*k, *v))
        .collect();

    // println!("split_block duration = {:?}", start.elapsed());
    (m, gaps)
}

fn calc_checksum(xs: &[usize]) -> usize {
    xs.iter().enumerate().fold(0usize, |acc, (idx, x)| {
        if *x > 1_000_000usize {
            acc
        } else {
            acc + idx * *x
        }
    })
}

fn find_next_free_idx(
    gaps: &mut Vec<(usize, usize)>,
    file_idx: usize,
    file_len: usize,
) -> Option<usize> {
    for (i, (gap_idx, gap_len)) in gaps.iter_mut().enumerate() {
        if *gap_idx >= file_idx {
            return None;
        }
        if *gap_len >= file_len {
            let v = *gap_idx;
            // update the gaps slice
            *gap_idx += file_len;
            *gap_len -= file_len;

            // remove empty gaps
            if *gap_len == 0 {
                gaps.remove(i);
            }

            return Some(v);
        }
    }
    None
}

fn day_09_a(_xs: &[usize]) -> String {
    let mut it_l = _xs.iter().enumerate();
    let mut it_r = _xs.iter().enumerate().rev();
    let mut xs = _xs.to_vec();
    loop {
        let (i, cl) = it_l.next().unwrap();
        if *cl > 1_000_000usize {
            for (j, cr) in it_r.by_ref() {
                if i >= j {
                    // println!("{:?}", &xs);
                    let v = calc_checksum(&xs);
                    return format!("day_09_a = {}", v);
                }
                if *cr < 1_000_000usize {
                    xs.swap(i, j);
                    break;
                }
            }
        }
    }
}

fn day_09_b(_xs: &[usize]) -> String {
    let (file_m, mut gaps) = split_block(_xs);
    let mut xs = _xs.to_vec();
    let mut keys: Vec<usize> = file_m.keys().copied().collect();
    keys.sort();

    for file_id in keys.iter().rev() {
        let (file_idx, file_len) = *file_m.get(file_id).unwrap();
        if let Some(gap_idx) = find_next_free_idx(&mut gaps, file_idx, file_len) {
            for n in 0..file_len {
                xs.swap(file_idx + n, gap_idx + n);
            }
            // println!("{:?}", &xs);
        }
        // println!("file_id = {}, gaps = {:?}", file_id, &gaps);
        // println!("{:?}", &xs);
    }

    let v = calc_checksum(&xs);
    format!("day_09_b = {}", v)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn make_block_test() {
        let s = "2333133121414131402";
        let xs = make_block(s);
        println!("{:?}", &xs);
        let cs_expected = vec![
            0, 0, 1_000_001, 1_000_001, 1_000_001, 1, 1, 1, 1_000_002, 1_000_002, 1_000_002, 2,
            1_000_003, 1_000_003, 1_000_003, 3, 3, 3, 1_000_004, 4, 4, 1_000_005, 5, 5, 5, 5,
            1_000_006, 6, 6, 6, 6, 1_000_007, 7, 7, 7, 1_000_008, 8, 8, 8, 8, 9, 9,
        ];
        assert_eq!(cs_expected, xs);
    }

    #[test]
    fn a_test() {
        let s = "2333133121414131402";
        let xs = make_block(s);
        let result = day_09_a(&xs);
        println!("{}", &result);
        assert_eq!("day_09_a = 1928".to_string(), result);
    }

    #[test]
    fn split_block_test() {
        let s = "2333133121414131402";
        let xs = make_block(s);
        let (m, gaps) = split_block(&xs);
        println!("{:?}", &m);
        println!("{:?}", &xs);
        assert!(m.contains_key(&0));
        assert_eq!(Some(&(0, 2)), m.get(&0));
        assert!(m.contains_key(&8));
        assert_eq!(Some(&(36, 4)), m.get(&8));
        assert!(gaps.contains(&(8, 3)));
    }

    #[test]
    fn b_test() {
        let s = "2333133121414131402";
        let xs = make_block(s);
        let result = day_09_b(&xs);
        println!("{}", &result);
        assert_eq!("day_09_b = 2858".to_string(), result);
    }
}
