use std::collections::HashSet;

use itertools::Itertools;
use log::{debug, info, trace};
use solvers::{
    dodeca::{triangles_to_pentas_shuffled, FACETS, PENTAS, TRI_TO_FACETS},
    triplets_summing_to_n, TRIPLETS_99_A, UNUSED_99_A,
};

fn main() {
    env_logger::init();

    // let args = env::args().collect_vec();
    // let seed: u64 = args[1].parse().unwrap();

    // see graph.svg for the pentagons/triangles/facets arrangement

    // generate_inf(1234567890);

    let triplets = TRIPLETS_99_A;
    let unused = UNUSED_99_A;

    gui::demo_3d(&triplets, &unused);
}

fn generate_inf(mut seed: u64) {
    use rand::prelude::*;
    let mut rng = SmallRng::seed_from_u64(seed);
    loop {
        const START: i32 = 1;
        const END: i32 = 65;
        const SUM: i32 = 99;
        const LEN: usize = 20;
        let triplets: [(i32, i32, i32); LEN] =
            triplets_summing_to_n(START, END, SUM, LEN, &mut rng)
                .unwrap()
                .into_iter()
                .collect_array()
                .unwrap();
        let unused = triplets.iter().fold(
            HashSet::from_iter(START..=END),
            |mut acc: HashSet<i32>, (a, b, c)| {
                acc.remove(a);
                acc.remove(b);
                acc.remove(c);
                acc
            },
        );
        println!("triplets: {:?}\nunused: {:?}", triplets, unused);
        let pentas = triangles_to_pentas_shuffled(&triplets, &mut rng, true, true);
        println!("pentas: {:?}", pentas);
        let sols = pentas_on_ico(&pentas, SUM);
        if sols.len() != 1 {
            break;
        }
        println!("OK");
        seed += 1;
    }
}

// given N>=12 pentas of 5 facets, try to place them on the vertices of an icosahedron.
// 3 touching facets of 3 touching pentas make up a triangle (icosa face).
// the sum of the value of the facets of a triangle must equal `n`.
// returns all found valid combinations of pentas configuration
// (penta id, rotation)
fn pentas_on_ico(pentas: &[[i32; 5]], n: i32) -> Vec<[(usize, usize); 12]> {
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
            if full_triangle && sum + val != n {
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

    let mut move_count = 0;

    let mut used = HashSet::from([id0]);
    let mut count = 0;
    'outer: loop {
        trace!("stack:\n{:?}", stack);
        let d = stack.len();
        for (i, penta) in pentas.iter().enumerate() {
            if !used.contains(&i) {
                for r in 0..5 {
                    trace!("deeper?");
                    if place_penta(&mut state, penta, r, d) {
                        move_count += 1;
                        trace!("deeper!");
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
            if count == 0 {
                info!("move count: {}", move_count);
            }
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
        trace!("backtracking?");
        while let Some((prev_i, prev_rot)) = stack.pop() {
            move_count += 1;
            trace!("backtracking!");
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
                    move_count += 1;
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
                            move_count += 1;
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
    solutions
}
