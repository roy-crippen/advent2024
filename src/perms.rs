use std::iter::Iterator;

pub struct Perms<T: Copy> {
    k: usize,            // length of each permutation
    xs: Vec<T>,          // elements to choose from
    current: Vec<usize>, // keep track of the current combination indices
    done: bool,          // flag to indicate if we're done iterating
}

impl<T: Copy> Perms<T> {
    pub fn new(k: usize, xs: Vec<T>) -> Self {
        Perms {
            k,
            xs,
            current: vec![0; k],
            done: false,
        }
    }
}

impl<T: Copy> Iterator for Perms<T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.xs.is_empty() || self.k == 0 || self.done {
            return None;
        }

        // Return the current combination
        let result = self.current.iter().map(|&idx| self.xs[idx]).collect();

        // Check if we've reached the last combination
        let mut all_max = true;
        for &idx in &self.current {
            if idx < self.xs.len() - 1 {
                all_max = false;
                break;
            }
        }

        if all_max {
            self.done = true;
            return Some(result);
        }

        // Increment the combination
        let mut i = self.k;
        loop {
            if i == 0 {
                // This should never happen if we've checked all_max, but just in case:
                self.done = true;
                return Some(result);
            }
            i -= 1;
            self.current[i] += 1;
            if self.current[i] < self.xs.len() {
                // Reset all indices to the right of i to 0
                for j in (i + 1)..self.k {
                    self.current[j] = 0;
                }
                return Some(result);
            }
        }
    }
}

// // Example usage:
// fn main() {
//     let perms1 = Perms::new(3, vec![1, 2]);
//     println!("Permutations of [1, 2] with length 3:");
//     for perm in perms1 {
//         println!("{:?}", perm);
//     }
//
//     let perms2 = Perms::new(2, vec!['a', 'b']);
//     println!("Permutations of ['a', 'b'] with length 2:");
//     for perm in perms2 {
//         println!("{:?}", perm);
//     }
// }
