use std::collections::HashSet;

use crate::common::{Block, Dir, Face, BLOCKS, BLOCK_COUNT, DEPTH, HEIGHT, VOLUME, WIDTH};
use itertools::Itertools;
use lazy_static::lazy_static;
use log::{debug, info, trace};

lazy_static! {
    static ref ROT_BLOCKS: Vec<[Block; ROT_COUNT]> = BLOCKS.iter().map(all_rots).collect_vec();
}

pub fn solve() {
    let mut solver = Solver::new();
    while solver.step() {
        // TODO visualize it
    }
    // cuboid_configs(HEIGHT, WIDTH, DEPTH, &ROT_BLOCKS);
}

fn rot_face(axis: &Dir, face: &Face) -> Face {
    match (axis, &face.dir) {
        (Dir::Front, Dir::Front) => face.clone(),
        (Dir::Front, Dir::Back) => face.clone(),
        (Dir::Front, Dir::Left) => {
            let mut res = face.clone();
            res.dir = Dir::Bottom;
            res
        }
        (Dir::Front, Dir::Right) => {
            let mut res = face.clone();
            res.dir = Dir::Top;
            res
        }
        (Dir::Front, Dir::Top) => {
            let mut res = face.clone();
            res.dir = Dir::Left;
            res
        }
        (Dir::Front, Dir::Bottom) => {
            let mut res = face.clone();
            res.dir = Dir::Right;
            res
        }
        (Dir::Back, Dir::Front) => face.clone(),
        (Dir::Back, Dir::Back) => face.clone(),
        (Dir::Back, Dir::Left) => {
            let mut res = face.clone();
            res.dir = Dir::Top;
            res
        }
        (Dir::Back, Dir::Right) => {
            let mut res = face.clone();
            res.dir = Dir::Bottom;
            res
        }
        (Dir::Back, Dir::Top) => {
            let mut res = face.clone();
            res.dir = Dir::Right;
            res
        }
        (Dir::Back, Dir::Bottom) => {
            let mut res = face.clone();
            res.dir = Dir::Left;
            res
        }
        (Dir::Left, Dir::Front) => {
            let mut res = face.clone();
            res.dir = Dir::Top;
            res
        }
        (Dir::Left, Dir::Back) => {
            let mut res = face.clone();
            res.dir = Dir::Bottom;
            res
        }
        (Dir::Left, Dir::Left) => face.clone(),
        (Dir::Left, Dir::Right) => face.clone(),
        (Dir::Left, Dir::Top) => {
            let mut res = face.clone();
            res.dir = Dir::Back;
            res
        }
        (Dir::Left, Dir::Bottom) => {
            let mut res = face.clone();
            res.dir = Dir::Front;
            res
        }
        (Dir::Right, Dir::Front) => {
            let mut res = face.clone();
            res.dir = Dir::Bottom;
            res
        }
        (Dir::Right, Dir::Back) => {
            let mut res = face.clone();
            res.dir = Dir::Top;
            res
        }
        (Dir::Right, Dir::Left) => face.clone(),
        (Dir::Right, Dir::Right) => face.clone(),
        (Dir::Right, Dir::Top) => {
            let mut res = face.clone();
            res.dir = Dir::Front;
            res
        }
        (Dir::Right, Dir::Bottom) => {
            let mut res = face.clone();
            res.dir = Dir::Back;
            res
        }
        (Dir::Top, Dir::Front) => {
            let mut res = face.clone();
            res.dir = Dir::Right;
            res
        }
        (Dir::Top, Dir::Back) => {
            let mut res = face.clone();
            res.dir = Dir::Left;
            res
        }
        (Dir::Top, Dir::Left) => {
            let mut res = face.clone();
            res.dir = Dir::Front;
            res
        }
        (Dir::Top, Dir::Right) => {
            let mut res = face.clone();
            res.dir = Dir::Back;
            res
        }
        (Dir::Top, Dir::Top) => face.clone(),
        (Dir::Top, Dir::Bottom) => face.clone(),
        (Dir::Bottom, Dir::Front) => {
            let mut res = face.clone();
            res.dir = Dir::Left;
            res
        }
        (Dir::Bottom, Dir::Back) => {
            let mut res = face.clone();
            res.dir = Dir::Right;
            res
        }
        (Dir::Bottom, Dir::Left) => {
            let mut res = face.clone();
            res.dir = Dir::Back;
            res
        }
        (Dir::Bottom, Dir::Right) => {
            let mut res = face.clone();
            res.dir = Dir::Front;
            res
        }
        (Dir::Bottom, Dir::Top) => face.clone(),
        (Dir::Bottom, Dir::Bottom) => face.clone(),
    }
}

// Faces arrays are always is this order: Front, Back, Left, Right, Top, Bottom
// It means rotated faces of a block must be reordered.
fn rot_faces(axis: &Dir, faces: &[Face; 6]) -> [Face; 6] {
    faces
        .iter()
        .map(|f| rot_face(axis, f))
        .sorted_by(|a, b| Ord::cmp(&a.dir, &b.dir))
        .collect_array()
        .unwrap()
}

fn rot_block(axis: &Dir, block: &Block) -> Block {
    match axis {
        Dir::Front => Block {
            height: block.width,
            width: block.height,
            depth: block.depth,
            faces: rot_faces(axis, &block.faces),
        },
        Dir::Back => Block {
            height: block.width,
            width: block.height,
            depth: block.depth,
            faces: rot_faces(axis, &block.faces),
        },
        Dir::Left => Block {
            height: block.depth,
            width: block.width,
            depth: block.height,
            faces: rot_faces(axis, &block.faces),
        },
        Dir::Right => Block {
            height: block.depth,
            width: block.width,
            depth: block.height,
            faces: rot_faces(axis, &block.faces),
        },
        Dir::Top => Block {
            height: block.height,
            width: block.depth,
            depth: block.width,
            faces: rot_faces(axis, &block.faces),
        },
        Dir::Bottom => Block {
            height: block.height,
            width: block.depth,
            depth: block.width,
            faces: rot_faces(axis, &block.faces),
        },
    }
}

// TODO re-enable all 24 rots
const ROT_COUNT: usize = 6;

fn all_rots(block: &Block) -> [Block; ROT_COUNT] {
    let rots_1 = [vec![], vec![Dir::Back], vec![Dir::Right]];
    let rots_2 = [vec![], vec![Dir::Top]];
    let mut result = arrayvec::ArrayVec::<Block, ROT_COUNT>::new();
    for rot_1 in &rots_1 {
        for rot_2 in &rots_2 {
            let mut res = block.clone();
            for r in rot_1 {
                res = rot_block(r, &res);
            }
            for r in rot_2 {
                res = rot_block(r, &res);
            }
            result.push(res);
        }
    }
    result.into_inner().unwrap()
}

struct RotBlock<'a> {
    // a block, with its rotation applied
    block: &'a Block,
    // the id of the block within `BLOCKS`
    block_id: usize,
    // the id of the rotation
    rot_id: usize,
}

struct Solver<'a> {
    stack: Vec<RotBlock<'a>>,
    // ids of blocks still to be stacked
    rem: HashSet<usize>,
    // next position within state where to place a block
    position: usize,
    // 3d array tracking what space of the puzzle is filled with blocks
    state: Vec<Option<&'a Block>>,
}

impl Solver<'_> {
    fn new<'a>() -> Solver<'a> {
        let mut solver = Solver {
            stack: vec![],
            rem: HashSet::from_iter(0..BLOCK_COUNT),
            position: 0,
            state: vec![None; VOLUME],
        };
        solver.init();
        solver
    }

    fn step(&mut self) -> bool {
        if !self.rem.is_empty() {
            // case 1:
            // try to go deeper (place a new block)
            let block_id = *self.rem.iter().find_or_first(|_| true).unwrap();
            for rot_id in 0..ROT_COUNT {
                if self.deeper(block_id, rot_id) {
                    return true;
                }
            }
        }
        // cant place a new block ->
        // case 2:
        // move sideway in the graph (replace the block at the top of the stack)
        // TODO
        // cant move sideway directly ->
        // case 3:
        // backtrack until we can move sideway
        // TODO
        // still cant backtrack ->
        // the end!
        false
    }

    // try to go deeper in the solution graph, by placing one more rotated block
    fn deeper(&mut self, block_id: usize, rot_id: usize) -> bool {
        let block = &ROT_BLOCKS[block_id][rot_id];
        match place_3d(HEIGHT, WIDTH, DEPTH, self.position, &self.state, block) {
            Some((Some(new_position), new_state)) => {
                self.stack.push(RotBlock {
                    block,
                    block_id,
                    rot_id,
                });
                self.rem.remove(&block_id);
                self.position = new_position;
                self.state = new_state;
                true
            }
            Some((None, new_state)) => {
                // we've reached a solution
                // TODO: do something with the solution
                true
            }
            None => false,
        }
    }

    fn init(&mut self) -> bool {
        self.deeper(0, 0)
    }
}

fn cuboid_configs(
    height: u8,
    width: u8,
    depth: u8,
    blocks_with_rots: &[[Block; ROT_COUNT]],
) -> Vec<Vec<&Block>> {
    let slice_size = height as usize * width as usize;
    let size = slice_size * depth as usize;
    let mut candidates = vec![(
        0,
        vec![None; size],
        (0..blocks_with_rots.len()).collect_vec(),
    )];
    let mut solutions = vec![];

    // TODO parallelize?
    while !candidates.is_empty() {
        let mut new_candidates = vec![];
        for (start_point, state, candidates) in &candidates {
            for i in 0..candidates.len() {
                let mut rem = candidates.clone();
                let block_id = rem.remove(i);
                let block_rots = //if *start_point == 0 {
                    // some configurations are equivalent;
                    // we could discard them by picking only some (6 out of 24) rotations of the 1st block.
                    // hackish, relies on the rotations being listed in a consistent order
                    // &blocks_with_rots[block_id][??]
                // } else {
                    &blocks_with_rots[block_id]
                // }
                ;
                for block_rot in block_rots {
                    match place_3d(height, width, depth, *start_point, state, block_rot) {
                        Some((Some(new_start_point), new_state)) => {
                            if *start_point < slice_size && new_start_point >= slice_size {
                                trace!("front slice built!");
                                // let mut sum_set = HashSet::new();
                                // for state in &new_state[0..slice_size] {
                                //     sum_set.insert(state.unwrap().faces[0].value);
                                // }
                                // let sum: u8 = sum_set.iter().sum();
                                // trace!("sum {}", sum);
                                // if sum != 100 {
                                //     trace!("valid front slice built!");
                                //     continue;
                                // }
                            }
                            new_candidates.push((new_start_point, new_state, rem.clone()))
                        }
                        Some((None, final_state)) => {
                            if rem.is_empty() {
                                trace!("Ping!");
                                solutions.push(
                                    final_state.into_iter().map(|b| b.unwrap()).collect_vec(),
                                );
                            }
                        }
                        None => (),
                    }
                }
            }
        }
        candidates = new_candidates;
        debug!("{}", candidates.len());
    }
    info!("cuboid count: {}", solutions.len());
    solutions
}

fn place_3d<'a>(
    cuboid_height: u8,
    cuboid_width: u8,
    cuboid_depth: u8,
    start_point: usize,
    state: &[Option<&'a Block>],
    block: &'a Block,
) -> Option<(Option<usize>, Vec<Option<&'a Block>>)> {
    let slice_area = cuboid_height as usize * cuboid_width as usize;

    let x_start = start_point % cuboid_height as usize;
    let x_end = x_start + block.height as usize;

    let y_start = (start_point % slice_area) / cuboid_height as usize;
    let y_end = y_start + block.width as usize;

    let z_start = start_point / slice_area;
    let z_end = z_start + block.depth as usize;

    if x_end <= cuboid_height as usize
        && y_end <= cuboid_width as usize
        && z_end <= cuboid_depth as usize
    {
        let mut new_state = state.to_vec();
        for k in z_start..z_end {
            for j in y_start..y_end {
                for i in x_start..x_end {
                    let idx = k * slice_area + j * cuboid_height as usize + i;
                    if state[idx].is_some() {
                        return None;
                    }
                    new_state[idx] = Some(block);
                }
            }
        }
        let new_start_point = new_state.iter().position(|&e| e.is_none());
        Some((new_start_point, new_state))
    } else {
        None
    }
}
#[cfg(test)]
mod test {
    use super::*;

    const TEST_FACE_1X1: Face = Face {
        value: 1,
        long: 1,
        short: 1,
        block: 1,
        dir: Dir::Front,
    };

    const TEST_FACE_2X1: Face = Face {
        value: 2,
        long: 2,
        short: 1,
        block: 2,
        dir: Dir::Front,
    };

    const TEST_FACE_3X1: Face = Face {
        value: 3,
        long: 3,
        short: 1,
        block: 3,
        dir: Dir::Front,
    };

    const TEST_FACE_2X2: Face = Face {
        value: 4,
        long: 2,
        short: 2,
        block: 3,
        dir: Dir::Front,
    };

    const TEST_FACE_3X2: Face = Face {
        value: 6,
        long: 3,
        short: 2,
        block: 3,
        dir: Dir::Front,
    };

    const TEST_BLOCK_1X1X1: Block = Block {
        height: 1,
        width: 1,
        depth: 1,
        faces: [
            TEST_FACE_1X1,
            TEST_FACE_1X1,
            TEST_FACE_1X1,
            TEST_FACE_1X1,
            TEST_FACE_1X1,
            TEST_FACE_1X1,
        ],
    };

    const TEST_BLOCK_2X1X1: Block = Block {
        height: 2,
        width: 1,
        depth: 1,
        faces: [
            TEST_FACE_2X1,
            TEST_FACE_2X1,
            TEST_FACE_2X1,
            TEST_FACE_2X1,
            TEST_FACE_1X1,
            TEST_FACE_1X1,
        ],
    };

    const TEST_BLOCK_1X2X1: Block = Block {
        height: 1,
        width: 2,
        depth: 1,
        faces: [
            TEST_FACE_2X1,
            TEST_FACE_2X1,
            TEST_FACE_2X1,
            TEST_FACE_2X1,
            TEST_FACE_1X1,
            TEST_FACE_1X1,
        ],
    };

    const TEST_BLOCK_1X1X2: Block = Block {
        height: 1,
        width: 1,
        depth: 2,
        faces: [
            TEST_FACE_2X1,
            TEST_FACE_2X1,
            TEST_FACE_2X1,
            TEST_FACE_2X1,
            TEST_FACE_1X1,
            TEST_FACE_1X1,
        ],
    };

    const TEST_BLOCK_2X2X1: Block = Block {
        height: 2,
        width: 2,
        depth: 1,
        faces: [
            TEST_FACE_2X2,
            TEST_FACE_2X2,
            TEST_FACE_2X1,
            TEST_FACE_2X1,
            TEST_FACE_2X1,
            TEST_FACE_2X1,
        ],
    };

    const TEST_BLOCK_3X2X1: Block = Block {
        height: 3,
        width: 2,
        depth: 1,
        faces: [
            TEST_FACE_3X2,
            TEST_FACE_3X2,
            TEST_FACE_3X1,
            TEST_FACE_3X1,
            TEST_FACE_2X1,
            TEST_FACE_2X1,
        ],
    };

    const TEST_BLOCK_2X2X2: Block = Block {
        height: 2,
        width: 2,
        depth: 2,
        faces: [
            TEST_FACE_2X2,
            TEST_FACE_2X2,
            TEST_FACE_2X2,
            TEST_FACE_2X2,
            TEST_FACE_2X2,
            TEST_FACE_2X2,
        ],
    };

    #[test]
    fn cuboid_1x1x1() {
        let blocks_with_rots = all_rots(&TEST_BLOCK_1X1X1);
        assert_eq!(cuboid_configs(1, 1, 1, &[blocks_with_rots]).len(), 24);
    }

    #[test]
    fn cuboid_2x1x1() {
        let mut blocks_with_rots = vec![];
        for block in [TEST_BLOCK_1X1X1, TEST_BLOCK_1X1X1] {
            blocks_with_rots.push(all_rots(&block));
        }
        assert_eq!(cuboid_configs(2, 1, 1, &blocks_with_rots).len(), 1152);

        let blocks_with_rots = all_rots(&TEST_BLOCK_2X1X1);
        assert_eq!(cuboid_configs(2, 1, 1, &[blocks_with_rots]).len(), 8);
    }

    #[test]
    fn cuboid_3x2x1() {
        let blocks_with_rots = all_rots(&TEST_BLOCK_3X2X1);
        assert_eq!(cuboid_configs(3, 2, 1, &[blocks_with_rots]).len(), 4);
    }

    #[test]
    fn cuboid_3x2x2() {
        let mut blocks_with_rots = vec![];
        for block in [TEST_BLOCK_2X2X1, TEST_BLOCK_2X2X2] {
            blocks_with_rots.push(all_rots(&block));
        }
        assert_eq!(cuboid_configs(3, 2, 2, &blocks_with_rots).len(), 384);

        let mut blocks_with_rots = vec![];
        for block in [TEST_BLOCK_3X2X1, TEST_BLOCK_3X2X1] {
            blocks_with_rots.push(all_rots(&block));
        }
        assert_eq!(cuboid_configs(3, 2, 2, &blocks_with_rots).len(), 64);
    }

    #[test]
    fn place_3d_1x1x1_in_1x1x1_empty() {
        assert_eq!(
            place_3d(1, 1, 1, 0, &[None], &TEST_BLOCK_1X1X1),
            Some((None, vec![Some(&TEST_BLOCK_1X1X1)]))
        );
    }

    #[test]
    fn place_3d_1x1x1_in_2x1x1_empty() {
        assert_eq!(
            place_3d(2, 1, 1, 0, &[None, None], &TEST_BLOCK_1X1X1),
            Some((Some(1), vec![Some(&TEST_BLOCK_1X1X1), None]))
        );
    }

    #[test]
    fn place_3d_1x1x1_in_2x1x1_not_empty() {
        assert_eq!(
            place_3d(
                2,
                1,
                1,
                1,
                &[Some(&TEST_BLOCK_1X1X1), None],
                &TEST_BLOCK_1X1X1
            ),
            Some((None, vec![Some(&TEST_BLOCK_1X1X1), Some(&TEST_BLOCK_1X1X1)]))
        );
    }

    #[test]
    fn place_3d_2x1x1_in_2x1x1_empty() {
        assert_eq!(
            place_3d(2, 1, 1, 0, &[None, None], &TEST_BLOCK_2X1X1),
            Some((None, vec![Some(&TEST_BLOCK_2X1X1), Some(&TEST_BLOCK_2X1X1)]))
        );
    }

    #[test]
    fn place_3d_2x1x1_in_2x1x1_not_empty() {
        assert_eq!(
            place_3d(
                2,
                1,
                1,
                1,
                &[Some(&TEST_BLOCK_1X1X1), None],
                &TEST_BLOCK_2X1X1
            ),
            None
        );
    }

    #[test]
    fn place_3d_1x1x1_in_2x2x2_not_empty() {
        assert_eq!(
            place_3d(
                2,
                2,
                2,
                7,
                &[
                    Some(&TEST_BLOCK_1X1X1),
                    Some(&TEST_BLOCK_1X1X1),
                    Some(&TEST_BLOCK_1X1X1),
                    Some(&TEST_BLOCK_1X1X1),
                    Some(&TEST_BLOCK_1X1X1),
                    Some(&TEST_BLOCK_1X1X1),
                    Some(&TEST_BLOCK_1X1X1),
                    None
                ],
                &TEST_BLOCK_1X1X1,
            ),
            Some((
                None,
                vec![
                    Some(&TEST_BLOCK_1X1X1),
                    Some(&TEST_BLOCK_1X1X1),
                    Some(&TEST_BLOCK_1X1X1),
                    Some(&TEST_BLOCK_1X1X1),
                    Some(&TEST_BLOCK_1X1X1),
                    Some(&TEST_BLOCK_1X1X1),
                    Some(&TEST_BLOCK_1X1X1),
                    Some(&TEST_BLOCK_1X1X1),
                ]
            ))
        );
    }

    #[test]
    fn place_3d_2x1x1_in_2x2x2_not_empty() {
        assert_eq!(
            place_3d(
                2,
                2,
                2,
                5,
                &[
                    Some(&TEST_BLOCK_1X1X1),
                    Some(&TEST_BLOCK_1X1X1),
                    Some(&TEST_BLOCK_1X1X1),
                    Some(&TEST_BLOCK_1X1X1),
                    Some(&TEST_BLOCK_1X1X1),
                    None,
                    Some(&TEST_BLOCK_1X1X1),
                    None
                ],
                &TEST_BLOCK_1X2X1
            ),
            Some((
                None,
                vec![
                    Some(&TEST_BLOCK_1X1X1),
                    Some(&TEST_BLOCK_1X1X1),
                    Some(&TEST_BLOCK_1X1X1),
                    Some(&TEST_BLOCK_1X1X1),
                    Some(&TEST_BLOCK_1X1X1),
                    Some(&TEST_BLOCK_1X2X1),
                    Some(&TEST_BLOCK_1X1X1),
                    Some(&TEST_BLOCK_1X2X1),
                ]
            ))
        );
    }

    #[test]
    fn place_3d_2x2x1_in_2x2x2_empty() {
        assert_eq!(
            place_3d(
                2,
                2,
                2,
                0,
                &[None, None, None, None, None, None, None, None],
                &TEST_BLOCK_2X2X1
            ),
            Some((
                Some(4),
                vec![
                    Some(&TEST_BLOCK_2X2X1),
                    Some(&TEST_BLOCK_2X2X1),
                    Some(&TEST_BLOCK_2X2X1),
                    Some(&TEST_BLOCK_2X2X1),
                    None,
                    None,
                    None,
                    None
                ]
            ))
        );
    }

    #[test]
    fn place_3d_2x2x1_in_2x2x2_not_empty() {
        assert_eq!(
            place_3d(
                2,
                2,
                2,
                4,
                &[
                    Some(&TEST_BLOCK_2X2X1),
                    Some(&TEST_BLOCK_2X2X1),
                    Some(&TEST_BLOCK_2X2X1),
                    Some(&TEST_BLOCK_2X2X1),
                    None,
                    None,
                    None,
                    None
                ],
                &TEST_BLOCK_2X2X1,
            ),
            Some((
                None,
                vec![
                    Some(&TEST_BLOCK_2X2X1),
                    Some(&TEST_BLOCK_2X2X1),
                    Some(&TEST_BLOCK_2X2X1),
                    Some(&TEST_BLOCK_2X2X1),
                    Some(&TEST_BLOCK_2X2X1),
                    Some(&TEST_BLOCK_2X2X1),
                    Some(&TEST_BLOCK_2X2X1),
                    Some(&TEST_BLOCK_2X2X1),
                ]
            ))
        );
    }

    #[test]
    fn place_3d_2x2x1_in_2x2x2_too_full() {
        assert_eq!(
            place_3d(
                2,
                2,
                2,
                4,
                &[
                    Some(&TEST_BLOCK_1X1X1),
                    Some(&TEST_BLOCK_1X1X2),
                    Some(&TEST_BLOCK_2X2X1),
                    Some(&TEST_BLOCK_2X2X1),
                    None,
                    Some(&TEST_BLOCK_1X1X2),
                    Some(&TEST_BLOCK_2X2X1),
                    Some(&TEST_BLOCK_2X2X1)
                ],
                &TEST_BLOCK_2X2X1
            ),
            None
        );
    }
}
