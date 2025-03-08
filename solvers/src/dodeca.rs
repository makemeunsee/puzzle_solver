use itertools::Itertools;
use rand::{rngs::SmallRng, seq::SliceRandom, RngCore};

// see graph.svg for the pentagons/triangles/facets arrangement

pub const PENTA0: [usize; 5] = [0, 2, 5, 4, 1];
pub const PENTA1: [usize; 5] = [0, 3, 7, 6, 2];
pub const PENTA2: [usize; 5] = [0, 1, 9, 8, 3];
pub const PENTA3: [usize; 5] = [4, 5, 11, 16, 10];
pub const PENTA4: [usize; 5] = [6, 7, 13, 17, 12];
pub const PENTA5: [usize; 5] = [8, 9, 15, 18, 14];
pub const PENTA6: [usize; 5] = [1, 4, 10, 15, 9];
pub const PENTA7: [usize; 5] = [2, 6, 12, 11, 5];
pub const PENTA8: [usize; 5] = [3, 8, 14, 13, 7];
pub const PENTA9: [usize; 5] = [11, 12, 17, 19, 16];
pub const PENTA10: [usize; 5] = [13, 14, 18, 19, 17];
pub const PENTA11: [usize; 5] = [10, 16, 19, 18, 15];

pub const PENTAS: [[usize; 5]; 12] = [
    PENTA0, PENTA1, PENTA2, PENTA3, PENTA4, PENTA5, PENTA6, PENTA7, PENTA8, PENTA9, PENTA10,
    PENTA11,
];

// for each triangle (icosa face), its facets
pub const TRI_TO_FACETS: [[usize; 3]; 20] = [
    [0, 5, 10],
    [4, 11, 30],
    [1, 35, 9],
    [6, 40, 14],
    [3, 31, 15],
    [2, 16, 39],
    [8, 36, 20],
    [7, 21, 44],
    [13, 41, 25],
    [12, 26, 34],
    [19, 32, 55],
    [17, 45, 38],
    [24, 37, 46],
    [22, 50, 43],
    [29, 42, 51],
    [27, 59, 33],
    [18, 56, 49],
    [23, 47, 54],
    [28, 52, 58],
    [48, 57, 53],
];

// equivalent to PENTAS; for each facet, its pentagon (dodeca face) and its triangle (icosa face)
pub const FACETS: [(usize, usize); 60] = [
    (0, 0),
    (0, 2),
    (0, 5),
    (0, 4),
    (0, 1),
    (1, 0),
    (1, 3),
    (1, 7),
    (1, 6),
    (1, 2),
    (2, 0),
    (2, 1),
    (2, 9),
    (2, 8),
    (2, 3),
    (3, 4),
    (3, 5),
    (3, 11),
    (3, 16),
    (3, 10),
    (4, 6),
    (4, 7),
    (4, 13),
    (4, 17),
    (4, 12),
    (5, 8),
    (5, 9),
    (5, 15),
    (5, 18),
    (5, 14),
    (6, 1),
    (6, 4),
    (6, 10),
    (6, 15),
    (6, 9),
    (7, 2),
    (7, 6),
    (7, 12),
    (7, 11),
    (7, 5),
    (8, 3),
    (8, 8),
    (8, 14),
    (8, 13),
    (8, 7),
    (9, 11),
    (9, 12),
    (9, 17),
    (9, 19),
    (9, 16),
    (10, 13),
    (10, 14),
    (10, 18),
    (10, 19),
    (10, 17),
    (11, 10),
    (11, 16),
    (11, 19),
    (11, 18),
    (11, 15),
];

pub const PENTAS_GRAPH: [[usize; 5]; 12] = [
    [1, 2, 6, 3, 7],
    [0, 7, 4, 8, 2],
    [0, 1, 8, 5, 6],
    [0, 6, 11, 9, 7],
    [1, 7, 9, 10, 8],
    [2, 8, 10, 11, 6],
    [0, 2, 5, 11, 3],
    [0, 3, 9, 4, 1],
    [1, 4, 10, 5, 2],
    [3, 11, 10, 4, 7],
    [4, 9, 11, 5, 8],
    [3, 6, 5, 10, 9],
];

// Helper function when rotating a tile and its neighbour tiles along.
// Given 2 neighbour facets of the 'pivot' tile, and a direction (could be inferred too),
// returns the ids of the neighbour tiles to rotate along and how to map their facets (offset to apply)
// e.g.: (i, j, offset) => each facet(id=k) of the source tile(id=i) maps to the facet(id=k+offset) of the target tile(id=j)
pub fn offset_to_rot(facet_from: usize, facet_to: usize, clockwise: bool) -> (usize, usize, usize) {
    let next_offset = if clockwise { 1 } else { 2 };

    let triangle_from = FACETS[facet_from].1;
    let facets_from = TRI_TO_FACETS[triangle_from];
    let facet_from_idx = facets_from.iter().position(|f| *f == facet_from).unwrap();
    let penta_facet_from = facets_from[(facet_from_idx + next_offset) % 3];

    let triangle_to = FACETS[facet_to].1;
    let facets_to = TRI_TO_FACETS[triangle_to];
    let facet_to_idx = facets_to.iter().position(|f| *f == facet_to).unwrap();
    let penta_facet_to = facets_to[(facet_to_idx + next_offset) % 3];

    (
        penta_facet_from / 5,
        penta_facet_to / 5,
        ((penta_facet_to % 5 + 5) - (penta_facet_from % 5)) % 5,
    )
}

pub fn triangles_to_pentas_shuffled(
    triplets: &[(i32, i32, i32); 20],
    rng: &mut SmallRng,
    rotation_shuffle: bool,
    position_shuffle: bool,
) -> [[i32; 5]; 12] {
    let triangles: [usize; 20] = (0..20).collect_array().unwrap();

    let mut facets = [0; 60];
    for tri in 0..20 {
        let facet = TRI_TO_FACETS[triangles[tri]];
        facets[facet[0]] = triplets[tri].0;
        facets[facet[1]] = triplets[tri].1;
        facets[facet[2]] = triplets[tri].2;
    }
    let pentas: [[i32; 5]; 12] = facets
        .into_iter()
        .chunks(5)
        .into_iter()
        .map(|x| x.collect_array().unwrap())
        .collect_array()
        .unwrap();
    let mut pentas_shuffled = pentas;
    if position_shuffle {
        pentas_shuffled.shuffle(rng);
    }
    pentas_shuffled = pentas_shuffled
        .into_iter()
        .map(|penta| {
            // biased but soooo negligibly
            let shift = if rotation_shuffle {
                rng.next_u32() as usize % 5
            } else {
                0
            };
            let mut result = [0; 5];
            for i in 0..5 {
                result[(i + shift) % 5] = penta[i];
            }
            result
        })
        .collect_array()
        .unwrap();
    pentas_shuffled
}
