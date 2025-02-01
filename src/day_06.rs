use crate::common::Solution;
use rayon::prelude::*;
use std::cmp::PartialEq;
use std::time::Instant;

#[derive(Clone, Debug)]
struct Grid {
    pub css: Vec<Vec<char>>,
    pub start_row: usize,
    pub start_col: usize,
    pub rows: usize,
    pub cols: usize,
}

#[derive(Clone, Debug)]
struct State {
    cur_row: usize,
    cur_col: usize,
    dir: Dir,
    visits: Vec<(usize, usize)>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
    Done,
}

static INPUT: &str = include_str!("../data/day_06.txt");

pub fn solve_day_06() -> Solution {
    let start = Instant::now();
    let grid = parse(INPUT);
    let (part_a, candidates) = day_06_a(&grid); // 5329
    let part_b = day_06_b(&grid, &candidates); // 2162
    let duration = start.elapsed();
    Solution {
        part_a,
        part_b,
        duration,
    }
}

impl Grid {
    #[allow(dead_code)]
    fn show(&self) {
        for xs in self.css.iter().take(self.rows) {
            for c in xs.iter().take(self.cols) {
                print!("{}", c);
            }
            println!();
        }
        println!(
            "start row: {}, start coll: {}\n",
            self.start_row, self.start_col
        );
    }

    fn walk_a(&self, state: &mut State) {
        match state.dir {
            Dir::Up => loop {
                match self.css[state.cur_row - 1][state.cur_col] {
                    '_' => {
                        state.dir = Dir::Done;
                        break;
                    }
                    '#' => {
                        state.dir = Dir::Right;
                        break;
                    }
                    _ => {
                        state.visits.push((state.cur_row, state.cur_col));
                        state.cur_row -= 1
                    }
                }
            },
            Dir::Down => loop {
                match self.css[state.cur_row + 1][state.cur_col] {
                    '_' => {
                        state.dir = Dir::Done;
                        break;
                    }
                    '#' => {
                        state.dir = Dir::Left;
                        break;
                    }
                    _ => {
                        state.visits.push((state.cur_row, state.cur_col));
                        state.cur_row += 1
                    }
                }
            },
            Dir::Left => loop {
                match self.css[state.cur_row][state.cur_col - 1] {
                    '_' => {
                        state.dir = Dir::Done;
                        break;
                    }
                    '#' => {
                        state.dir = Dir::Up;
                        break;
                    }
                    _ => {
                        state.visits.push((state.cur_row, state.cur_col));
                        state.cur_col -= 1
                    }
                }
            },
            Dir::Right => loop {
                match self.css[state.cur_row][state.cur_col + 1] {
                    '_' => {
                        state.dir = Dir::Done;
                        break;
                    }
                    '#' => {
                        state.dir = Dir::Down;
                        break;
                    }
                    _ => {
                        state.visits.push((state.cur_row, state.cur_col));
                        state.cur_col += 1
                    }
                }
            },
            Dir::Done => (),
        }
    }

    fn walk_b(&self, state: &mut State, obstacle_row: usize, obstacle_col: usize) {
        match state.dir {
            Dir::Up => loop {
                let c = self.css[state.cur_row - 1][state.cur_col];
                if c == '_' {
                    state.dir = Dir::Done;
                    break;
                } else if c == '#'
                    || (state.cur_row - 1 == obstacle_row && state.cur_col == obstacle_col)
                {
                    state.dir = Dir::Right;
                    break;
                } else {
                    state.cur_row -= 1;
                }
            },
            Dir::Down => loop {
                let c = self.css[state.cur_row + 1][state.cur_col];
                if c == '_' {
                    state.dir = Dir::Done;
                    break;
                } else if c == '#'
                    || (state.cur_row + 1 == obstacle_row && state.cur_col == obstacle_col)
                {
                    state.dir = Dir::Left;
                    break;
                } else {
                    state.cur_row += 1;
                }
            },
            Dir::Left => loop {
                let c = self.css[state.cur_row][state.cur_col - 1];
                if c == '_' {
                    state.dir = Dir::Done;
                    break;
                } else if c == '#'
                    || (state.cur_row == obstacle_row && state.cur_col - 1 == obstacle_col)
                {
                    state.dir = Dir::Up;
                    break;
                } else {
                    state.cur_col -= 1
                }
            },
            Dir::Right => loop {
                let c = self.css[state.cur_row][state.cur_col + 1];
                if c == '_' {
                    state.dir = Dir::Done;
                    break;
                } else if c == '#'
                    || (state.cur_row == obstacle_row && state.cur_col + 1 == obstacle_col)
                {
                    state.dir = Dir::Down;
                    break;
                } else {
                    state.cur_col += 1;
                }
            },
            Dir::Done => (),
        }
    }
}

fn day_06_a(grid: &Grid) -> (String, Vec<(usize, usize)>) {
    let mut state = State {
        cur_row: grid.start_row,
        cur_col: grid.start_col,
        dir: Dir::Up,
        visits: Vec::new(),
    };

    while state.dir != Dir::Done {
        grid.walk_a(&mut state);
    }

    // clean up the results
    state.visits.push((state.cur_row, state.cur_col));
    state.visits.sort_unstable();
    state.visits.dedup();

    let s = format!("day_06_a = {}", state.visits.len());
    (s, state.visits)
}

fn day_06_b(grid: &Grid, candidates: &[(usize, usize)]) -> String {
    let state = State {
        cur_row: grid.start_row,
        cur_col: grid.start_col,
        dir: Dir::Up,
        visits: Vec::new(),
    };

    let cnt: usize = candidates
        .par_iter()
        .map(|&(r, c)| {
            let mut tor_state = state.clone();
            let mut hare_state = state.clone();

            loop {
                grid.walk_b(&mut tor_state, r, c);
                grid.walk_b(&mut hare_state, r, c);
                grid.walk_b(&mut hare_state, r, c);
                if hare_state.dir == Dir::Done {
                    return 0;
                } else if tor_state.dir == hare_state.dir
                    && tor_state.cur_row == hare_state.cur_row
                    && tor_state.cur_col == hare_state.cur_col
                {
                    return 1;
                }
            }
        })
        .sum();

    format!("day_06_b = {}", cnt)
}

fn parse(s: &str) -> Grid {
    let mut start_row = 0;
    let mut start_col = 0;

    // fill grid
    let mut xss: Vec<Vec<char>> = Vec::new();
    for (row, line) in s.lines().enumerate() {
        let mut xs: Vec<char> = line.chars().collect();
        // pad col
        xs.insert(0, '_');
        xs.push('_');

        if let Some(col) = xs.iter().position(|&c| c == '^') {
            start_row = row + 1;
            start_col = col;
        }
        xss.push(xs);
    }

    // finish padding the grid
    let cols = xss[0].len();
    let ts: Vec<char> = std::iter::repeat('_').take(cols).collect();
    xss.insert(0, ts.clone());
    xss.push(ts);

    let rows = xss.len();
    Grid {
        css: xss,
        start_row,
        start_col,
        rows,
        cols,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static STR: &str = r"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test]
    fn a_test() {
        let grid = parse(STR);
        grid.show();
        let (result, _) = day_06_a(&grid);
        println!("{}", &result);
        assert_eq!("day_06_a = 41".to_string(), result);
    }

    #[test]
    fn b_test() {
        let grid = parse(STR);
        let (_, visits) = day_06_a(&grid);
        let result = day_06_b(&grid, &visits);
        println!("{}", &result);
        assert_eq!("day_06_b = 6".to_string(), result);
    }
}
