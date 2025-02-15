use std::{collections::HashSet, env};

use itertools::Itertools;
use log::{debug, info, trace};
use solvers::dodeca::{FACETS, PENTAS, TRI_TO_FACETS};

fn main() {
    env_logger::init();

    // find triplets summing to 96 with:
    // triplets_summing_to_n(1,65,96)

    let triplets = [
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
    let unused = [19, 47, 48, 54, 57];

    let args = env::args().collect_vec();
    let seed: u64 = args[1].parse().unwrap();

    // see graph.svg for the pentagons/triangles/facets arrangement

    let pentas = generate_one(&triplets, seed);
    println!("{:?}", pentas);

    // let sols = pentas_on_ico(&pentas);
    // println!("{:?}", sols);

    gui::demo_3d(&pentas);
}

fn generate_inf(triplets: &[(i32, i32, i32); 20], mut seed: u64) {
    let pentas = generate_one(triplets, seed);
    let sols = pentas_on_ico(&pentas);
    println!("{:?}", sols);
    loop {
        if sols.len() != 1 {
            break;
        }
        seed += 1;
    }
}

fn generate_one(triplets: &[(i32, i32, i32); 20], seed: u64) -> [[i32; 5]; 12] {
    use rand::prelude::*;
    // Get an RNG:
    let mut rng = SmallRng::seed_from_u64(seed);
    let mut triangles: [usize; 20] = (0..20).collect_array().unwrap();
    triangles.shuffle(&mut rng);
    let mut facets = [0; 60];
    for tri in 0..20 {
        // biased but soooo negligibly
        let rotation = rng.next_u32() % 3;
        let facet = TRI_TO_FACETS[triangles[tri]];
        facets[facet[rotation as usize]] = triplets[tri].0;
        facets[facet[(rotation as usize + 1) % 3]] = triplets[tri].1;
        facets[facet[(rotation as usize + 2) % 3]] = triplets[tri].2;
    }
    info!("facets:\n{:?}", facets);
    let pentas: [[i32; 5]; 12] = facets
        .into_iter()
        .chunks(5)
        .into_iter()
        .map(|x| x.collect_array().unwrap())
        .collect_array()
        .unwrap();
    info!("PENTAS:\n{:?}", PENTAS);
    info!("pentas:\n{:?}", pentas);
    let mut pentas_shuffled = pentas;
    pentas_shuffled.shuffle(&mut rng);
    pentas_shuffled = pentas_shuffled
        .into_iter()
        .map(|penta| {
            // biased but soooo negligibly
            let shift = rng.next_u32() as usize % 5;
            let mut result = [0; 5];
            for i in 0..5 {
                result[(i + shift) % 5] = penta[i];
            }
            result
        })
        .collect_array()
        .unwrap();
    info!("pentas shuffled:\n{:?}", pentas_shuffled);
    // pentas_shuffled
    pentas
}

// given N>=12 pentas of 5 facets, try to place them on the vertices of an icosahedron.
// 3 touching facets of 3 touching pentas make up a triangle (icosa face).
// the sum of the value of the facets of a triangle must equal 96.
// returns all found valid combinations of pentas configuration
// (penta id, rotation)
fn pentas_on_ico(pentas: &[[i32; 5]]) -> Vec<[(usize, usize); 12]> {
    let mut solutions: Vec<[(usize, usize); 12]> = vec![];

    let mut state = [0; 60];
    let id0 = 0;
    let rot0 = 0;
    let mut stack = vec![(id0, rot0)];

    let remove_penta = |state: &mut [i32; 60], at: usize| {
        debug!(
            "state:\n{:?}",
            TRI_TO_FACETS
                .iter()
                .map(|&[a, b, c]| [state[a], state[b], state[c]])
                .collect_vec()
        );
        debug!("removing penta from {}", at);
        let base = at * 5;
        for r in 0..5 {
            let idx = base + r;
            if state[idx] == 0 {
                // we did something wrong...
                panic!(
                    "state[{}] already empty: {}\nstate:{:?}",
                    idx, state[idx], state
                );
                // return false;
            }
            state[idx] = 0;
        }
    };

    let place_penta = |state: &mut [i32; 60], penta: &[i32; 5], rot: usize, at: usize| {
        debug!(
            "state:\n{:?}",
            TRI_TO_FACETS
                .iter()
                .map(|&[a, b, c]| [state[a], state[b], state[c]])
                .collect_vec()
        );
        debug!("placing penta {:?}, with rot #{}, at {}", penta, rot, at);
        let base = at * 5;
        // perform checks...
        for (i, val) in penta.iter().enumerate() {
            let ri = (i + rot) % 5;
            let idx = base + ri;
            if state[idx] != 0 {
                // we did something wrong...
                panic!(
                    "cant place penta {:?} (ids: {:?}) with rot {}, state[{}] already filled: {}\nstate:{:?}",
                    penta, PENTAS[at], rot, idx, state[idx], state
                );
            }
            let triangle = FACETS[idx].1;
            let facets = TRI_TO_FACETS[triangle];
            let mut sum = 0;
            let mut full_triangle = true;
            for facet in facets {
                if facet != idx && state[facet] == 0 {
                    full_triangle = false;
                }
                sum += state[facet];
            }
            if full_triangle && sum + val != 96 {
                trace!(
                    "nope: too big; val {}, penta {:?}, tri #{}; local state: {} -> {}, {} -> {}, {} -> {}",
                    val,
                    penta,
                    triangle,
                    facets[0],
                    state[facets[0]],
                    facets[1],
                    state[facets[1]],
                    facets[2],
                    state[facets[2]]
                );
                debug!("nope");
                return false;
            }
        }
        // setting values
        for (i, v) in penta.iter().enumerate() {
            let ri = (i + rot) % 5;
            let idx = base + ri;
            state[idx] = *v;
        }
        debug!("placed");
        debug!(
            "new state:\n{:?}",
            TRI_TO_FACETS
                .iter()
                .map(|&[a, b, c]| [state[a], state[b], state[c]])
                .collect_vec()
        );
        true
    };
    place_penta(&mut state, &pentas[id0], rot0, 0);

    let mut used = HashSet::from([id0]);
    let mut count = 0;
    'outer: loop {
        debug!("stack:\n{:?}", stack);
        let d = stack.len();
        for (i, penta) in pentas.iter().enumerate() {
            if !used.contains(&i) {
                for r in 0..5 {
                    debug!("deeper?");
                    if place_penta(&mut state, penta, r, d) {
                        debug!("deeper!");
                        // deeper
                        used.insert(i);
                        // rem.remove(&i);
                        stack.push((i, r));
                        continue 'outer;
                    }
                }
            }
        }
        let d = stack.len();
        if d == 12 {
            // win
            count += 1;
            solutions.push(stack.clone().into_iter().collect_array().unwrap());
            info!("win, new count: {}, stack: {:?}", count, stack);
            debug!(
                "state:\n{:?}",
                TRI_TO_FACETS
                    .iter()
                    .map(|&[a, b, c]| [state[a], state[b], state[c]])
                    .collect_vec()
            );
        }

        // backtrack
        debug!("backtracking?");
        while let Some((prev_i, prev_rot)) = stack.pop() {
            debug!("backtracking!");
            let d = stack.len();
            if d == 0 {
                // we've backtracked to the 0 state
                // all further states are rotational equivalent
                // to what we have explored already -> exit
                break 'outer;
            }

            used.remove(&prev_i);
            remove_penta(&mut state, d);

            // rem.insert(prev_i);
            for r in prev_rot + 1..5 {
                if place_penta(&mut state, &pentas[prev_i], r, d) {
                    // sideway, same penta but different rot
                    used.insert(prev_i);
                    // rem.remove(&prev_i);
                    stack.push((prev_i, r));
                    continue 'outer;
                }
            }
            for (i, penta) in pentas.iter().enumerate().skip(prev_i + 1) {
                if !used.contains(&i) {
                    for r in 0..5 {
                        if place_penta(&mut state, penta, r, d) {
                            // sideway, same penta but different rot
                            used.insert(i);
                            // rem.remove(&prev_i);
                            stack.push((i, r));
                            continue 'outer;
                        }
                    }
                }
            }
        }
    }
    info!("sol count: {}", count);
    solutions
}

fn triplets_summing_to_n(min: u16, max: u16, n: u16) -> Vec<Vec<(u16, u16, u16)>> {
    let mut solutions: Vec<(u16, u16, u16)> = vec![];
    for i in min..=max {
        for j in i + 1..=max {
            for k in j + 1..=max {
                if i + j + k == n {
                    // debug!("{} + {} + {} = {}", i, j, k, n);
                    solutions.push((i, j, k));
                }
            }
        }
    }
    let total = solutions.len();
    info!("sols: {} - {:?}", total, solutions);

    let sol0 = solutions[256];
    let mut stack = vec![(sol0, 1)];
    let mut used = HashSet::from([sol0.0, sol0.1, sol0.2]);
    let mut count = 0;
    let mut max_depth = 0;
    let mut max_depth_count = 0;
    'outer: while !stack.is_empty() {
        let (e, offset) = stack.last().unwrap();
        let d = stack.len();
        if d == 1 {
            count += 1;
            info!("new root {:?}, {}/{}", e, count, total);
        }
        for (ooffset, (i, j, k)) in solutions.iter().enumerate().skip(*offset) {
            if !used.contains(i) && !used.contains(j) && !used.contains(k) {
                // deeper
                used.insert(*i);
                used.insert(*j);
                used.insert(*k);
                stack.push(((*i, *j, *k), ooffset + 1));
                // let d = stack.len();
                // debug!("deeper {:?} - {}", (i, j, k), d);
                continue 'outer;
            }
        }
        // // TODO deadend, how deep are we?
        let d = stack.len();
        trace!(
            "depth: {} - {:?}",
            d,
            stack.iter().map(|p| p.0).collect_vec()
        );
        if d > max_depth {
            max_depth_count = 1;
            info!(
                "deadend, depth: {}\n{:?}",
                d,
                stack.iter().map(|p| p.0).collect_vec()
            );
            max_depth = d;
        } else if d == max_depth {
            max_depth_count += 1;
            if max_depth_count % 1000 == 0 {
                let triplets = stack.iter().map(|p| p.0).collect_vec();
                let mut anti_set: HashSet<u16> = HashSet::from_iter(min..=max);
                for t in &triplets {
                    anti_set.remove(&t.0);
                    anti_set.remove(&t.1);
                    anti_set.remove(&t.2);
                }
                info!(
                    "yikes {}\n{:?}\n{:?}",
                    max_depth_count,
                    triplets,
                    anti_set.into_iter().sorted().collect_vec()
                );
            }
        }

        // backtrack
        while let Some(((i, j, k), offset)) = stack.pop() {
            // let d = stack.len();
            // debug!("backtracking - {}", d);
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
                    // let d = stack.len();
                    // debug!("sideway {:?} - {}", (i, j, k), d);
                    continue 'outer;
                }
            }
        }
    }
    debug!("max depth {}, count: {}", max_depth, max_depth_count);
    vec![]
}
