use crate::common;
use crate::common::{directions, neighbors_8, pad_grid, Dir, Solution};
use std::time::Instant;

static INPUT: &str = include_str!("../data/day_04.txt");

pub fn solve_day_04() -> Solution {
    let start = Instant::now();
    let mut css = parse_input(INPUT);
    pad_grid(&mut css, &'.');
    let part_a = day_04_a(&css); // 2573
    let part_b = day_04_b(&css); // 1850
    let duration = start.elapsed();
    Solution {
        part_a,
        part_b,
        duration,
    }
}

fn day_04_a(css: &[Vec<char>]) -> String {
    let x_indexes = get_ch_indexes(css, 'X');
    let mut acc = 0;
    for (x, y) in x_indexes {
        for dir in directions() {
            acc += find_xmas(css, x, y, 'X', &dir);
        }
    }
    format!("day_04_a = {}", acc)
}

fn day_04_b(css: &[Vec<char>]) -> String {
    let v = get_ch_indexes(css, 'A')
        .iter()
        .fold(0, |acc, (x, y)| acc + find_xmas_b(css, *x, *y));

    format!("day_04_b = {}", v)
}

fn parse_input(s: &str) -> Vec<Vec<char>> {
    s.lines().map(|x| x.chars().collect()).collect()
}

fn get_ch_indexes(css: &[Vec<char>], ch: char) -> Vec<(usize, usize)> {
    css.iter()
        .enumerate()
        .flat_map(|(i, cs)| {
            cs.iter()
                .enumerate()
                .map(move |(j, c)| if *c == ch { Some((i, j)) } else { None })
        })
        .flatten()
        .collect::<Vec<(usize, usize)>>()
}

pub fn find_xmas(css: &[Vec<char>], x: usize, y: usize, current_char: char, dir: &Dir) -> usize {
    let (next_x, next_y, next_char) = common::neighbor(css, x, y, dir);
    match next_char {
        'M' if { current_char == 'X' } => find_xmas(css, next_x, next_y, 'M', dir),
        'A' if { current_char == 'M' } => find_xmas(css, next_x, next_y, 'A', dir),
        'S' if { current_char == 'A' } => 1,
        _ => 0,
    }
}

fn find_xmas_b(css: &[Vec<char>], x: usize, y: usize) -> usize {
    let [(_, _), (_, ne), (_, _), (_, se), (_, _), (_, sw), (_, _), (_, nw)] =
        neighbors_8(css, x, y);
    if ((ne == 'M' && sw == 'S') || (ne == 'S' && sw == 'M'))
        && ((nw == 'M' && se == 'S') || (nw == 'S' && se == 'M'))
    {
        1
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static STR: &str = r"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    #[test]
    fn a_test() {
        let mut css = parse_input(STR);
        pad_grid(&mut css, &'.');
        let result = day_04_a(&css);
        println!("{}", &result);
        assert_eq!("day_04_a = 18".to_string(), result);
    }

    #[test]
    fn b_test() {
        let mut css = parse_input(STR);
        pad_grid(&mut css, &'.');
        let result = day_04_b(&css);
        println!("{}", &result);
        assert_eq!("day_04_b = 9".to_string(), result);
    }
}
