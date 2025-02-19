use itertools::Itertools;
use lazy_static::lazy_static;
use three_d::{
    degrees, CpuMesh, Indices, InnerSpace, Mat4, Positions, SquareMatrix, Vec3, Vector3,
};

#[derive(Debug)]
pub struct Polyhedron {
    pub positions: Vec<Vector3<f32>>,
    pub indices: Vec<u16>,
}

// const GOLD: f64 = 1.618033988749895;
const GOLD: f32 = 1.618034;

const MAGIC_ROT_A: Mat4 = Mat4::new(
    (GOLD - 1.) / 2.,
    GOLD / 2.,
    0.5,
    0.,
    -GOLD / 2.,
    0.5,
    (1. - GOLD) / 2.,
    0.,
    -0.5,
    (1. - GOLD) / 2.,
    GOLD / 2.,
    0.,
    0.,
    0.,
    0.,
    1.,
);

const MAGIC_ROT_B: Mat4 = Mat4::new(
    (1. - GOLD) / 2.,
    GOLD / 2.,
    -0.5,
    0.,
    -GOLD / 2.,
    -0.5,
    (1. - GOLD) / 2.,
    0.,
    -0.5,
    (GOLD - 1.) / 2.,
    GOLD / 2.,
    0.,
    0.,
    0.,
    0.,
    1.,
);

pub const ICOSAHEDRON_VERTICES: [Vec3; 12] = [
    Vector3::new(1.0, GOLD, 0.0),   // D0
    Vector3::new(-1.0, GOLD, 0.0),  // D1
    Vector3::new(0.0, 1.0, GOLD),   // D2
    Vector3::new(GOLD, 0.0, -1.0),  // D3
    Vector3::new(-GOLD, 0.0, -1.0), // D4
    Vector3::new(0.0, -1.0, GOLD),  // D5
    Vector3::new(GOLD, 0.0, 1.0),   // D6
    Vector3::new(0.0, 1.0, -GOLD),  // D7
    Vector3::new(-GOLD, 0.0, 1.0),  // D8
    Vector3::new(0.0, -1.0, -GOLD), // D9
    Vector3::new(-1.0, -GOLD, 0.0), // D10
    Vector3::new(1.0, -GOLD, 0.0),  // D11
];

pub const DODECAHEDRON_VERTICES: [Vec3; 20] = [
    Vector3::new(0.0, GOLD, 1.0 / GOLD),
    Vector3::new(1.0, 1.0, 1.0),
    Vector3::new(0.0, GOLD, -1.0 / GOLD),
    Vector3::new(-1.0, 1.0, 1.0),
    Vector3::new(GOLD, 1.0 / GOLD, 0.0),
    Vector3::new(1.0, 1.0, -1.0),
    Vector3::new(-1.0, 1.0, -1.0),
    Vector3::new(-GOLD, 1.0 / GOLD, 0.0),
    Vector3::new(-1.0 / GOLD, 0.0, GOLD),
    Vector3::new(1.0 / GOLD, 0.0, GOLD),
    Vector3::new(GOLD, -1.0 / GOLD, 0.0),
    Vector3::new(1.0 / GOLD, 0.0, -GOLD),
    Vector3::new(-1.0 / GOLD, 0.0, -GOLD),
    Vector3::new(-GOLD, -1.0 / GOLD, 0.0),
    Vector3::new(-1.0, -1.0, 1.0),
    Vector3::new(1.0, -1.0, 1.0),
    Vector3::new(1.0, -1.0, -1.0),
    Vector3::new(-1.0, -1.0, -1.0),
    Vector3::new(0.0, -GOLD, 1.0 / GOLD),
    Vector3::new(0.0, -GOLD, -1.0 / GOLD),
];

impl Polyhedron {
    pub fn ico_tile() -> Polyhedron {
        let triangle_center_0 =
            (ICOSAHEDRON_VERTICES[0] + ICOSAHEDRON_VERTICES[1] + ICOSAHEDRON_VERTICES[2]) / 3.;
        let triangle_center_1 =
            (ICOSAHEDRON_VERTICES[0] + ICOSAHEDRON_VERTICES[6] + ICOSAHEDRON_VERTICES[2]) / 3.;
        let triangle_center_2 =
            (ICOSAHEDRON_VERTICES[0] + ICOSAHEDRON_VERTICES[6] + ICOSAHEDRON_VERTICES[3]) / 3.;
        let triangle_center_3 =
            (ICOSAHEDRON_VERTICES[0] + ICOSAHEDRON_VERTICES[7] + ICOSAHEDRON_VERTICES[3]) / 3.;
        let triangle_center_4 =
            (ICOSAHEDRON_VERTICES[0] + ICOSAHEDRON_VERTICES[7] + ICOSAHEDRON_VERTICES[1]) / 3.;

        let vertices = [
            ICOSAHEDRON_VERTICES[0],
            triangle_center_0,
            (ICOSAHEDRON_VERTICES[0] + ICOSAHEDRON_VERTICES[2]) / 2.,
            triangle_center_1,
            (ICOSAHEDRON_VERTICES[0] + ICOSAHEDRON_VERTICES[6]) / 2.,
            triangle_center_2,
            (ICOSAHEDRON_VERTICES[0] + ICOSAHEDRON_VERTICES[3]) / 2.,
            triangle_center_3,
            (ICOSAHEDRON_VERTICES[0] + ICOSAHEDRON_VERTICES[7]) / 2.,
            triangle_center_4,
            (ICOSAHEDRON_VERTICES[0] + ICOSAHEDRON_VERTICES[1]) / 2.,
        ];

        let ideal_indices = [
            0, 1, 2, //
            0, 2, 3, //
            0, 3, 4, //
            0, 4, 5, //
            0, 5, 6, //
            0, 6, 7, //
            0, 7, 8, //
            0, 8, 9, //
            0, 9, 10, //
            0, 10, 1, //
            2, 1, 3, //
            4, 3, 5, //
            6, 5, 7, //
            8, 7, 9, //
            10, 9, 1, //
            1, 5, 3, //
            1, 7, 5, //
            1, 9, 7, //
        ];

        let l = ideal_indices.len() as u16;

        Polyhedron {
            positions: ideal_indices.into_iter().map(|i| vertices[i]).collect_vec(),
            indices: (0..l).collect_vec(),
        }
    }

    pub fn regular_isocahedron() -> Polyhedron {
        let ideal_indices = vec![
            0, 1, 2, //
            0, 2, 6, //
            0, 7, 1, //
            1, 8, 2, //
            0, 6, 3, //
            0, 3, 7, //
            1, 7, 4, //
            1, 4, 8, //
            2, 8, 5, //
            2, 5, 6, //
            3, 6, 11, //
            3, 9, 7, //
            4, 7, 9, //
            4, 10, 8, //
            5, 8, 10, //
            5, 11, 6, //
            3, 11, 9, //
            4, 9, 10, //
            5, 10, 11, //
            9, 11, 10, //
        ];

        let l = ideal_indices.len() as u16;

        Polyhedron {
            positions: ideal_indices
                .into_iter()
                .map(|i| ICOSAHEDRON_VERTICES[i])
                .collect_vec(),
            indices: (0..l).collect_vec(),
        }
    }

    pub fn regular_dodecahedron() -> Polyhedron {
        let ideal_indices = [
            0, 1, 4, 0, 4, 5, 0, 5, 2, //
            0, 2, 6, 0, 6, 7, 0, 7, 3, //
            0, 3, 8, 0, 8, 9, 0, 9, 1, //
            4, 10, 16, 4, 16, 11, 4, 11, 5, //
            6, 12, 17, 6, 17, 13, 6, 13, 7, //
            8, 14, 18, 8, 18, 15, 8, 15, 9, //
            1, 9, 15, 1, 15, 10, 1, 10, 4, //
            2, 5, 11, 2, 11, 12, 2, 12, 6, //
            3, 7, 13, 3, 13, 14, 3, 14, 8, //
            11, 16, 19, 11, 19, 17, 11, 17, 12, //
            13, 17, 19, 13, 19, 18, 13, 18, 14, //
            10, 15, 18, 10, 18, 19, 10, 19, 16, //
        ];

        let l = ideal_indices.len() as u16;

        Polyhedron {
            positions: ideal_indices
                .into_iter()
                .map(|i| DODECAHEDRON_VERTICES[i])
                .collect_vec(),
            indices: (0..l).collect_vec(),
        }
    }

    pub fn into_mesh(self) -> CpuMesh {
        let mut mesh = CpuMesh {
            positions: Positions::F32(self.positions),
            indices: Indices::U16(self.indices),
            ..Default::default()
        };
        mesh.compute_normals();
        mesh
    }
}

pub const ICO_TILE_COUNT: usize = 12;

// transformations to tile ico tiles into an ico
lazy_static! {
    pub static ref TRANSFORMATIONS_BASE: [Mat4;ICO_TILE_COUNT]=[
        Mat4::identity(),                                              // D0
        Mat4::from_angle_y(degrees(180.)),                             // D1
        Mat4::from_angle_x(degrees(180.)) * MAGIC_ROT_A * MAGIC_ROT_B, // D2
        Mat4::from_angle_z(degrees(180.)) * MAGIC_ROT_B,               // D3
        MAGIC_ROT_B,                                                   // D4
        Mat4::from_angle_y(degrees(180.)) * MAGIC_ROT_A * MAGIC_ROT_B, // D5
        Mat4::from_angle_y(degrees(180.)) * MAGIC_ROT_B,               // D6
        Mat4::from_angle_z(degrees(180.)) * MAGIC_ROT_A * MAGIC_ROT_B, // D7
        Mat4::from_angle_x(degrees(180.)) * MAGIC_ROT_B,               // D8
        MAGIC_ROT_A * MAGIC_ROT_B,                                     // D9
        Mat4::from_angle_z(degrees(180.)),                             // D10
        Mat4::from_angle_x(degrees(180.)),                             // D11
    ];
}

lazy_static! {
    pub static ref TILE0_FACET0_CENTER: Vec3 = Polyhedron::ico_tile().positions[0];
}

// further transformation to apply to move to the a particular facet of a tile
pub fn facet_shift_rotation(tile_idx: usize, facet_idx: usize) -> Mat4 {
    // facets are visually rotated
    // by the `TRANSFORMATIONS_BASE`
    let rot_shift = match tile_idx {
        1 => 1,
        4 => -2,
        5 => 2,
        6 => -2,
        8 => 1,
        9 => 2,
        10 => -2,
        11 => -2,
        _ => 0,
    };
    Mat4::from_axis_angle(
        Polyhedron::ico_tile().positions[0].normalize(),
        degrees((rot_shift + facet_idx as i32) as f32 * -72.),
    )
}
