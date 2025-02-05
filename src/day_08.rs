use std::collections::{HashMap, HashSet};
use std::hash::BuildHasherDefault;
use std::time::Instant;

use nohash_hasher::NoHashHasher;

use crate::common::Solution;

#[derive(Clone, Debug)]
struct Grid {
    pub antenna_m: FastMap,
    pub rows: usize,
    pub cols: usize,
}

// hashmap<antenna, (row, col)>
type FastMap = HashMap<char, Vec<(i32, i32)>, BuildHasherDefault<NoHashHasher<usize>>>;

static INPUT: &str = include_str!("../data/day_08.txt");

pub fn solve_day_08() -> Solution {
    let start = Instant::now();
    let grid = parse(INPUT);
    let part_a = day_08_a(&grid); // 396
    let part_b = day_08_b(&grid); // 1196 too low s/b 1200
    let duration = start.elapsed();
    Solution {
        part_a,
        part_b,
        duration,
    }
}

fn day_08_a(grid: &Grid) -> String {
    let mut anti_s: HashSet<(i32, i32)> = HashSet::new();
    for c in grid.antenna_m.keys() {
        grid.find_antis(c, &mut anti_s, false);
    }

    let v = anti_s.len();
    format!("day_08_a = {}", v)
}

fn day_08_b(grid: &Grid) -> String {
    let mut anti_s: HashSet<(i32, i32)> = HashSet::new();
    for c in grid.antenna_m.keys() {
        grid.find_antis(c, &mut anti_s, true);
    }

    let v = anti_s.len();
    format!("day_08_b = {}", v)
}

impl Grid {
    fn find_antis(&self, ch: &char, anti_s: &mut HashSet<(i32, i32)>, many: bool) {
        let rows: i32 = self.rows as i32;
        let cols: i32 = self.cols as i32;

        let is_valid = |(r, c): (i32, i32)| -> bool { r >= 0 && r < rows && c >= 0 && c < cols };

        if self.antenna_m.contains_key(ch) {
            let ps = self.antenna_m.get(ch).unwrap();
            for (r1, c1) in ps {
                for (r2, c2) in ps {
                    if r1 == r2 && c1 == c2 {
                        continue;
                    }

                    let dr = r1 - r2;
                    let dc = c1 - c2;

                    let anti = (r1 + dr, c1 + dc);
                    if is_valid(anti) {
                        anti_s.insert(anti);
                    }

                    if many {
                        // add the source points
                        anti_s.insert((*r1, *c1));
                        anti_s.insert((*r2, *c2));

                        let (mut new_dr, mut new_dc) = (dr, dc);
                        loop {
                            let anti = (r1 + new_dr, c1 + new_dc);
                            if is_valid(anti) {
                                anti_s.insert(anti);
                                new_dr += dr;
                                new_dc += dc;
                            } else {
                                break;
                            }
                        }
                    }
                }
            }
        }
    }
}

fn parse(s: &str) -> Grid {
    let mut antenna_m: FastMap =
        HashMap::with_capacity_and_hasher(100, BuildHasherDefault::default());
    let mut rows = 0;
    let mut cols = 0;

    for (row, line) in s.lines().enumerate() {
        let cs: Vec<char> = line.chars().collect();
        for (col, c) in cs.iter().enumerate() {
            if *c != '.' {
                if antenna_m.contains_key(c) {
                    antenna_m.get_mut(c).unwrap().push((row as i32, col as i32));
                } else {
                    antenna_m.insert(*c, vec![(row as i32, col as i32)]);
                }
            }
            cols = cols.max(col);
        }
        rows = rows.max(row);
    }

    rows += 1;
    cols += 1;

    Grid {
        antenna_m,
        rows,
        cols,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static STR1: &str = r"..........
..........
..........
....a.....
........a.
.....a....
..........
..........
..........
..........";

    static STR2: &str = r"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    static STR3: &str = r"T.........
...T......
.T........
..........
..........
..........
..........
..........
..........
..........";

    #[test]
    fn parse_test() {
        let grid = parse(STR1);
        println!("{:?}", &grid);

        let grid = parse(STR2);
        println!("{:?}", &grid);

        let grid = parse(INPUT);
        println!("{:?}", &grid);
    }

    #[test]
    fn a_test() {
        let grid = parse(STR1);
        let result = day_08_a(&grid);
        println!("{}", &result);
        assert_eq!("day_08_a = 4".to_string(), result);

        let grid = parse(STR2);
        let result = day_08_a(&grid);
        println!("{}", &result);
        assert_eq!("day_08_a = 14".to_string(), result);

        let grid = parse(INPUT);
        let result = day_08_a(&grid);
        println!("{}", &result);
        assert_eq!("day_08_a = 396".to_string(), result);
    }

    #[test]
    fn b_test() {
        let grid = parse(STR3);
        let result = day_08_b(&grid);
        println!("{}", &result);
        assert_eq!("day_08_b = 9".to_string(), result);

        let grid = parse(STR2);
        let result = day_08_b(&grid);
        println!("{}", &result);
        assert_eq!("day_08_b = 34".to_string(), result);

        let grid = parse(INPUT);
        let result = day_08_b(&grid);
        println!("{}", &result);
        assert_eq!("day_08_b = 1200".to_string(), result);
    }
}
