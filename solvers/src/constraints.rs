use crate::common::{Dir, Face, AREA_L, AREA_M, AREA_S, BLOCKS, DEPTH, HEIGHT, WIDTH};
use itertools::Itertools;
use log::{debug, info, trace};
use std::collections::{HashMap, HashSet};

pub fn solve(target: u8) {
    let all_faces = [
        BLOCKS[0].faces.clone(),
        BLOCKS[1].faces.clone(),
        BLOCKS[2].faces.clone(),
        BLOCKS[3].faces.clone(),
        BLOCKS[4].faces.clone(),
        BLOCKS[5].faces.clone(),
        BLOCKS[6].faces.clone(),
        BLOCKS[7].faces.clone(),
        BLOCKS[8].faces.clone(),
    ]
    .concat();
    let sols_small = combinations_to_n(&all_faces, WIDTH, DEPTH, target);
    let sols_medium = combinations_to_n(&all_faces, HEIGHT, DEPTH, target);
    let sols_large = combinations_to_n(&all_faces, HEIGHT, WIDTH, target);

    let pairs_small: Vec<(Vec<Face>, Vec<Face>)> = match_combos_in_pairs(sols_small, WIDTH, DEPTH);
    let pairs_medium: Vec<(Vec<Face>, Vec<Face>)> =
        match_combos_in_pairs(sols_medium, HEIGHT, DEPTH);
    let pairs_large: Vec<(Vec<Face>, Vec<Face>)> = match_combos_in_pairs(sols_large, HEIGHT, WIDTH);

    let pairs_s_count = pairs_small.len();
    let pairs_m_count = pairs_medium.len();
    let pairs_l_count = pairs_large.len();
    let total = pairs_l_count * pairs_m_count * pairs_s_count;
    debug!("6-combos to check: {}", total);

    for pair_s in &pairs_small {
        let set_s_0: HashSet<Face> = pair_s.0.clone().into_iter().collect();
        let set_s_1: HashSet<Face> = pair_s.1.clone().into_iter().collect();
        for pair_m in &pairs_medium {
            let set_m_0: HashSet<Face> = pair_m.0.clone().into_iter().collect();
            let set_m_1: HashSet<Face> = pair_m.1.clone().into_iter().collect();
            if set_s_0.is_disjoint(&set_m_0)
                && set_s_0.is_disjoint(&set_m_1)
                && set_s_1.is_disjoint(&set_m_0)
                && set_s_1.is_disjoint(&set_m_1)
            {
                for pair_l in &pairs_large {
                    let set_l_0: HashSet<Face> = pair_l.0.clone().into_iter().collect();
                    let set_l_1: HashSet<Face> = pair_l.1.clone().into_iter().collect();
                    if set_s_0.is_disjoint(&set_l_0)
                        && set_s_0.is_disjoint(&set_l_1)
                        && set_s_1.is_disjoint(&set_l_0)
                        && set_s_1.is_disjoint(&set_l_1)
                        && set_m_0.is_disjoint(&set_l_0)
                        && set_m_0.is_disjoint(&set_l_1)
                        && set_m_1.is_disjoint(&set_l_0)
                        && set_m_1.is_disjoint(&set_l_1)
                        && compatible_six_combo(pair_s, pair_m, pair_l)
                    {
                        info!(
                            "sol:\n{:?} {:?} {:?} {:?} {:?} {:?}",
                            pair_s.0.iter().map(|f| f.value).collect_vec(),
                            pair_s.1.iter().map(|f| f.value).collect_vec(),
                            pair_m.0.iter().map(|f| f.value).collect_vec(),
                            pair_m.1.iter().map(|f| f.value).collect_vec(),
                            pair_l.0.iter().map(|f| f.value).collect_vec(),
                            pair_l.1.iter().map(|f| f.value).collect_vec()
                        );
                    }
                }
            }
        }
    }
}

fn combinations_to_n(faces: &[Face], long: u8, short: u8, n: u8) -> Vec<Vec<Face>> {
    let area = long * short;
    let mut solutions: Vec<Vec<Face>> = vec![];
    // first candidate = no face selected, all faces selectable
    let mut candidates: Vec<(Vec<Face>, Vec<Face>)> = vec![(vec![], faces.to_vec())];
    // process until all candidates are exhausted
    while !candidates.is_empty() {
        let mut new_candidates: Vec<(Vec<Face>, Vec<Face>)> = vec![];
        for (candidate, rem) in &candidates {
            // for each candidate, build further candidates by selecting 1 more face from the remaining selectable faces
            for i in 0..rem.len() {
                let mut new_candidate = candidate.clone();
                new_candidate.push(rem[i].clone());

                // update the list of remaining selectable faces, excluding faces from blocks used already
                let forbidden: HashSet<u8> = new_candidate.iter().map(|f| f.block).collect();
                let new_rem: Vec<Face> = rem[i + 1..]
                    .iter()
                    .filter(|&r| !forbidden.contains(&r.block))
                    .cloned()
                    .collect();

                // eliminate candidates which make up a too big combined area
                let current_area = new_candidate.iter().map(|face| face.area()).sum::<u8>();
                if area < current_area {
                    continue;
                }

                let sum = new_candidate.iter().map(|face| face.value).sum::<u8>();

                // store found solutions away;
                // keep the remaining candidates for another iteration
                if sum == n && area == current_area && form_a_rectangle(long, short, &new_candidate)
                {
                    solutions.push(new_candidate);
                } else if sum < n {
                    new_candidates.push((new_candidate, new_rem))
                }
            }
        }
        candidates = new_candidates;
    }
    solutions
}

fn form_a_rectangle(long: u8, short: u8, faces: &[Face]) -> bool {
    let size = long as usize * short as usize;
    let mut candidates = vec![((0, 0), vec![0; size], faces.to_vec())];
    while !candidates.is_empty() {
        let mut new_candidates = vec![];
        for (start_point, state, candidates) in &candidates {
            for i in 0..candidates.len() {
                let mut rem = candidates.clone();
                let face = rem.remove(i);
                match place(
                    long,
                    short,
                    start_point,
                    state,
                    face.long,
                    face.short,
                    face.value,
                ) {
                    Some((Some(new_start_point), new_state)) => {
                        new_candidates.push((new_start_point, new_state, rem.clone()))
                    }
                    Some((None, final_state)) => {
                        if rem.is_empty() {
                            trace!(
                                "Possible rectangle {} x {} for [ {}]",
                                long,
                                short,
                                faces_to_string(faces)
                            );
                            trace!("{}", state_to_string(&final_state, long, short));
                            return true;
                        }
                    }
                    None => (),
                }
                if face.long != face.short {
                    match place(
                        long,
                        short,
                        start_point,
                        state,
                        face.short,
                        face.long,
                        face.value,
                    ) {
                        Some((Some(new_start_point), new_state)) => {
                            new_candidates.push((new_start_point, new_state, rem.clone()))
                        }
                        Some((None, final_state)) => {
                            if rem.is_empty() {
                                trace!(
                                    "Possible rectangle {} x {} for [ {}]",
                                    long,
                                    short,
                                    faces_to_string(faces)
                                );
                                trace!("{}", state_to_string(&final_state, long, short));
                                return true;
                            }
                        }
                        None => (),
                    }
                }
            }
        }
        candidates = new_candidates;
    }
    trace!(
        "No possible rectangle {} x {} for [ {}]",
        long,
        short,
        faces_to_string(faces)
    );
    false
}

fn faces_to_string(faces: &[Face]) -> String {
    faces
        .iter()
        .map(|f| format!("{} ({}x{}) ", f.value, f.long, f.short))
        .collect_vec()
        .concat()
}

fn state_to_string(state: &[u8], long: u8, short: u8) -> String {
    let mut result = "state:\n".to_string();
    for j in 0..short {
        for i in 0..long {
            result.push_str(&format!("{:0>2} ", state[(j * long + i) as usize]));
        }
        result.push('\n');
    }
    result.push('\n');
    result
}

fn place(
    rect_long: u8,
    rect_short: u8,
    start_point: &(u8, u8),
    state: &[u8],
    piece_x: u8,
    piece_y: u8,
    piece_val: u8,
) -> Option<(Option<(u8, u8)>, Vec<u8>)> {
    let x_start = start_point.0;
    let x_end = x_start + piece_x;
    let y_start = start_point.1;
    let y_end = y_start + piece_y;
    if x_end <= rect_long && y_end <= rect_short {
        let mut new_state = state.to_vec();
        for j in y_start..y_end {
            for i in x_start..x_end {
                let idx = (j * rect_long + i) as usize;
                if state[idx] != 0 {
                    return None;
                }
                new_state[idx] = piece_val;
            }
        }
        let new_start_point = new_state
            .iter()
            .position(|&e| e == 0)
            .map(|idx| (idx as u8 % rect_long, idx as u8 / rect_long));
        Some((new_start_point, new_state))
    } else {
        None
    }
}

fn find_corner<'a>(
    faces_s: &'a [Face],
    faces_m: &'a [Face],
    faces_l: &'a [Face],
) -> Option<(&'a Face, &'a Face, &'a Face)> {
    let mut corners = vec![];
    for fs in faces_s {
        let block = fs.block;
        for fm in faces_m {
            if block == fm.block && fs.opposite() != fm {
                for fl in faces_l {
                    if block == fl.block && fs.opposite() != fl && fm.opposite() != fl {
                        corners.push((fs, fm, fl));
                    }
                }
            }
        }
    }
    if corners.len() == 1 {
        Some(corners[0])
    } else {
        None
    }
}

// check if the given combination of pairs of faces is sound,
// i.e. if each face can be assigned to a side of the puzzle,
// considering its relative position to the other faces of its block.
fn compatible_six_combo(
    pairs_s: &(Vec<Face>, Vec<Face>),
    pairs_m: &(Vec<Face>, Vec<Face>),
    pairs_l: &(Vec<Face>, Vec<Face>),
) -> bool {
    let mut corners = vec![];
    for faces_s in [&pairs_s.0, &pairs_s.1] {
        for faces_m in [&pairs_m.0, &pairs_m.1] {
            for faces_l in [&pairs_l.0, &pairs_l.1] {
                if let Some(corner) = find_corner(faces_s, faces_m, faces_l) {
                    corners.push(corner);
                } else {
                    return false;
                }
            }
        }
    }

    let corner0 = corners[0];

    let mut constraints = HashMap::new();
    // Assign the 1st face of the 1st corner to the Top side (arbitrary, could be Bottom too)
    constraints.insert(corner0.0, Dir::Top);
    // Assign the 2nd face of the 1st corner to the Left side (arbitrary, could be Right too)
    constraints.insert(corner0.1, Dir::Left);
    // Infer the side for the 3rd face of the corner, either Front or Back;
    // Depends on the 'spin' of the corner
    let last_constraint = if corner0.0.dir.prod(corner0.1.dir) == corner0.2.dir {
        Dir::Front
    } else if corner0.0.dir.prod(corner0.1.dir) == corner0.2.dir.opposite() {
        Dir::Back
    } else {
        panic!("nope: {:?}", corner0) // shouldnt happen here
    };

    // Propagate to all faces, check consistency
    if let Some(constraints) =
        can_propagate(pairs_s, pairs_m, pairs_l, constraints, last_constraint)
    {
        corners
            .into_iter()
            .all(|corner| is_corner_possible(&corner, &constraints))
    } else {
        false
    }
}

fn is_corner_possible(corner: &(&Face, &Face, &Face), constraints: &HashMap<&Face, Dir>) -> bool {
    let face0 = corner.0;
    let face1 = corner.1;
    let face2 = corner.2;
    let dir0 = face0.dir;
    let dir1 = face1.dir;
    let dir2 = face2.dir;
    let rot_dir0 = *constraints.get(face0).unwrap();
    let rot_dir1 = *constraints.get(face1).unwrap();
    let rot_dir2 = *constraints.get(face2).unwrap();
    let positive = dir0.prod(dir1) == dir2;
    let c_positive = rot_dir0.prod(rot_dir1) == rot_dir2;
    trace!(
        "{:?} - corner: {} ({:?} {:?} {:?}) - constraints: {} ({:?} {:?} {:?})",
        (face0.value, face1.value, face2.value),
        positive,
        dir0,
        dir1,
        dir2,
        c_positive,
        rot_dir0,
        rot_dir1,
        rot_dir2
    );
    !(positive ^ c_positive)
}

fn propagate<'a>(
    mut constraints: HashMap<&'a Face, Dir>,
    faces_and_dirs: &[(&'a [Face], Dir)],
) -> Option<HashMap<&'a Face, Dir>> {
    for &(faces, dir) in faces_and_dirs {
        for face in faces {
            if let Some(old_dir) = constraints.get(face) {
                if *old_dir != dir {
                    return None;
                }
            } else {
                constraints.insert(face, dir);
            }
            let oface = face.opposite();
            let odir = dir.opposite();
            if let Some(old_dir) = constraints.get(oface) {
                if *old_dir != odir {
                    return None;
                }
            } else {
                constraints.insert(oface, odir);
            }
        }
    }
    Some(constraints)
}

fn can_propagate<'a>(
    pairs_s: &'a (Vec<Face>, Vec<Face>),
    pairs_m: &'a (Vec<Face>, Vec<Face>),
    pairs_l: &'a (Vec<Face>, Vec<Face>),
    constraints: HashMap<&'a Face, Dir>,
    last_constraint: Dir,
) -> Option<HashMap<&'a Face, Dir>> {
    propagate(
        constraints,
        &[
            (&pairs_s.0, Dir::Top),
            (&pairs_s.1, Dir::Bottom),
            (&pairs_m.0, Dir::Left),
            (&pairs_m.1, Dir::Right),
            (&pairs_l.0, last_constraint),
            (&pairs_l.1, last_constraint.opposite()),
        ],
    )
}

fn match_combos_in_pairs(
    combos: Vec<Vec<Face>>,
    long: u8,
    short: u8,
) -> Vec<(Vec<Face>, Vec<Face>)> {
    let mut pairs: Vec<(Vec<Face>, Vec<Face>)> = vec![];
    let area = long * short;

    let mut non_oppo_out_count = 0;
    let mut oppo_out_count = 0;
    let mut face_too_big_out_count = 0;

    // take potential solutions for opposite sides 2 by 2
    for i in 0..combos.len() {
        let sol_a = &combos[i];
        for face_a in sol_a {
            if face_a.long > long || face_a.short > short {
                face_too_big_out_count += 1;
                continue;
            }
        }
        'roger: for sol_b in &combos[i..] {
            for face_a in sol_a {
                for face_b in sol_b {
                    if face_a.block == face_b.block {
                        // exclude pairs using non opposite faces of a common block
                        if face_a.dir.opposite() != face_b.dir {
                            non_oppo_out_count += 1;
                            continue 'roger;
                        }
                        // exclude pairs using opposite faces of a common block,
                        // if the block hasnt the correct dimension to expose a face
                        // on each side of the puzzle
                        let i = face_a.block as usize;
                        let depth = BLOCKS[i].depth;
                        let width = BLOCKS[i].width;
                        let height = BLOCKS[i].height;
                        // preemptively increase the counter, to simplify code
                        oppo_out_count += 1;
                        match (area, &face_a.dir) {
                            (AREA_L, Dir::Front) if depth != DEPTH => continue 'roger,
                            (AREA_L, Dir::Back) if depth != DEPTH => continue 'roger,
                            (AREA_L, Dir::Left) if width != DEPTH => continue 'roger,
                            (AREA_L, Dir::Right) if width != DEPTH => continue 'roger,
                            (AREA_L, Dir::Top) if height != DEPTH => continue 'roger,
                            (AREA_L, Dir::Bottom) if height != DEPTH => continue 'roger,
                            (AREA_M, Dir::Front) if depth != WIDTH => continue 'roger,
                            (AREA_M, Dir::Back) if depth != WIDTH => continue 'roger,
                            (AREA_M, Dir::Left) if width != WIDTH => continue 'roger,
                            (AREA_M, Dir::Right) if width != WIDTH => continue 'roger,
                            (AREA_M, Dir::Top) if height != WIDTH => continue 'roger,
                            (AREA_M, Dir::Bottom) if height != WIDTH => continue 'roger,
                            (AREA_S, Dir::Front) if depth != HEIGHT => continue 'roger,
                            (AREA_S, Dir::Back) if depth != HEIGHT => continue 'roger,
                            (AREA_S, Dir::Left) if width != HEIGHT => continue 'roger,
                            (AREA_S, Dir::Right) if width != HEIGHT => continue 'roger,
                            (AREA_S, Dir::Top) if height != HEIGHT => continue 'roger,
                            (AREA_S, Dir::Bottom) if height != HEIGHT => continue 'roger,
                            // reset the preemptively increased counter if not excluded in the end
                            _ => oppo_out_count -= 1,
                        }
                    }
                }
            }
            pairs.push((sol_a.clone(), sol_b.clone()));
        }
    }
    debug!("Removed {} for sharing a block but non opposite faces, {} for sharing a block of wrong dimension, {} for having a too big face", non_oppo_out_count, oppo_out_count,face_too_big_out_count);
    debug!(
        "{}x{}'s combos, size: {}, viable pairs count: {} out of {}",
        short,
        long,
        combos.len(),
        pairs.len(),
        (combos.len() * (combos.len() + 1)) / 2,
    );

    pairs
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn form_a_rectangle_works_neg() {
        let _ = env_logger::builder().is_test(true).try_init();
        assert!(!form_a_rectangle(
            2,
            2,
            &[
                Face {
                    value: 1,
                    long: 1,
                    short: 1,
                    block: 2,
                    dir: Dir::Top,
                },
                Face {
                    value: 2,
                    long: 2,
                    short: 1,
                    block: 1,
                    dir: Dir::Back,
                },
                Face {
                    value: 3,
                    long: 2,
                    short: 1,
                    block: 8,
                    dir: Dir::Front,
                },
            ]
        ));
    }
    #[test]
    fn form_a_rectangle_works_pos_1() {
        let _ = env_logger::builder().is_test(true).try_init();
        assert!(form_a_rectangle(
            11,
            9,
            &[
                Face {
                    value: 30,
                    long: 5,
                    short: 5,
                    block: 2,
                    dir: Dir::Top,
                },
                Face {
                    value: 31,
                    long: 9,
                    short: 6,
                    block: 1,
                    dir: Dir::Back,
                },
                Face {
                    value: 39,
                    long: 5,
                    short: 4,
                    block: 8,
                    dir: Dir::Front,
                },
            ]
        ));
    }
    #[test]
    fn form_a_rectangle_works_pos_2() {
        let _ = env_logger::builder().is_test(true).try_init();
        assert!(form_a_rectangle(
            4,
            3,
            &[
                Face {
                    value: 4,
                    long: 4,
                    short: 1,
                    block: 0,
                    dir: Dir::Top,
                },
                Face {
                    value: 2,
                    long: 2,
                    short: 1,
                    block: 0,
                    dir: Dir::Back,
                },
                Face {
                    value: 6,
                    long: 3,
                    short: 2,
                    block: 0,
                    dir: Dir::Front,
                },
            ]
        ));
    }
    #[test]
    fn form_a_rectangle_works_pos_3() {
        let _ = env_logger::builder().is_test(true).try_init();
        assert!(form_a_rectangle(
            6,
            4,
            &[
                Face {
                    value: 1,
                    long: 1,
                    short: 1,
                    block: 0,
                    dir: Dir::Top,
                },
                Face {
                    value: 2,
                    long: 2,
                    short: 1,
                    block: 0,
                    dir: Dir::Back,
                },
                Face {
                    value: 3,
                    long: 3,
                    short: 1,
                    block: 0,
                    dir: Dir::Front,
                },
                Face {
                    value: 4,
                    long: 4,
                    short: 1,
                    block: 0,
                    dir: Dir::Front,
                },
                Face {
                    value: 6,
                    long: 3,
                    short: 2,
                    block: 0,
                    dir: Dir::Front,
                },
                Face {
                    value: 8,
                    long: 4,
                    short: 2,
                    block: 0,
                    dir: Dir::Front,
                },
            ]
        ));
    }
}
