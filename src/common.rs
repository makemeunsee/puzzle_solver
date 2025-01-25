use itertools::Itertools;
use lazy_static::lazy_static;
use std::{collections::HashSet, hash::Hash};

// The puzzle is a paralleliped of dimensions 12x11x9.
// It is composed of smaller parallelepipeds (blocks).
// Each face of each block has a value assigned to it.
// The puzzle is complete when for each of the 6 faces of the puzzle,
// the sum of the values of the block faces composing the puzzle face
// equals to 100.

pub const BLOCK_COUNT: usize = 9;

pub const HEIGHT: u8 = 12;
pub const WIDTH: u8 = 11;
pub const DEPTH: u8 = 9;

pub const AREA_L: u8 = 132;
pub const AREA_M: u8 = 108;
pub const AREA_S: u8 = 99;

lazy_static! {
    static ref AREAS: HashSet<u8> = HashSet::from([AREA_L, AREA_M, AREA_S]);
}

pub const VOLUME: usize = HEIGHT as usize * WIDTH as usize * DEPTH as usize;

#[derive(Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Hash)]
pub enum Dir {
    Front,
    Back,
    Left,
    Right,
    Top,
    Bottom,
}

impl Dir {
    pub fn opposite(&self) -> Dir {
        match &self {
            Dir::Front => Dir::Back,
            Dir::Back => Dir::Front,
            Dir::Left => Dir::Right,
            Dir::Right => Dir::Left,
            Dir::Top => Dir::Bottom,
            Dir::Bottom => Dir::Top,
        }
    }

    pub fn prod(&self, dir: Dir) -> Dir {
        match (&self, dir) {
            (Dir::Front, Dir::Front) => panic!("no null dir"),
            (Dir::Front, Dir::Back) => panic!("no null dir"),
            (Dir::Front, Dir::Left) => Dir::Bottom,
            (Dir::Front, Dir::Right) => Dir::Top,
            (Dir::Front, Dir::Top) => Dir::Left,
            (Dir::Front, Dir::Bottom) => Dir::Right,
            (Dir::Back, Dir::Front) => panic!("no null dir"),
            (Dir::Back, Dir::Back) => panic!("no null dir"),
            (Dir::Back, Dir::Left) => Dir::Top,
            (Dir::Back, Dir::Right) => Dir::Bottom,
            (Dir::Back, Dir::Top) => Dir::Right,
            (Dir::Back, Dir::Bottom) => Dir::Left,
            (Dir::Left, Dir::Front) => Dir::Top,
            (Dir::Left, Dir::Back) => Dir::Bottom,
            (Dir::Left, Dir::Left) => panic!("no null dir"),
            (Dir::Left, Dir::Right) => panic!("no null dir"),
            (Dir::Left, Dir::Top) => Dir::Back,
            (Dir::Left, Dir::Bottom) => Dir::Front,
            (Dir::Right, Dir::Front) => Dir::Bottom,
            (Dir::Right, Dir::Back) => Dir::Top,
            (Dir::Right, Dir::Left) => panic!("no null dir"),
            (Dir::Right, Dir::Right) => panic!("no null dir"),
            (Dir::Right, Dir::Top) => Dir::Front,
            (Dir::Right, Dir::Bottom) => Dir::Back,
            (Dir::Top, Dir::Front) => Dir::Right,
            (Dir::Top, Dir::Back) => Dir::Left,
            (Dir::Top, Dir::Left) => Dir::Front,
            (Dir::Top, Dir::Right) => Dir::Back,
            (Dir::Top, Dir::Top) => panic!("no null dir"),
            (Dir::Top, Dir::Bottom) => panic!("no null dir"),
            (Dir::Bottom, Dir::Front) => Dir::Left,
            (Dir::Bottom, Dir::Back) => Dir::Right,
            (Dir::Bottom, Dir::Left) => Dir::Back,
            (Dir::Bottom, Dir::Right) => Dir::Front,
            (Dir::Bottom, Dir::Top) => panic!("no null dir"),
            (Dir::Bottom, Dir::Bottom) => panic!("no null dir"),
        }
    }
}

/// A face of a block
#[derive(PartialEq, Eq, Debug, Clone, Hash)]
pub struct Face {
    pub value: u8,
    pub long: u8,
    pub short: u8,
    pub block: u8, // index of the block of this face in `BLOCKS`; [0..8]
    pub dir: Dir,
}

impl Face {
    pub fn area(&self) -> u8 {
        self.long * self.short
    }

    pub fn opposite(&self) -> &Face {
        let faces = &BLOCKS[self.block as usize].faces;
        match self.dir {
            Dir::Front => &faces[1],
            Dir::Back => &faces[0],
            Dir::Left => &faces[3],
            Dir::Right => &faces[2],
            Dir::Top => &faces[5],
            Dir::Bottom => &faces[4],
        }
    }
}

/// A block of the puzzle
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Block {
    // height >= width >= depth
    pub height: u8,
    pub width: u8,
    pub depth: u8,
    pub faces: [Face; 6], // order: Front, Back, Left, Right, Top, Bottom
    pub label: &'static str,
}

/// The blocks composing the puzzle
pub const BLOCKS: [Block; BLOCK_COUNT] = [
    Block {
        height: 9,
        width: 6,
        depth: 5,
        faces: [
            Face {
                value: 43,
                long: 9,
                short: 6,
                block: 0,
                dir: Dir::Front,
            },
            Face {
                value: 18,
                long: 9,
                short: 6,
                block: 0,
                dir: Dir::Back,
            },
            Face {
                value: 7,
                long: 9,
                short: 5,
                block: 0,
                dir: Dir::Left,
            },
            Face {
                value: 47,
                long: 9,
                short: 5,
                block: 0,
                dir: Dir::Right,
            },
            Face {
                value: 36,
                long: 6,
                short: 5,
                block: 0,
                dir: Dir::Top,
            },
            Face {
                value: 14,
                long: 6,
                short: 5,
                block: 0,
                dir: Dir::Bottom,
            },
        ],
        label: "bigger_chunk",
    },
    Block {
        height: 9,
        width: 6,
        depth: 3,
        faces: [
            Face {
                value: 34,
                long: 9,
                short: 6,
                block: 1,
                dir: Dir::Front,
            },
            Face {
                value: 31,
                long: 9,
                short: 6,
                block: 1,
                dir: Dir::Back,
            },
            Face {
                value: 41,
                long: 9,
                short: 3,
                block: 1,
                dir: Dir::Left,
            },
            Face {
                value: 24,
                long: 9,
                short: 3,
                block: 1,
                dir: Dir::Right,
            },
            Face {
                value: 16,
                long: 6,
                short: 3,
                block: 1,
                dir: Dir::Top,
            },
            Face {
                value: 33,
                long: 6,
                short: 3,
                block: 1,
                dir: Dir::Bottom,
            },
        ],
        label: "thinner_chunk",
    },
    Block {
        height: 9,
        width: 5,
        depth: 5,
        faces: [
            Face {
                value: 38,
                long: 9,
                short: 5,
                block: 2,
                dir: Dir::Front,
            },
            Face {
                value: 53,
                long: 9,
                short: 5,
                block: 2,
                dir: Dir::Back,
            },
            Face {
                value: 8,
                long: 9,
                short: 5,
                block: 2,
                dir: Dir::Left,
            },
            Face {
                value: 44,
                long: 9,
                short: 5,
                block: 2,
                dir: Dir::Right,
            },
            Face {
                value: 30,
                long: 5,
                short: 5,
                block: 2,
                dir: Dir::Top,
            },
            Face {
                value: 22,
                long: 5,
                short: 5,
                block: 2,
                dir: Dir::Bottom,
            },
        ],
        label: "square_chunk",
    },
    Block {
        height: 9,
        width: 4,
        depth: 3,
        faces: [
            Face {
                value: 49,
                long: 9,
                short: 4,
                block: 3,
                dir: Dir::Front,
            },
            Face {
                value: 15,
                long: 9,
                short: 4,
                block: 3,
                dir: Dir::Back,
            },
            Face {
                value: 27,
                long: 9,
                short: 3,
                block: 3,
                dir: Dir::Left,
            },
            Face {
                value: 9,
                long: 9,
                short: 3,
                block: 3,
                dir: Dir::Right,
            },
            Face {
                value: 3,
                long: 4,
                short: 3,
                block: 3,
                dir: Dir::Top,
            },
            Face {
                value: 54,
                long: 4,
                short: 3,
                block: 3,
                dir: Dir::Bottom,
            },
        ],
        label: "small_chunk",
    },
    Block {
        height: 9,
        width: 4,
        depth: 2,
        faces: [
            Face {
                value: 29,
                long: 9,
                short: 4,
                block: 4,
                dir: Dir::Front,
            },
            Face {
                value: 11,
                long: 9,
                short: 4,
                block: 4,
                dir: Dir::Back,
            },
            Face {
                value: 48,
                long: 9,
                short: 2,
                block: 4,
                dir: Dir::Left,
            },
            Face {
                value: 37,
                long: 9,
                short: 2,
                block: 4,
                dir: Dir::Right,
            },
            Face {
                value: 45,
                long: 4,
                short: 2,
                block: 4,
                dir: Dir::Top,
            },
            Face {
                value: 51,
                long: 4,
                short: 2,
                block: 4,
                dir: Dir::Bottom,
            },
        ],
        label: "smaller_chunk",
    },
    Block {
        height: 6,
        width: 5,
        depth: 4,
        faces: [
            Face {
                value: 6,
                long: 6,
                short: 5,
                block: 5,
                dir: Dir::Front,
            },
            Face {
                value: 23,
                long: 6,
                short: 5,
                block: 5,
                dir: Dir::Back,
            },
            Face {
                value: 4,
                long: 6,
                short: 4,
                block: 5,
                dir: Dir::Left,
            },
            Face {
                value: 50,
                long: 6,
                short: 4,
                block: 5,
                dir: Dir::Right,
            },
            Face {
                value: 19,
                long: 5,
                short: 4,
                block: 5,
                dir: Dir::Top,
            },
            Face {
                value: 32,
                long: 5,
                short: 4,
                block: 5,
                dir: Dir::Bottom,
            },
        ],
        label: "big_brick",
    },
    Block {
        height: 6,
        width: 4,
        depth: 4,
        faces: [
            Face {
                value: 1,
                long: 6,
                short: 4,
                block: 6,
                dir: Dir::Front,
            },
            Face {
                value: 40,
                long: 6,
                short: 4,
                block: 6,
                dir: Dir::Back,
            },
            Face {
                value: 13,
                long: 6,
                short: 4,
                block: 6,
                dir: Dir::Left,
            },
            Face {
                value: 25,
                long: 6,
                short: 4,
                block: 6,
                dir: Dir::Right,
            },
            Face {
                value: 52,
                long: 4,
                short: 4,
                block: 6,
                dir: Dir::Top,
            },
            Face {
                value: 46,
                long: 4,
                short: 4,
                block: 6,
                dir: Dir::Bottom,
            },
        ],
        label: "long_square",
    },
    Block {
        height: 5,
        width: 5,
        depth: 3,
        faces: [
            Face {
                value: 10,
                long: 5,
                short: 5,
                block: 7,
                dir: Dir::Front,
            },
            Face {
                value: 20,
                long: 5,
                short: 5,
                block: 7,
                dir: Dir::Back,
            },
            Face {
                value: 28,
                long: 5,
                short: 3,
                block: 7,
                dir: Dir::Left,
            },
            Face {
                value: 35,
                long: 5,
                short: 3,
                block: 7,
                dir: Dir::Right,
            },
            Face {
                value: 5,
                long: 5,
                short: 3,
                block: 7,
                dir: Dir::Top,
            },
            Face {
                value: 17,
                long: 5,
                short: 3,
                block: 7,
                dir: Dir::Bottom,
            },
        ],
        label: "short_square",
    },
    Block {
        height: 5,
        width: 4,
        depth: 3,
        faces: [
            Face {
                value: 39,
                long: 5,
                short: 4,
                block: 8,
                dir: Dir::Front,
            },
            Face {
                value: 42,
                long: 5,
                short: 4,
                block: 8,
                dir: Dir::Back,
            },
            Face {
                value: 21,
                long: 5,
                short: 3,
                block: 8,
                dir: Dir::Left,
            },
            Face {
                value: 2,
                long: 5,
                short: 3,
                block: 8,
                dir: Dir::Right,
            },
            Face {
                value: 26,
                long: 4,
                short: 3,
                block: 8,
                dir: Dir::Top,
            },
            Face {
                value: 12,
                long: 4,
                short: 3,
                block: 8,
                dir: Dir::Bottom,
            },
        ],
        label: "small_brick",
    },
];

lazy_static! {
    pub static ref FACES: Vec<&'static Face> = BLOCKS
        .iter()
        .map(|b| b.faces.each_ref())
        .collect_vec()
        .concat();
}
