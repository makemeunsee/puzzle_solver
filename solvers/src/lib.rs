use std::collections::HashSet;

use itertools::Itertools;
use log::{debug, trace};
use rand::{rngs::SmallRng, seq::SliceRandom};

pub mod dodeca;

// Finds triplets of unique numbers in the range [min, max] summing to `n`.
// Looks for a set of `len` such triplets.
// Returns the first matching set.
// Shuffling is used to ensure 2 calls with the same parameters but the `rng`
// give very different results.
pub fn triplets_summing_to_n(
    min: i32,
    max: i32,
    n: i32,
    len: usize,
    rng: &mut SmallRng,
) -> Option<Vec<(i32, i32, i32)>> {
    let mut solutions: Vec<(i32, i32, i32)> = vec![];
    for i in min..=max {
        for j in i + 1..=max {
            for k in j + 1..=max {
                if i + j + k == n {
                    solutions.push((i, j, k));
                }
            }
        }
    }
    let total = solutions.len();
    solutions.shuffle(rng);
    debug!("sols: {} - {:?}", total, solutions);

    let sol0 = solutions[0];
    let mut stack = vec![(sol0, 1)];
    let mut used = HashSet::from([sol0.0, sol0.1, sol0.2]);
    let mut count = 0;
    let mut max_depth = 0;
    'outer: while !stack.is_empty() {
        let (e, offset) = stack.last().unwrap();
        let d = stack.len();
        if d == 1 {
            count += 1;
            debug!("new root {:?}, {}/{}", e, count, total);
        }
        for (ooffset, (i, j, k)) in solutions.iter().enumerate().skip(*offset) {
            if !used.contains(i) && !used.contains(j) && !used.contains(k) {
                // deeper
                used.insert(*i);
                used.insert(*j);
                used.insert(*k);
                stack.push(((*i, *j, *k), ooffset + 1));
                continue 'outer;
            }
        }
        let d = stack.len();
        trace!(
            "depth: {} - {:?}",
            d,
            stack.iter().map(|p| p.0).collect_vec()
        );
        if d > max_depth {
            let result = stack.iter().map(|p| p.0).collect_vec();
            debug!("new depth: {}\n{:?}", d, result,);
            max_depth = d;
            if max_depth >= len {
                return Some(result.into_iter().take(len).collect_vec());
            }
        }

        // backtrack
        while let Some(((i, j, k), offset)) = stack.pop() {
            used.remove(&i);
            used.remove(&j);
            used.remove(&k);
            for (ooffset, (i, j, k)) in solutions.iter().enumerate().skip(offset) {
                if !used.contains(i) && !used.contains(j) && !used.contains(k) {
                    // sideway
                    used.insert(*i);
                    used.insert(*j);
                    used.insert(*k);
                    stack.push(((*i, *j, *k), ooffset + 1));
                    continue 'outer;
                }
            }
        }
    }
    None
}

// found with:
// triplets_summing_to_n(1,65,96)
pub const TRIPLETS_96: [(i32, i32, i32); 20] = [
    (13, 28, 55),
    (1, 31, 64),
    (2, 29, 65),
    (3, 30, 63),
    (4, 32, 60),
    (5, 33, 58),
    (6, 34, 56),
    (7, 27, 62),
    (8, 37, 51),
    (9, 26, 61),
    (10, 40, 46),
    (11, 41, 44),
    (12, 35, 49),
    (14, 39, 43),
    (15, 36, 45),
    (16, 38, 42),
    (17, 20, 59),
    (18, 25, 53),
    (21, 23, 52),
    (22, 24, 50),
];

pub const UNUSED_96: [i32; 5] = [19, 47, 48, 54, 57];

pub const TRIPLETS_99_A: [(i32, i32, i32); 20] = [
    (14, 38, 47),
    (25, 35, 39),
    (18, 27, 54),
    (28, 29, 42),
    (2, 41, 56),
    (15, 21, 63),
    (16, 32, 51),
    (7, 44, 48),
    (1, 37, 61),
    (17, 24, 58),
    (8, 34, 57),
    (19, 20, 60),
    (11, 36, 52),
    (6, 31, 62),
    (4, 40, 55),
    (5, 45, 49),
    (3, 46, 50),
    (12, 22, 65),
    (9, 26, 64),
    (10, 30, 59),
];
pub const UNUSED_99_A: [i32; 5] = [13, 23, 33, 43, 53];

pub const TRIPLETS_99_B: [(i32, i32, i32); 20] = [
    (17, 39, 43),
    (8, 44, 47),
    (3, 40, 56),
    (10, 34, 55),
    (11, 27, 61),
    (13, 24, 62),
    (2, 38, 59),
    (12, 22, 65),
    (16, 29, 54),
    (23, 25, 51),
    (15, 26, 58),
    (30, 32, 37),
    (4, 42, 53),
    (19, 20, 60),
    (18, 31, 50),
    (14, 36, 49),
    (1, 35, 63),
    (7, 28, 64),
    (6, 41, 52),
    (5, 46, 48),
];
pub const UNUSED_99_B: [i32; 5] = [9, 21, 33, 45, 57];
