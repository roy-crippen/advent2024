use std::fmt::Debug;
use std::iter::Iterator;
use std::time::Duration;

pub struct Solution {
    pub part_a: String,
    pub part_b: String,
    pub duration: Duration,
}

pub enum Dir {
    N,
    Ne,
    E,
    Se,
    S,
    Sw,
    W,
    Nw,
}

pub fn directions() -> Vec<Dir> {
    Vec::from([
        Dir::N,
        Dir::Ne,
        Dir::E,
        Dir::Se,
        Dir::S,
        Dir::Sw,
        Dir::W,
        Dir::Nw,
    ])
}
pub fn neighbors_8<T: PartialEq + Copy>(css: &[Vec<T>], x: usize, y: usize) -> [(Dir, T); 8] {
    [
        (Dir::N, css[x][y - 1]),
        (Dir::Ne, css[x + 1][y - 1]),
        (Dir::E, css[x + 1][y]),
        (Dir::Se, css[x + 1][y + 1]),
        (Dir::S, css[x][y + 1]),
        (Dir::Sw, css[x - 1][y + 1]),
        (Dir::W, css[x - 1][y]),
        (Dir::Nw, css[x - 1][y - 1]),
    ]
}

pub fn neighbor<T: PartialEq + Copy>(
    css: &[Vec<T>],
    x: usize,
    y: usize,
    dir: &Dir,
) -> (usize, usize, T) {
    match dir {
        Dir::N => (x, y - 1, css[x][y - 1]),
        Dir::Ne => (x + 1, y - 1, css[x + 1][y - 1]),
        Dir::E => (x + 1, y, css[x + 1][y]),
        Dir::Se => (x + 1, y + 1, css[x + 1][y + 1]),
        Dir::S => (x, y + 1, css[x][y + 1]),
        Dir::Sw => (x - 1, y + 1, css[x - 1][y + 1]),
        Dir::W => (x - 1, y, css[x - 1][y]),
        Dir::Nw => (x - 1, y - 1, css[x - 1][y - 1]),
    }
}

pub fn pad_grid<T: Clone + Copy>(css: &mut Vec<Vec<T>>, ch: &T) {
    for cs in css.iter_mut() {
        cs.insert(0, *ch);
        cs.push(*ch);
    }
    let row: Vec<T> = vec![*ch; css[0].len()];
    css.insert(0, row.clone());
    css.push(row);
}

/// Returns k nested 'loops' from xs, ordered.
///
/// `k_nested_recur`(3, &[1,2]) sudo code same as:
///
///  for (i,vi) 0..2 { for (j,vj) in i..2 { for (k,vk) in j..2 { list.push(vi, vj, vk) } } }
///
/// Recursive solution.
///
/// ```
/// use lib::common::k_nested_recur;
///
/// let res = [[1,1,1], [1,1,2], [1,2,1], [1,2,2], [2,1,1], [2,1,2], [2,2,1], [2,2,2]];
/// assert_eq!(k_nested_recur(3, &[1,2]), res);
///
/// let xss = k_nested_recur(8, &["red", "green", "blue", "orange"]);
/// assert_eq!(xss.len(), 65536);
///
/// ```
pub fn k_nested_recur<T>(k: usize, xs: &[T]) -> Vec<Vec<T>>
where
    T: Clone + PartialEq + Ord + Debug,
{
    match k {
        0 => vec![vec![]],
        1 => xs.iter().map(|x| vec![x.clone()]).collect(),
        _ => {
            let mut list: Vec<Vec<T>> = Vec::new();
            for x in xs {
                let ts = k_nested_recur(k - 1, xs);
                for mut t in ts {
                    t.push(x.clone());
                    list.push(t);
                }
            }
            list.sort();
            list
        }
    }
}
