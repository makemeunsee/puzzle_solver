use itertools::Itertools;
use solvers::dodeca::TRI_TO_FACETS;
use three_d::*;

#[derive(Debug, Eq, PartialEq)]
enum MaterialType {
    Normal,
    Color,
    Forward,
}

#[derive(Debug)]
pub struct Polyhedron {
    pub positions: Vec<Vector3<f32>>,
    pub indices: Vec<u16>,
}

// const GOLD: f64 = 1.618033988749895;
const GOLD: f32 = 1.618034;

const ICOSAHEDRON_VERTICES: [Vec3; 12] = [
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

const DODECAHEDRON_VERTICES: [Vec3; 20] = [
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
    pub fn ico_facet() -> Polyhedron {
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

fn number_rot(facet_idx: usize, text_idx: usize) -> Mat4 {
    // facets are visually rotated
    // by the `transformations_base`
    let rot_shift = match facet_idx {
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
        Polyhedron::ico_facet().positions[0].normalize(),
        degrees((rot_shift + text_idx as i32) as f32 * -72.),
    )
}

const COLOR_LIGHT_BLUE: Srgba = Srgba::new_opaque(100, 150, 255);
const COLOR_LIGHT_GOLD: Srgba = Srgba::new_opaque(220, 220, 150);
const COLOR_NEON_GREEN: Srgba = Srgba::new_opaque(100, 255, 100);
const COLOR_FIERY_RED: Srgba = Srgba::new_opaque(255, 50, 0);
const COLOR_GOLD: Srgba = Srgba::new_opaque(240, 160, 80);
const COLOR_GRAY_BROWN: Srgba = Srgba::new_opaque(94, 94, 80);

const COLOR_FACET_0: Srgba = Srgba::new_opaque(150, 90, 0);
const COLOR_FACET_BASE: Srgba = COLOR_GOLD;
const COLOR_FACET_PICK: Srgba = COLOR_GRAY_BROWN;
const COLOR_TEXT_PICK: Srgba = COLOR_NEON_GREEN;
const COLOR_TEXT_GOOD: Srgba = COLOR_LIGHT_GOLD;
const COLOR_TEXT_BAD: Srgba = Srgba::BLACK;

const STATIC_FACET_ID: usize = 11;

pub fn demo_3d(pentas: &[[i32; 5]; 12]) {
    let window = Window::new(WindowSettings {
        title: "Dodeca".to_string(),
        ..Default::default()
    })
    .unwrap();
    let context = window.gl();
    let mut camera = Camera::new_perspective(
        window.viewport(),
        vec3(4.0, 4.0, 8.0),
        vec3(0.0, 0.0, 0.0),
        vec3(0.0, 1.0, 0.0),
        degrees(45.0),
        0.1,
        100.0,
    );
    let mut control = FreeOrbitControl::new(camera.target(), 1.0, 50.0);
    let mut gui = three_d::GUI::new(&context);

    let dodeca_mesh = Polyhedron::regular_dodecahedron().into_mesh();
    let dodeca_mat = CpuMaterial {
        albedo: Srgba::WHITE,
        emissive: Srgba {
            r: 5,
            g: 5,
            b: 0,
            a: 255,
        },
        metallic: 0.6,
        roughness: 0.3,
        ..Default::default()
    };
    let mut dodeca = Gm::new(
        Mesh::new(&context, &dodeca_mesh),
        PhysicalMaterial::new(&context, &dodeca_mat),
    );
    dodeca.material.render_states.cull = Cull::Back;

    let facet_mesh = Polyhedron::ico_facet().into_mesh();

    let magic_rot_a = Mat4::new(
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
    let magic_rot_b = Mat4::new(
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
    let transformations_base = vec![
        Mat4::identity(),                                              // D0
        Mat4::from_angle_y(degrees(180.)),                             // D1
        Mat4::from_angle_x(degrees(180.)) * magic_rot_a * magic_rot_b, // D2
        Mat4::from_angle_z(degrees(180.)) * magic_rot_b,               // D3
        magic_rot_b,                                                   // D4
        Mat4::from_angle_y(degrees(180.)) * magic_rot_a * magic_rot_b, // D5
        Mat4::from_angle_y(degrees(180.)) * magic_rot_b,               // D6
        Mat4::from_angle_z(degrees(180.)) * magic_rot_a * magic_rot_b, // D7
        Mat4::from_angle_x(degrees(180.)) * magic_rot_b,               // D8
        magic_rot_a * magic_rot_b,                                     // D9
        Mat4::from_angle_z(degrees(180.)),                             // D10
        Mat4::from_angle_x(degrees(180.)),                             // D11
    ];
    let translation_base = Polyhedron::ico_facet().positions[0];

    let instances = Instances {
        transformations: transformations_base.clone(),
        colors: Some(vec![Srgba::GREEN; transformations_base.len()]),
        ..Default::default()
    };
    let mut instanced_facets = Gm::new(
        InstancedMesh::new(&context, &instances, &facet_mesh),
        PhysicalMaterial::new(&context, &dodeca_mat),
    );
    instanced_facets.material.render_states.cull = Cull::Back;

    let smaller = Mat4::from_scale(0.1);
    let facet_align =
        Mat4::from_axis_angle(Vec3::unit_x(), degrees(-69.1)) * Mat4::from_angle_z(degrees(-60.));
    let facet_center = (1. * Polyhedron::ico_facet().positions[0]
        + 2. * Polyhedron::ico_facet().positions[1])
        / 3.;
    let facet_translate = Mat4::from_translation(facet_center * 1.001);

    let mut numbers = vec![];
    let text_generator =
        TextGenerator::new(include_bytes!("OldEnglishFive_mod.ttf"), 0, 2.).unwrap();
    for (i, penta) in pentas.iter().enumerate() {
        for (j, v) in penta.iter().enumerate() {
            let text_mesh = text_generator.generate(
                &format!("{:0>2}\n\u{2009}_", v),
                TextLayoutOptions { line_height: 0.05 },
            );
            let (x_min, x_max, y_min, y_max, z_min, z_max) = text_mesh
                .positions
                .to_f32()
                .iter()
                .fold((1000., 0., 1000., 0., 1000., 0.), |mut acc, p| {
                    acc.0 = f32::min(acc.0, p.x);
                    acc.1 = f32::max(acc.1, p.x);
                    acc.2 = f32::min(acc.2, p.y);
                    acc.3 = f32::max(acc.3, p.y);
                    acc.4 = f32::min(acc.4, p.z);
                    acc.5 = f32::max(acc.5, p.z);
                    acc
                });
            let to_origin = Mat4::from_translation(Vec3::new(
                -(x_min + x_max) / 2.,
                -(y_min + y_max) / 2.,
                -(z_min + z_max) / 2.,
            ));
            let mut text = Gm::new(
                Mesh::new(&context, &text_mesh),
                ColorMaterial {
                    color: Srgba::BLACK,
                    ..Default::default()
                },
            );
            text.material.render_states.cull = Cull::Front;

            // matrix to put the number on the 1st face of a facet
            let pos_mat = facet_translate * facet_align * smaller * to_origin;
            // matrix to put the number from the 1st face of a facet to its proper face
            let rot_mat = number_rot(i, j);

            numbers.push((text, i, rot_mat, pos_mat, *v));
        }
    }

    let mut ambient = AmbientLight::new(&context, 0.2, Srgba::WHITE);
    let mut directional0 = DirectionalLight::new(
        &context,
        1.0,
        Srgba::new_opaque(255, 150, 0),
        vec3(0.0, -1.0, 0.0),
    );
    let mut directional1 = DirectionalLight::new(&context, 1.0, Srgba::WHITE, vec3(0.0, -1.0, 0.0));
    let mut directional2 = DirectionalLight::new(
        &context,
        1.0,
        Srgba::new_opaque(255, 192, 203),
        vec3(0.0, -1.0, 0.0),
    );
    let mut spot0 = SpotLight::new(
        &context,
        5.0,
        Srgba::new_opaque(220, 200, 180),
        vec3(0.0, 0.0, 0.0),
        vec3(0.0, -1.0, 0.0),
        degrees(25.0),
        Attenuation {
            constant: 0.1,
            linear: 0.001,
            quadratic: 0.0001,
        },
    );

    let mut puzzle_state: [i32; 60] = pentas
        .iter()
        .flat_map(|penta| *penta)
        .collect_array()
        .unwrap();

    let mut show_dodeca = false;
    let mut trans_factor = 0.05;
    let mut facet_anim_speed = 10.0;
    let mut material_type = MaterialType::Forward;

    let mut time_d0 = 0.;
    let mut time_d1 = 0.;
    let mut time_d2 = 0.;
    let mut time_s0 = 0.;
    // let mut time_p0 = 0.;
    // let mut time_p1 = 0.;
    let mut speed_d0 = 0;
    let mut speed_d1 = 0;
    let mut speed_d2 = 0;
    let mut speed_s0 = 0;
    // let mut speed_p0 = 3;
    // let mut speed_p1 = 3;

    let mut picked_facet_id = None;
    let mut rotating = [None; 12];
    let mut swapping = None;
    let mut pressed_on = None;

    let mut facet_colors = vec![COLOR_FACET_BASE; transformations_base.len()];
    facet_colors[STATIC_FACET_ID] = COLOR_FACET_0;
    let mut change = true;

    window.render_loop(move |mut frame_input| {
        let mut panel_width = 0.0;
        gui.update(
            &mut frame_input.events,
            frame_input.accumulated_time,
            frame_input.viewport,
            frame_input.device_pixel_ratio,
            |gui_context| {
                use three_d::egui::*;
                SidePanel::left("side_panel").show(gui_context, |ui| {
                    use three_d::egui::*;
                    ui.add_space(50.);
                    ui.heading("Control Panel");

                    ui.add(three_d::egui::Separator::default());

                    ui.checkbox(&mut show_dodeca, "Show dodecahedron");
                    ui.add(Slider::new(&mut trans_factor, -2.5..=2.5).text("Facet break out"));
                    ui.add(
                        Slider::new(&mut facet_anim_speed, 1.0..=20.0)
                            .text("Facet animation speed"),
                    );

                    ui.add(three_d::egui::Separator::default());

                    ui.label("Light options");
                    ui.add(
                        Slider::new(&mut ambient.intensity, 0.0..=1.0).text("Ambient intensity"),
                    );
                    ui.add(
                        Slider::new(&mut directional0.intensity, 0.0..=1.0)
                            .text("Directional 0 intensity"),
                    );
                    ui.add(Slider::new(&mut speed_d0, 0..=10).text("Directional 0 speed"));
                    ui.add(
                        Slider::new(&mut directional1.intensity, 0.0..=1.0)
                            .text("Directional 1 intensity"),
                    );
                    ui.add(Slider::new(&mut speed_d1, 0..=10).text("Directional 1 speed"));
                    ui.add(
                        Slider::new(&mut directional2.intensity, 0.0..=1.0)
                            .text("Directional 2 intensity"),
                    );
                    ui.add(Slider::new(&mut speed_d2, 0..=10).text("Directional 2 speed"));
                    ui.add(Slider::new(&mut spot0.intensity, 0.0..=10.0).text("Spot intensity"));
                    ui.add(Slider::new(&mut speed_s0, 0..=10).text("Spot speed"));
                    // ui.add(Slider::new(&mut point0.intensity, 0.0..=1.0).text("Point 0 intensity"));
                    // ui.add(Slider::new(&mut speed_p0, 0..=10).text("Point 0 speed"));
                    // ui.add(Slider::new(&mut point1.intensity, 0.0..=1.0).text("Point 1 intensity"));
                    // ui.add(Slider::new(&mut speed_p1, 0..=10).text("Point 1 speed"));

                    ui.add(three_d::egui::Separator::default());

                    ui.label("Material options");
                    ui.radio_value(&mut material_type, MaterialType::Forward, "Forward");
                    ui.radio_value(&mut material_type, MaterialType::Normal, "Normal");
                    ui.radio_value(&mut material_type, MaterialType::Color, "Color");
                });
                panel_width = gui_context.used_rect().width();
            },
        );
        let viewport = Viewport {
            x: (panel_width * frame_input.device_pixel_ratio) as i32,
            y: 0,
            width: frame_input.viewport.width
                - (panel_width * frame_input.device_pixel_ratio) as u32,
            height: frame_input.viewport.height,
        };
        camera.set_viewport(viewport);

        let mut pick_id = None;

        for event in frame_input.events.iter() {
            // track left click presses to identify dragging movements
            if let Event::MousePress {
                button, position, ..
            } = *event
            {
                if button == MouseButton::Left {
                    pressed_on = Some((position.x, position.y));
                }
            }
            // maybe pick
            if let Event::MouseRelease {
                button, position, ..
            } = *event
            {
                // pick only if not a dragging movement
                if button == MouseButton::Left {
                    let moved = if let Some((x, y)) = pressed_on {
                        let delta_x = position.x - x;
                        let delta_y = position.y - y;
                        delta_x * delta_x + delta_y * delta_y > 50.
                    } else {
                        false
                    };
                    pressed_on = None;
                    if !moved {
                        if let Some(pick) = pick(&context, &camera, position, &instanced_facets) {
                            match pick.geometry_id {
                                0 => pick_id = Some(pick.instance_id as usize),
                                _ => unreachable!(),
                            };
                        } else {
                            // picked out -> unpick current
                            if let Some(id) = picked_facet_id {
                                picked_facet_id = None;
                                facet_colors[id] = COLOR_FACET_BASE;
                                change = true;
                            }
                        }
                    }
                }
            }
        }

        // a geometry was picked
        if let Some(pick_id) = pick_id {
            if pick_id != STATIC_FACET_ID {
                picked_facet_id = match picked_facet_id {
                    // picked the same facet -> rotate it
                    Some(id) if id == pick_id => {
                        if (rotating[id] as Option<f32>).is_none() {
                            rotating[id] = Some(0.);
                        }
                        Some(id)
                    }

                    // picked the another facet -> swap them
                    Some(id) => {
                        if swapping.is_none() {
                            swapping = Some((id, pick_id, 0.));
                            change = true;
                        }
                        Some(id)
                    }

                    // picked a new facet
                    None => {
                        change = true;
                        Some(pick_id)
                    }
                };
            }
        }

        // animate the rotations
        for (i, rot_opt) in rotating.iter_mut().enumerate() {
            match rot_opt {
                None => (),
                Some(rot) => {
                    let next_rot = f32::min(
                        72.0,
                        *rot + (7.2 * frame_input.elapsed_time as f32 * facet_anim_speed / 200.0),
                    );
                    *rot_opt = if next_rot == 72. {
                        None
                    } else {
                        Some(next_rot)
                    };

                    if next_rot == 72.0 {
                        let offset = i * 5;
                        for j in 0..4 {
                            let new_j = (j + 1) % 5;
                            let offset0 = offset + j;
                            let offset1 = offset + new_j;
                            puzzle_state.swap(offset0, offset1);
                            numbers.swap(offset0, offset1);
                            let tmp = numbers[offset0].2;
                            numbers[offset0].2 = numbers[offset1].2;
                            numbers[offset1].2 = tmp;
                        }
                        change = true;
                    }
                }
            }
        }

        // animate the swapping
        if let Some((picked_id, o_id, prog)) = swapping {
            let next_prog = f32::min(
                100.,
                prog + frame_input.elapsed_time as f32 * facet_anim_speed / 40.,
            );
            if next_prog >= 50. && prog < 50. {
                let offset = picked_id * 5;
                let o_offset = o_id * 5;
                for j in 0..5 {
                    let offset0 = offset + j;
                    let offset1 = o_offset + j;
                    puzzle_state.swap(offset0, offset1);
                    numbers.swap(offset0, offset1);
                    let tmp = numbers[offset0].1;
                    numbers[offset0].1 = numbers[offset1].1;
                    numbers[offset1].1 = tmp;
                    let tmp = numbers[offset0].2;
                    numbers[offset0].2 = numbers[offset1].2;
                    numbers[offset1].2 = tmp;
                }
                swapping = Some((o_id, picked_id, next_prog));
                picked_facet_id = Some(o_id);
                change = true;
            } else if next_prog == 100. {
                swapping = None;
            } else {
                swapping = Some((picked_id, o_id, next_prog));
            }
        }

        if change {
            let mut win = true;
            for [a, b, c] in TRI_TO_FACETS {
                if puzzle_state[a] + puzzle_state[b] + puzzle_state[c] != 96 {
                    numbers[a].0.material.color = COLOR_TEXT_BAD;
                    numbers[b].0.material.color = COLOR_TEXT_BAD;
                    numbers[c].0.material.color = COLOR_TEXT_BAD;

                    win = false;
                } else {
                    numbers[a].0.material.color = COLOR_TEXT_GOOD;
                    numbers[b].0.material.color = COLOR_TEXT_GOOD;
                    numbers[c].0.material.color = COLOR_TEXT_GOOD;
                }
            }
            if let Some((_, o_id, _)) = swapping {
                facet_colors[o_id] = COLOR_FACET_BASE;
            }
            if let Some(id) = picked_facet_id {
                pick_number_color(id, &mut numbers);
                facet_colors[id] = COLOR_FACET_PICK;
            }
            if win {
                // TODO
            }
            change = false;
        }

        control.handle_events(&mut camera, &mut frame_input.events);

        time_d0 += (speed_d0 * speed_d0) as f32 * 0.0001 * frame_input.elapsed_time as f32;
        let c = time_d0.cos();
        let s = time_d0.sin();
        directional0.direction = vec3(-1.0 - c, -1.0, 1.0 + s);
        time_d1 += (speed_d1 * speed_d1) as f32 * 0.0001 * frame_input.elapsed_time as f32;
        let c = time_d1.cos();
        let s = time_d1.sin();
        directional1.direction = vec3(1.0 + c, -1.0, -1.0 - s);
        time_d2 += (speed_d2 * speed_d2) as f32 * 0.0001 * frame_input.elapsed_time as f32;
        let c = time_d2.cos();
        let s = time_d2.sin();
        directional2.direction = vec3(-1.0 + c, 1.0, 1.0 - s);
        time_s0 += (speed_s0 * speed_s0) as f32 * 0.0001 * frame_input.elapsed_time as f32;
        let c = time_s0.cos();
        let s = time_s0.sin();
        spot0.position = vec3(3.0 + c, 5.0 + s, 3.0 - s);
        spot0.direction = -vec3(3.0 + c, 5.0 + s, 3.0 - s);

        let transformations = transformations_base
            .iter()
            .enumerate()
            .map(|(i, mat)| {
                let rot_mat = if let Some(rot) = rotating[i] {
                    Mat4::from_axis_angle(translation_base.normalize(), degrees(rot))
                } else {
                    Mat4::identity()
                };

                let trans_factor = trans_factor
                    + match swapping {
                        Some((id, o_id, prog)) if i == id || i == o_id => {
                            if prog < 25. {
                                0.005 * prog
                            } else if prog > 75. {
                                0.005 * (100. - prog)
                            } else {
                                0.125
                            }
                        }
                        _ => 0.,
                    };

                mat * rot_mat * Mat4::from_translation(translation_base * trans_factor)
            })
            .collect_vec();

        for (number, i, rot_mat, pos_mat, _) in numbers.iter_mut() {
            number.set_transformation(transformations[*i] * *rot_mat * *pos_mat);
        }

        instanced_facets.set_instances(&Instances {
            transformations,
            colors: Some(facet_colors.clone()),
            ..Default::default()
        });

        // Draw

        directional0.generate_shadow_map(1024, &instanced_facets);
        directional1.generate_shadow_map(1024, &instanced_facets);
        directional2.generate_shadow_map(1024, &instanced_facets);
        spot0.generate_shadow_map(1024, &instanced_facets);

        let lights = [
            &ambient as &dyn Light,
            &spot0,
            &directional0,
            &directional1,
            &directional2,
            // &point0,
            // &point1,
        ];

        let screen = frame_input.screen();
        screen.clear(ClearState::default());

        match material_type {
            MaterialType::Normal => {
                screen
                    .write::<RendererError>(|| {
                        if show_dodeca {
                            dodeca.render_with_material(
                                &NormalMaterial::from_physical_material(&dodeca.material),
                                &camera,
                                &lights,
                            );
                        }
                        instanced_facets.render_with_material(
                            &NormalMaterial::from_physical_material(&instanced_facets.material),
                            &camera,
                            &lights,
                        );
                        Ok(())
                    })
                    .unwrap();
            }
            MaterialType::Color => {
                screen
                    .write::<RendererError>(|| {
                        if show_dodeca {
                            dodeca.render_with_material(
                                &ColorMaterial::from_physical_material(&dodeca.material),
                                &camera,
                                &lights,
                            );
                        }
                        instanced_facets.render_with_material(
                            &ColorMaterial::from_physical_material(&instanced_facets.material),
                            &camera,
                            &lights,
                        );
                        Ok(())
                    })
                    .unwrap();
            }
            MaterialType::Forward => {
                screen
                    .write::<RendererError>(|| {
                        if show_dodeca {
                            dodeca.render(&camera, &lights);
                        }
                        instanced_facets.render(&camera, &lights);
                        for number in &numbers {
                            number.0.render(&camera, &[]);
                        }
                        Ok(())
                    })
                    .unwrap();
            }
        }

        screen.write(|| gui.render()).unwrap();

        FrameOutput::default()
    });
}

fn pick_number_color(
    picked: usize,
    numbers: &mut [(
        Gm<Mesh, ColorMaterial>,
        usize,
        Matrix4<f32>,
        Matrix4<f32>,
        i32,
    )],
) {
    for num in numbers.iter_mut().skip(picked * 5).take(5) {
        num.0.material.color = COLOR_TEXT_PICK;
    }
}
