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
    [1, 9, 35],
    [6, 14, 40],
    [3, 15, 31],
    [2, 16, 39],
    [8, 20, 36],
    [7, 21, 44],
    [13, 25, 41],
    [12, 26, 34],
    [19, 32, 55],
    [17, 38, 45],
    [24, 37, 46],
    [22, 43, 50],
    [29, 42, 51],
    [27, 33, 59],
    [18, 49, 56],
    [23, 47, 54],
    [28, 52, 58],
    [48, 53, 57],
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
