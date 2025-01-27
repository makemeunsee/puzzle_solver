use std::collections::HashSet;

use crate::common::{Block, Dir, Face, BLOCK_COUNT, DEPTH, HEIGHT, WIDTH};
use itertools::Itertools;
use log::{debug, trace};

pub fn solver(shape_only: bool) -> Solver {
    let height = HEIGHT as usize;
    let width = WIDTH as usize;
    let depth = DEPTH as usize;
    let volume = height * width * depth;
    let (rot_blocks, target) = if shape_only {
        (
            crate::common::BLOCKS.iter().map(shape_rots).collect_vec(),
            None,
        )
    } else {
        (
            crate::common::BLOCKS.iter().map(all_rots).collect_vec(),
            Some(100),
        )
    };
    let mut solver = Solver {
        puzzle_height: height,
        puzzle_width: width,
        puzzle_depth: depth,
        target,
        rot_blocks,
        stack: vec![],
        rem: HashSet::from_iter(0..BLOCK_COUNT),
        position: 0,
        state: vec![None; volume],
        face_sums: [0; 6],
        face_free_areas: [
            HEIGHT * WIDTH,
            HEIGHT * WIDTH,
            HEIGHT * DEPTH,
            HEIGHT * DEPTH,
            WIDTH * DEPTH,
            WIDTH * DEPTH,
        ],
        done: false,
        solutions: HashSet::new(),
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

// create the rotational variants of a block, treating it as a faceless shape
// usually 6 variants, possibly fewer:
// 3 (square base case)
// 1 (cube case)
fn shape_rots(block: &Block) -> Vec<Block> {
    let rotss = [
        vec![],
        vec![Dir::Top],
        vec![Dir::Back],
        vec![Dir::Back, Dir::Top],
        vec![Dir::Right],
        vec![Dir::Right, Dir::Top],
    ];
    let mut result = vec![];
    for rots in &rotss {
        let mut res = block.clone();
        for rot in rots {
            res = rot_block(rot, &res);
        }
        result.push(res);
    }
    result
        // removes invariant rotations:
        .into_iter()
        .unique_by(|b| (b.height, b.width, b.depth))
        .collect_vec()
}

// create the 24 rotational variants of a block
fn all_rots(block: &Block) -> Vec<Block> {
    // order matters! some optimisation relies on it, see `move_sideway_or_backtrack`
    let rotss_1 = [
        vec![],
        vec![Dir::Top],
        vec![Dir::Back],
        vec![Dir::Back, Dir::Top],
        vec![Dir::Right],
        vec![Dir::Right, Dir::Top],
    ];
    let rotss_2 = [
        vec![],
        vec![Dir::Top, Dir::Top],
        vec![Dir::Right, Dir::Right],
        vec![Dir::Back, Dir::Back],
    ];
    let mut result = vec![];
    for rots_1 in &rotss_1 {
        for rots_2 in &rotss_2 {
            let mut res = block.clone();
            for r in rots_1 {
                res = rot_block(r, &res);
            }
            for r in rots_2 {
                res = rot_block(r, &res);
            }
            result.push(res);
        }
    }
    result
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BlockInPuzzle {
    // the index of a block row within a reference [[Block]] 2D array
    block_id: usize,
    // the index of a rotation within a reference [Block] array
    rot_id: usize,
    // where the block was placed
    position: usize,
}

pub struct Solver {
    puzzle_height: usize,
    puzzle_width: usize,
    puzzle_depth: usize,
    target: Option<u8>,
    rot_blocks: Vec<Vec<Block>>,
    stack: Vec<BlockInPuzzle>,
    // ids of blocks still to be stacked
    rem: HashSet<usize>,
    // next position within state where to place a block
    position: usize,
    // 3d array tracking what space of the puzzle is filled with blocks
    state: Vec<Option<BlockInPuzzle>>,
    face_sums: [u8; 6],
    face_free_areas: [u8; 6],
    done: bool,
    solutions: HashSet<Vec<BlockInPuzzle>>,
}

impl Solver {
    fn block_count(&self) -> usize {
        self.rot_blocks.len()
    }

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

    pub fn solutions(&self) -> Vec<String> {
        self.solutions
            .iter()
            .map(|sol| {
                print_state(
                    self.puzzle_height,
                    self.puzzle_width,
                    self.puzzle_depth,
                    &self.rot_blocks,
                    &sol.iter().map(|x| Some(*x)).collect_vec(),
                )
            })
            .collect_vec()
    }

    fn print_state(&self) -> String {
        print_state(
            self.puzzle_height,
            self.puzzle_width,
            self.puzzle_depth,
            &self.rot_blocks,
            &self.state,
        )
    }

    fn print_stack_tiny(&self) -> String {
        let mut result = "[ ".to_string();
        for bip in &self.stack {
            result.push_str(&format!("({}, {}) ", bip.block_id, bip.rot_id));
        }
        result.push_str("] - sums: ");
        result.push_str(&format!("{:?}", self.face_sums));
        result
    }

    pub fn done(&self) -> bool {
        self.done
    }

    pub fn step_to_solution(&mut self) -> bool {
        while self.step() {
            if self.stack.len() == self.block_count() {
                return true;
            }
        }
        false
    }

    fn trace_deeper(&self) {
        let bip = self.stack.last().unwrap();
        trace!(
            "deeper ({}={}), new block {} {}, rot {} - rem {:?}",
            self.stack.len(),
            self.print_stack_tiny(),
            bip.block_id,
            self.rot_blocks[bip.block_id][bip.rot_id].label,
            bip.rot_id,
            self.rem
        );
    }

    pub fn step(&mut self) -> bool {
        trace!("step");
        if !self.rem.is_empty() {
            trace!("deeper?");
            // case 1:
            // try to go deeper (place a new block)
            // note: sorting is not necessary, but convenient when tracing the execution
            let rem = self.rem.clone().into_iter().sorted().collect_vec();
            for block_id in rem {
                for rot_id in 0..self.rot_blocks[block_id].len() {
                    if self.deeper(block_id, rot_id) {
                        self.trace_deeper();
                        return true;
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
        self.remove_block_from_face_state(top);
        self.position = position;
        self.rem.insert(block_id);

        let mut limit = self.rot_blocks[block_id].len();

        // discard rotational invariants:
        // limit the possible rotations of the 1st block
        if self.target.is_some() && self.stack.is_empty() {
            limit /= 4; // ugly assumption: rotations must be conveniently ordered
        };

        // try placing again the same block, with a different rot
        for rot_id in rot_id + 1..limit {
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
        for block_id in block_id + 1..self.block_count() {
            if !self.rem.contains(&block_id) {
                continue;
            }
            for rot_id in 0..self.rot_blocks[block_id].len() {
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

    fn remove_block_from_face_state(&mut self, bip: BlockInPuzzle) {
        let block = &self.rot_blocks[bip.block_id][bip.rot_id];
        let position = bip.position;

        let slice_area = self.puzzle_height * self.puzzle_width;

        let x_start = position % self.puzzle_height;
        let x_end = x_start + block.height as usize;

        let y_start = (position % slice_area) / self.puzzle_height;
        let y_end = y_start + block.width as usize;

        let z_start = position / slice_area;
        let z_end = z_start + block.depth as usize;

        let mut clean_face = |dir: Dir| {
            let idx = dir as usize;
            self.face_sums[idx] -= block.faces[idx].value;
            self.face_free_areas[idx] += block.faces[idx].area();
        };

        if self.target.is_some() {
            if x_start == 0 {
                clean_face(Dir::Bottom);
            }
            if x_end == self.puzzle_height {
                clean_face(Dir::Top);
            }
            if y_start == 0 {
                clean_face(Dir::Left);
            }
            if y_end == self.puzzle_width {
                clean_face(Dir::Right);
            }
            if z_start == 0 {
                clean_face(Dir::Front);
            }
            if z_end == self.puzzle_depth {
                clean_face(Dir::Back);
            }
        }
    }

    fn remove_block_from_state(&mut self, bip: BlockInPuzzle) {
        let position = bip.position;
        let block = &self.rot_blocks[bip.block_id][bip.rot_id];

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
        match self.place_3d(BlockInPuzzle {
            block_id,
            rot_id,
            position: self.position,
        }) {
            Some(new_position) => {
                self.stack.push(BlockInPuzzle {
                    block_id,
                    rot_id,
                    position: self.position,
                });
                self.rem.remove(&block_id);
                self.position = new_position;
                if self.position == self.puzzle_height * self.puzzle_width * self.puzzle_depth {
                    let solution = self.print_state();
                    debug!("solution:\n{}\n{}", &solution, self.print_stack_tiny());
                    self.solutions
                        .insert(self.state.iter().map(|mbip| mbip.unwrap()).collect_vec());
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

    fn place_3d(&mut self, bip: BlockInPuzzle) -> Option<usize> {
        let block = &self.rot_blocks[bip.block_id][bip.rot_id];
        let slice_area = self.puzzle_height * self.puzzle_width;

        let start_point = bip.position;

        let x_start = start_point % self.puzzle_height;
        let x_end = x_start + block.height as usize;

        let y_start = (start_point % slice_area) / self.puzzle_height;
        let y_end = y_start + block.width as usize;

        let z_start = start_point / slice_area;
        let z_end = z_start + block.depth as usize;

        if x_end > self.puzzle_height || y_end > self.puzzle_width || z_end > self.puzzle_depth {
            trace!("block sticks out");
            return None;
        }
        if let Some(target_sum) = self.target {
            // if a puzzle face is completed, its value must add up to target_sum
            if x_start == 0 {
                let idx = Dir::Bottom as usize;
                let current_sum = self.face_sums[idx];
                let new_sum = current_sum + block.faces[idx].value;
                if new_sum > target_sum {
                    trace!("bottom sum too big");
                    return None;
                }
                if new_sum == target_sum {
                    for k in 0..self.puzzle_depth {
                        for j in 0..self.puzzle_width {
                            let idx = k * slice_area + j * self.puzzle_height;
                            // where the block would fit on the face
                            if k >= z_start && k < z_end && j >= y_start && j < y_end {
                                if self.state[idx].is_some() {
                                    // if there's another block -> bail
                                    trace!("bottom there's a block here");
                                    return None;
                                } else {
                                    // if it's empty -> ok
                                    continue;
                                }
                            }
                            // this is spot is not filled nor would be filled by the block -> bail
                            if self.state[idx].is_none() {
                                trace!("bottom sum==target_sum but face not full");
                                return None;
                            }
                        }
                    }
                }
                if new_sum < target_sum
                    && self.face_free_areas[idx] as i16 - block.faces[idx].area() as i16 == 0
                {
                    return None;
                }
            }
            if x_end == self.puzzle_height {
                let idx = Dir::Top as usize;
                let current_sum = self.face_sums[idx];
                let new_sum = current_sum + block.faces[idx].value;
                if new_sum > target_sum {
                    trace!("top sum too big");
                    return None;
                }
                if new_sum == target_sum {
                    for k in 0..self.puzzle_depth {
                        for j in 0..self.puzzle_width {
                            let idx =
                                k * slice_area + j * self.puzzle_height + self.puzzle_height - 1;
                            // where the block would fit on the face
                            if k >= z_start && k < z_end && j >= y_start && j < y_end {
                                if self.state[idx].is_some() {
                                    // if there's another block -> bail
                                    trace!("top there's a block here");
                                    return None;
                                } else {
                                    // if it's empty -> ok
                                    continue;
                                }
                            }
                            // this is spot is not filled nor would be filled by the block -> bail
                            if self.state[idx].is_none() {
                                trace!("front sum==target_sum but face not full");
                                return None;
                            }
                        }
                    }
                }
                if new_sum < target_sum
                    && self.face_free_areas[idx] as i16 - block.faces[idx].area() as i16 == 0
                {
                    return None;
                }
            }

            if y_start == 0 {
                let idx = Dir::Left as usize;
                let current_sum = self.face_sums[idx];
                let new_sum = current_sum + block.faces[idx].value;
                if new_sum > target_sum {
                    trace!("left sum too big");
                    return None;
                }
                if new_sum == target_sum {
                    for k in 0..self.puzzle_depth {
                        for i in 0..self.puzzle_height {
                            let idx = k * slice_area + i;
                            // where the block would fit on the face
                            if k >= z_start && k < z_end && i >= x_start && i < x_end {
                                if self.state[idx].is_some() {
                                    // if there's another block -> bail
                                    trace!("left there's a block here");
                                    return None;
                                } else {
                                    // if it's empty -> ok
                                    continue;
                                }
                            }
                            // this is spot is not filled nor would be filled by the block -> bail
                            if self.state[idx].is_none() {
                                trace!("left sum==target_sum but face not full");
                                return None;
                            }
                        }
                    }
                }
                if new_sum < target_sum
                    && self.face_free_areas[idx] as i16 - block.faces[idx].area() as i16 == 0
                {
                    return None;
                }
            }
            if y_end == self.puzzle_width {
                let idx = Dir::Right as usize;
                let current_sum = self.face_sums[idx];
                let new_sum = current_sum + block.faces[idx].value;
                if new_sum > target_sum {
                    trace!("right sum too big");
                    return None;
                }
                if new_sum == target_sum {
                    for k in 0..self.puzzle_depth {
                        for i in 0..self.puzzle_height {
                            let idx =
                                k * slice_area + (self.puzzle_width - 1) * self.puzzle_height + i;
                            // where the block would fit on the face
                            if k >= z_start && k < z_end && i >= x_start && i < x_end {
                                if self.state[idx].is_some() {
                                    // if there's another block -> bail
                                    trace!("right there's a block here");
                                    return None;
                                } else {
                                    // if it's empty -> ok
                                    continue;
                                }
                            }
                            // this is spot is not filled nor would be filled by the block -> bail
                            if self.state[idx].is_none() {
                                trace!("right sum==target_sum but face not full");
                                return None;
                            }
                        }
                    }
                }
                if new_sum < target_sum
                    && self.face_free_areas[idx] as i16 - block.faces[idx].area() as i16 == 0
                {
                    return None;
                }
            }

            if z_start == 0 {
                let idx = Dir::Front as usize;
                let current_sum = self.face_sums[idx];
                let new_sum = current_sum + block.faces[idx].value;
                if new_sum > target_sum {
                    trace!("front sum too big");
                    return None;
                }
                if new_sum == target_sum {
                    for j in 0..self.puzzle_width {
                        for i in 0..self.puzzle_height {
                            let idx = j * self.puzzle_height + i;
                            // where the block would fit on the face
                            if j >= y_start && j < y_end && i >= x_start && i < x_end {
                                if self.state[idx].is_some() {
                                    // if there's another block -> bail
                                    trace!("front there's a block here");
                                    return None;
                                } else {
                                    // if it's empty -> ok
                                    continue;
                                }
                            }
                            // this is spot is not filled nor would be filled by the block -> bail
                            if self.state[idx].is_none() {
                                trace!("front sum==target_sum but face not full");
                                return None;
                            }
                        }
                    }
                }
                if new_sum < target_sum
                    && self.face_free_areas[idx] as i16 - block.faces[idx].area() as i16 == 0
                {
                    return None;
                }
            }
            if z_end == self.puzzle_depth {
                let idx = Dir::Back as usize;
                let current_sum = self.face_sums[idx];
                let new_sum = current_sum + block.faces[idx].value;
                if new_sum > target_sum {
                    trace!("back sum too big");
                    return None;
                }
                if new_sum == target_sum {
                    for j in 0..self.puzzle_width {
                        for i in 0..self.puzzle_height {
                            let idx =
                                (self.puzzle_depth - 1) * slice_area + j * self.puzzle_height + i;
                            // where the block would fit on the face
                            if j >= y_start && j < y_end && i >= x_start && i < x_end {
                                if self.state[idx].is_some() {
                                    // if there's another block -> bail
                                    trace!("back there's a block here");
                                    return None;
                                } else {
                                    // if it's empty -> ok
                                    continue;
                                }
                            }
                            // this is spot is not filled nor would be filled by the block -> bail
                            if self.state[idx].is_none() {
                                trace!("back sum==target_sum but face not full");
                                return None;
                            }
                        }
                    }
                }
                if new_sum < target_sum
                    && self.face_free_areas[idx] as i16 - block.faces[idx].area() as i16 == 0
                {
                    return None;
                }
            }
        }

        // only if the block fits...
        for k in z_start..z_end {
            for j in y_start..y_end {
                for i in x_start..x_end {
                    let idx = k * slice_area + j * self.puzzle_height + i;
                    if self.state[idx].is_some() {
                        trace!("there's a block here");
                        return None;
                    }
                }
            }
        }
        // ... is the state updated
        for k in z_start..z_end {
            for j in y_start..y_end {
                for i in x_start..x_end {
                    let idx = k * slice_area + j * self.puzzle_height + i;
                    self.state[idx] = Some(bip);
                }
            }
        }

        if self.target.is_some() {
            let mut add_block_to_face = |dir: Dir| {
                let idx = dir as usize;
                self.face_sums[idx] += block.faces[idx].value;
                self.face_free_areas[idx] -= block.faces[idx].area();
            };
            if x_start == 0 {
                add_block_to_face(Dir::Bottom);
            }
            if x_end == self.puzzle_height {
                add_block_to_face(Dir::Top);
            }
            if y_start == 0 {
                add_block_to_face(Dir::Left);
            }
            if y_end == self.puzzle_width {
                add_block_to_face(Dir::Right);
            }
            if z_start == 0 {
                add_block_to_face(Dir::Front);
            }
            if z_end == self.puzzle_depth {
                add_block_to_face(Dir::Back);
            }
        }

        let volume = slice_area * self.puzzle_depth;
        let new_start_point = self
            .state
            .iter()
            .skip(start_point)
            .position(|&e| e.is_none())
            .map(|r| r + start_point)
            .unwrap_or(volume);
        Some(new_start_point)
    }
}

fn print_state(
    puzzle_height: usize,
    puzzle_width: usize,
    puzzle_depth: usize,
    rot_blocks: &[Vec<Block>],
    state: &[Option<BlockInPuzzle>],
) -> String {
    let slice_area = puzzle_height * puzzle_width;

    let mut result = String::new();

    result.push_str("Front:\n");
    for i in (0..puzzle_height).rev() {
        for j in 0..puzzle_width {
            let idx = j * puzzle_height + i;
            result.push_str(&format!(
                "{} ",
                state[idx]
                    .map(|b| format!(
                        "{:0>2}",
                        rot_blocks[b.block_id][b.rot_id].faces[Dir::Front as usize].value
                    ))
                    .unwrap_or("".to_string())
            ));
        }
        result.push('\n');
    }

    result.push_str("Back:\n");
    for i in (0..puzzle_height).rev() {
        for j in (0..puzzle_width).rev() {
            let idx = (puzzle_depth - 1) * slice_area + j * puzzle_height + i;
            result.push_str(&format!(
                "{} ",
                state[idx]
                    .map(|b| format!(
                        "{:0>2}",
                        rot_blocks[b.block_id][b.rot_id].faces[Dir::Back as usize].value
                    ))
                    .unwrap_or("".to_string())
            ));
        }
        result.push('\n');
    }

    result.push_str("Left:\n");
    for i in (0..puzzle_height).rev() {
        for k in (0..puzzle_depth).rev() {
            let idx = k * slice_area + i;
            result.push_str(&format!(
                "{} ",
                state[idx]
                    .map(|b| format!(
                        "{:0>2}",
                        rot_blocks[b.block_id][b.rot_id].faces[Dir::Left as usize].value
                    ))
                    .unwrap_or("".to_string())
            ));
        }
        result.push('\n');
    }

    result.push_str("Right:\n");
    for i in (0..puzzle_height).rev() {
        for k in 0..puzzle_depth {
            let idx = k * slice_area + (puzzle_width - 1) * puzzle_height + i;
            result.push_str(&format!(
                "{} ",
                state[idx]
                    .map(|b| format!(
                        "{:0>2}",
                        rot_blocks[b.block_id][b.rot_id].faces[Dir::Right as usize].value
                    ))
                    .unwrap_or("".to_string())
            ));
        }
        result.push('\n');
    }

    result.push_str("Top:\n");
    for k in (0..puzzle_depth).rev() {
        for j in 0..puzzle_width {
            let idx = k * slice_area + j * puzzle_height + puzzle_height - 1;
            result.push_str(&format!(
                "{} ",
                state[idx]
                    .map(|b| format!(
                        "{:0>2}",
                        rot_blocks[b.block_id][b.rot_id].faces[Dir::Top as usize].value
                    ))
                    .unwrap_or("".to_string())
            ));
        }
        result.push('\n');
    }

    result.push_str("Bottom:\n");
    for k in 0..puzzle_depth {
        for j in 0..puzzle_width {
            let idx = k * slice_area + j * puzzle_height;
            result.push_str(&format!(
                "{} ",
                state[idx]
                    .map(|b| format!(
                        "{:0>2}",
                        rot_blocks[b.block_id][b.rot_id].faces[Dir::Bottom as usize].value
                    ))
                    .unwrap_or("".to_string())
            ));
        }
        result.push('\n');
    }

    result
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
            puzzle_height: 2,
            puzzle_width: 2,
            puzzle_depth: 1,
            target: None,
            rot_blocks,
            stack: vec![],
            rem: HashSet::from_iter(0..2),
            position: 0,
            state: vec![None; 4],
            face_sums: [0; 6],
            face_free_areas: [4, 4, 2, 2, 2, 2],
            done: false,
            solutions: HashSet::new(),
        };
        solver.init();
        while !solver.done() && solver.step() {}
        assert_eq!(solver.solutions.len(), 4);
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
            puzzle_height: 2,
            puzzle_width: 2,
            puzzle_depth: 2,
            target: None,
            rot_blocks,
            stack: vec![],
            rem: HashSet::from_iter(0..4),
            position: 0,
            state: vec![None; 8],
            face_sums: [0; 6],
            face_free_areas: [4, 4, 4, 4, 4, 4],
            done: false,
            solutions: HashSet::new(),
        };
        solver.init();
        while !solver.done() && solver.step() {}
        assert_eq!(solver.solutions.len(), 216);
    }

    #[test]
    fn all_rots_creates_24_distinct_blocks() {
        let _ = env_logger::builder().is_test(true).try_init();

        let block = Block {
            height: 3,
            width: 2,
            depth: 1,
            faces: [
                Face {
                    value: 0,
                    long: 3,
                    short: 2,
                    block: 0,
                    dir: Dir::Front,
                },
                Face {
                    value: 1,
                    long: 3,
                    short: 2,
                    block: 0,
                    dir: Dir::Back,
                },
                Face {
                    value: 2,
                    long: 3,
                    short: 1,
                    block: 0,
                    dir: Dir::Left,
                },
                Face {
                    value: 3,
                    long: 3,
                    short: 1,
                    block: 0,
                    dir: Dir::Right,
                },
                Face {
                    value: 4,
                    long: 2,
                    short: 1,
                    block: 0,
                    dir: Dir::Top,
                },
                Face {
                    value: 5,
                    long: 2,
                    short: 1,
                    block: 0,
                    dir: Dir::Bottom,
                },
            ],
            label: "U",
        };

        let rots = all_rots(&block);
        let set: HashSet<Block> = HashSet::from_iter(rots);
        assert_eq!(set.len(), 24);
    }

    #[test]
    fn solve_2x_2x1x1_in_2x2x1_to_sum() {
        let _ = env_logger::builder().is_test(true).try_init();

        let block_a = Block {
            height: 2,
            width: 1,
            depth: 1,
            faces: [
                Face {
                    value: 9,
                    long: 2,
                    short: 1,
                    block: 0,
                    dir: Dir::Front,
                },
                Face {
                    value: 8,
                    long: 2,
                    short: 1,
                    block: 0,
                    dir: Dir::Back,
                },
                Face {
                    value: 12,
                    long: 2,
                    short: 1,
                    block: 0,
                    dir: Dir::Left,
                },
                Face {
                    value: 14,
                    long: 2,
                    short: 1,
                    block: 0,
                    dir: Dir::Right,
                },
                Face {
                    value: 6,
                    long: 1,
                    short: 1,
                    block: 0,
                    dir: Dir::Top,
                },
                Face {
                    value: 7,
                    long: 1,
                    short: 1,
                    block: 0,
                    dir: Dir::Bottom,
                },
            ],
            label: "A",
        };

        let block_b = Block {
            height: 2,
            width: 1,
            depth: 1,
            faces: [
                Face {
                    value: 3,
                    long: 2,
                    short: 1,
                    block: 0,
                    dir: Dir::Front,
                },
                Face {
                    value: 4,
                    long: 2,
                    short: 1,
                    block: 0,
                    dir: Dir::Back,
                },
                Face {
                    value: 13,
                    long: 2,
                    short: 1,
                    block: 0,
                    dir: Dir::Left,
                },
                Face {
                    value: 12,
                    long: 2,
                    short: 1,
                    block: 0,
                    dir: Dir::Right,
                },
                Face {
                    value: 6,
                    long: 1,
                    short: 1,
                    block: 0,
                    dir: Dir::Top,
                },
                Face {
                    value: 5,
                    long: 1,
                    short: 1,
                    block: 0,
                    dir: Dir::Bottom,
                },
            ],
            label: "B",
        };

        let rot_blocks = [block_a, block_b]
            .into_iter()
            .map(|block| all_rots(&block).to_vec())
            .collect_vec();

        let mut solver = Solver {
            puzzle_height: 2,
            puzzle_width: 2,
            puzzle_depth: 1,
            target: Some(12),
            rot_blocks,
            stack: vec![],
            rem: HashSet::from_iter(0..2),
            position: 0,
            state: vec![None; 4],
            face_sums: [0; 6],
            face_free_areas: [4, 4, 2, 2, 2, 2],
            done: false,
            solutions: HashSet::new(),
        };
        solver.init();
        while !solver.done() && solver.step() {}
        assert_eq!(solver.solutions.len(), 8);
    }
}
