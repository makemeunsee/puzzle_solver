use std::collections::HashSet;

use crate::common::{Block, Dir, Face};
use itertools::Itertools;
use log::{debug, trace};

pub fn solver() -> Solver {
    let height = crate::common::HEIGHT as usize;
    let width = crate::common::WIDTH as usize;
    let depth = crate::common::DEPTH as usize;
    let volume = height * width * depth;
    let rot_blocks = crate::common::BLOCKS
        .iter()
        .map(|block| all_rots(block).to_vec())
        .collect_vec();
    let mut solver = Solver {
        block_count: crate::common::BLOCK_COUNT,
        rot_count: ROT_COUNT,
        puzzle_height: height,
        puzzle_width: width,
        puzzle_depth: depth,
        rot_blocks,
        stack: vec![],
        rem: HashSet::from_iter(0..crate::common::BLOCK_COUNT),
        position: 0,
        state: vec![None; volume],
        done: false,
        count: 0,
    };
    solver.init();
    solver
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
            label: block.label,
        },
        Dir::Back => Block {
            height: block.width,
            width: block.height,
            depth: block.depth,
            faces: rot_faces(axis, &block.faces),
            label: block.label,
        },
        Dir::Left => Block {
            height: block.depth,
            width: block.width,
            depth: block.height,
            faces: rot_faces(axis, &block.faces),
            label: block.label,
        },
        Dir::Right => Block {
            height: block.depth,
            width: block.width,
            depth: block.height,
            faces: rot_faces(axis, &block.faces),
            label: block.label,
        },
        Dir::Top => Block {
            height: block.height,
            width: block.depth,
            depth: block.width,
            faces: rot_faces(axis, &block.faces),
            label: block.label,
        },
        Dir::Bottom => Block {
            height: block.height,
            width: block.depth,
            depth: block.width,
            faces: rot_faces(axis, &block.faces),
            label: block.label,
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

#[derive(Debug, Clone, Copy)]
pub struct BlockInPlace {
    // the index of a block row within a reference [[Block]] 2D array
    block_id: usize,
    // the index of a rotation within a reference [Block] array
    rot_id: usize,
    // where the block was placed
    position: usize,
}

pub struct Solver {
    block_count: usize,
    rot_count: usize,
    puzzle_height: usize,
    puzzle_width: usize,
    puzzle_depth: usize,
    rot_blocks: Vec<Vec<Block>>,
    stack: Vec<BlockInPlace>,
    // ids of blocks still to be stacked
    rem: HashSet<usize>,
    // next position within state where to place a block
    position: usize,
    // 3d array tracking what space of the puzzle is filled with blocks
    state: Vec<Option<BlockInPlace>>,
    done: bool,
    count: u64,
}

impl Solver {
    pub fn stack(&self) -> Vec<(&Block, usize, usize, usize, usize)> {
        let slice_area = self.puzzle_height * self.puzzle_width;
        self.stack
            .iter()
            .map(|bip| {
                let x = bip.position % self.puzzle_height;
                let y = (bip.position % slice_area) / self.puzzle_height;
                let z = bip.position / slice_area;
                (
                    &self.rot_blocks[bip.block_id][bip.rot_id],
                    bip.block_id,
                    x,
                    y,
                    z,
                )
            })
            .collect_vec()
    }

    pub fn solution_count(&self) -> u64 {
        self.count
    }

    fn print_state(&self) -> String {
        let mut result = String::new();
        let mut idx = 0;
        for _ in 0..self.puzzle_depth {
            for _ in 0..self.puzzle_width {
                for _ in 0..self.puzzle_height {
                    result.push_str(&format!(
                        "{} ",
                        self.state[idx]
                            .map(|b| self.rot_blocks[b.block_id][b.rot_id].label)
                            .unwrap_or("")
                    ));
                    idx += 1;
                }
                result.push('\n');
            }
            result.push('\n');
        }
        result
    }

    fn print_stack_tiny(&self) -> String {
        let mut result = "[ ".to_string();
        for block in &self.stack {
            result.push_str(&format!("{} ", block.block_id));
        }
        result.push(']');
        result
    }

    pub fn done(&self) -> bool {
        self.done
    }

    pub fn step_to_solution(&mut self) -> bool {
        while self.step() {
            if self.stack.len() == self.block_count {
                return true;
            }
        }
        false
    }

    pub fn step(&mut self) -> bool {
        trace!("step");
        if !self.rem.is_empty() {
            trace!("deeper?");
            // case 1:
            // try to go deeper (place a new block)
            let rem = self.rem.clone().into_iter().sorted().collect_vec();
            for block_id in rem {
                for rot_id in 0..self.rot_count {
                    if self.deeper(block_id, rot_id) {
                        trace!(
                            "deeper ({}={}), new block {}, rot {} - rem {:?}",
                            self.stack.len(),
                            self.print_stack_tiny(),
                            block_id,
                            rot_id,
                            self.rem
                        );
                        return true;
                    } else {
                        trace!("skipped block {}, rot {} (doesnt fit)", block_id, rot_id);
                    }
                }
            }
        }
        trace!("not deeper, sideway?");
        // cant place a new block ->
        loop {
            // case 2:
            // move sideway in the graph (replace the block at the top of the stack)
            // case 3:
            // or backtrack
            if self.move_sideway_or_backtrack() {
                return true;
            }
            // case 4:
            // cant move sideway, cant backtrack, the end
            if self.stack.is_empty() {
                break;
            }
        }
        trace!("not sideway, done");
        self.done = true;
        false
    }

    fn move_sideway_or_backtrack(&mut self) -> bool {
        // backtrack
        let top = self.stack.pop().unwrap();
        let block_id = top.block_id;
        let rot_id = top.rot_id;
        let position = top.position;
        self.remove_block_from_state(top);
        self.position = position;
        self.rem.insert(block_id);

        // try placing again the same block, with a different rot
        for rot_id in rot_id + 1..self.rot_count {
            if self.deeper(block_id, rot_id) {
                trace!(
                    "sideway a ({}={}), new block {}, rot {} - rem {:?}",
                    self.stack.len(),
                    self.print_stack_tiny(),
                    block_id,
                    rot_id,
                    self.rem
                );
                return true;
            } else {
                trace!("skipped block {}, rot {} (doesnt fit)", block_id, rot_id);
            }
        }
        // try placing a sibling
        for block_id in block_id + 1..self.block_count {
            if !self.rem.contains(&block_id) {
                continue;
            }
            for rot_id in 0..self.rot_count {
                if self.deeper(block_id, rot_id) {
                    trace!(
                        "sideway b ({}={}), new block {}, rot {} - rem {:?}",
                        self.stack.len(),
                        self.print_stack_tiny(),
                        block_id,
                        rot_id,
                        self.rem
                    );
                    return true;
                } else {
                    trace!("skipped block {}, rot {} (doesnt fit)", block_id, rot_id);
                }
            }
        }
        // stay backtracked
        trace!(
            "not sideway, backtracking ({}={}) - rem {:?}",
            self.stack.len(),
            self.print_stack_tiny(),
            self.rem
        );
        false
    }

    fn remove_block_from_state(&mut self, rot_block: BlockInPlace) {
        let position = rot_block.position;
        let block = &self.rot_blocks[rot_block.block_id][rot_block.rot_id];

        let slice_area = self.puzzle_height * self.puzzle_width;

        let x_start = position % self.puzzle_height;
        let x_end = x_start + block.height as usize;

        let y_start = (position % slice_area) / self.puzzle_height;
        let y_end = y_start + block.width as usize;

        let z_start = position / slice_area;
        let z_end = z_start + block.depth as usize;

        for k in z_start..z_end {
            for j in y_start..y_end {
                for i in x_start..x_end {
                    let idx = k * slice_area + j * self.puzzle_height + i;
                    self.state[idx] = None;
                }
            }
        }
    }

    // try to go deeper in the solution graph, by placing one more rotated block
    fn deeper(&mut self, block_id: usize, rot_id: usize) -> bool {
        let block = &self.rot_blocks[block_id][rot_id];
        match place_3d(
            self.puzzle_height,
            self.puzzle_width,
            self.puzzle_depth,
            &mut self.state,
            BlockInPlace {
                block_id,
                rot_id,
                position: self.position,
            },
            block,
        ) {
            Some(new_position) => {
                self.stack.push(BlockInPlace {
                    block_id,
                    rot_id,
                    position: self.position,
                });
                self.rem.remove(&block_id);
                self.position = new_position;
                if self.position == self.puzzle_height * self.puzzle_width * self.puzzle_depth {
                    debug!("solution:\n{}", self.print_state());
                    self.count += 1;
                }
                true
            }
            None => false,
        }
    }

    fn init(&mut self) -> bool {
        trace!("init");
        self.deeper(0, 0);
        trace!(
            "deeper ({}={}), new block {}, rot {} - rem {:?}",
            self.stack.len(),
            self.print_stack_tiny(),
            0,
            0,
            self.rem
        );
        true
    }
}

fn place_3d(
    cuboid_height: usize,
    cuboid_width: usize,
    cuboid_depth: usize,
    state: &mut [Option<BlockInPlace>],
    block_id: BlockInPlace,
    block: &Block,
) -> Option<usize> {
    let slice_area = cuboid_height * cuboid_width;

    let start_point = block_id.position;

    let x_start = start_point % cuboid_height;
    let x_end = x_start + block.height as usize;

    let y_start = (start_point % slice_area) / cuboid_height;
    let y_end = y_start + block.width as usize;

    let z_start = start_point / slice_area;
    let z_end = z_start + block.depth as usize;

    if x_end <= cuboid_height && y_end <= cuboid_width && z_end <= cuboid_depth {
        // only if the block fits...
        for k in z_start..z_end {
            for j in y_start..y_end {
                for i in x_start..x_end {
                    let idx = k * slice_area + j * cuboid_height + i;
                    if state[idx].is_some() {
                        return None;
                    }
                }
            }
        }
        // ... is the state updated
        for k in z_start..z_end {
            for j in y_start..y_end {
                for i in x_start..x_end {
                    let idx = k * slice_area + j * cuboid_height + i;
                    state[idx] = Some(block_id);
                }
            }
        }
        let volume = slice_area * cuboid_depth;
        let new_start_point = state
            .iter()
            .skip(start_point)
            .position(|&e| e.is_none())
            .map(|r| r + start_point)
            .unwrap_or(volume);
        Some(new_start_point)
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
        label: "2X1X1",
    };

    #[test]
    fn solve_2x_2x1x1_in_2x2x1() {
        let _ = env_logger::builder().is_test(true).try_init();

        let mut block_a = TEST_BLOCK_2X1X1.clone();
        block_a.label = "A";
        let block_a_rot_1 = rot_block(&Dir::Front, &block_a);
        let block_a_rot_2 = rot_block(&Dir::Right, &block_a);

        let mut block_b = TEST_BLOCK_2X1X1.clone();
        block_b.label = "B";
        let block_b_rot_1 = rot_block(&Dir::Front, &block_b);
        let block_b_rot_2 = rot_block(&Dir::Right, &block_b);

        let rot_blocks = [
            [block_a, block_a_rot_1, block_a_rot_2].to_vec(),
            [block_b, block_b_rot_1, block_b_rot_2].to_vec(),
        ]
        .to_vec();

        let mut solver = Solver {
            block_count: 2,
            rot_count: 3,
            puzzle_height: 2,
            puzzle_width: 2,
            puzzle_depth: 1,
            rot_blocks,
            stack: vec![],
            rem: HashSet::from_iter(0..2),
            position: 0,
            state: vec![None; 4],
            done: false,
            count: 0,
        };
        solver.init();
        while !solver.done() && solver.step() {}
        assert_eq!(solver.solution_count(), 4);
    }

    #[test]
    fn solve_4x_2x1x1_in_2x2x2() {
        let _ = env_logger::builder().is_test(true).try_init();

        let mut block_a = TEST_BLOCK_2X1X1.clone();
        block_a.label = "A";
        let block_a_rot_1 = rot_block(&Dir::Front, &block_a);
        let block_a_rot_2 = rot_block(&Dir::Right, &block_a);

        let mut block_b = TEST_BLOCK_2X1X1.clone();
        block_b.label = "B";
        let block_b_rot_1 = rot_block(&Dir::Front, &block_b);
        let block_b_rot_2 = rot_block(&Dir::Right, &block_b);

        let mut block_c = TEST_BLOCK_2X1X1.clone();
        block_c.label = "C";
        let block_c_rot_1 = rot_block(&Dir::Front, &block_c);
        let block_c_rot_2 = rot_block(&Dir::Right, &block_c);

        let mut block_d = TEST_BLOCK_2X1X1.clone();
        block_d.label = "D";
        let block_d_rot_1 = rot_block(&Dir::Front, &block_d);
        let block_d_rot_2 = rot_block(&Dir::Right, &block_d);

        let rot_blocks = [
            [block_a, block_a_rot_1, block_a_rot_2].to_vec(),
            [block_b, block_b_rot_1, block_b_rot_2].to_vec(),
            [block_c, block_c_rot_1, block_c_rot_2].to_vec(),
            [block_d, block_d_rot_1, block_d_rot_2].to_vec(),
        ]
        .to_vec();

        let mut solver = Solver {
            block_count: 4,
            rot_count: 3,
            puzzle_height: 2,
            puzzle_width: 2,
            puzzle_depth: 2,
            rot_blocks,
            stack: vec![],
            rem: HashSet::from_iter(0..4),
            position: 0,
            state: vec![None; 8],
            done: false,
            count: 0,
        };
        solver.init();
        while !solver.done() && solver.step() {}
        assert_eq!(solver.solution_count(), 216);
    }
}
