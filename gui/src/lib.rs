use itertools::Itertools;
use three_d::*;

#[derive(Debug, Eq, PartialEq)]
enum MaterialType {
    // Position,
    Normal,
    Color,
    // Depth,
    // Orm,
    // Uv,
    Forward,
    // Deferred,
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
            (ICOSAHEDRON_VERTICES[8] + ICOSAHEDRON_VERTICES[1] + ICOSAHEDRON_VERTICES[2]) / 3.;
        let triangle_center_2 =
            (ICOSAHEDRON_VERTICES[8] + ICOSAHEDRON_VERTICES[5] + ICOSAHEDRON_VERTICES[2]) / 3.;
        let triangle_center_3 =
            (ICOSAHEDRON_VERTICES[6] + ICOSAHEDRON_VERTICES[5] + ICOSAHEDRON_VERTICES[2]) / 3.;
        let triangle_center_4 =
            (ICOSAHEDRON_VERTICES[6] + ICOSAHEDRON_VERTICES[0] + ICOSAHEDRON_VERTICES[2]) / 3.;

        let vertices = [
            ICOSAHEDRON_VERTICES[2],
            triangle_center_0,
            (ICOSAHEDRON_VERTICES[1] + ICOSAHEDRON_VERTICES[2]) / 2.,
            triangle_center_1,
            (ICOSAHEDRON_VERTICES[8] + ICOSAHEDRON_VERTICES[2]) / 2.,
            triangle_center_2,
            (ICOSAHEDRON_VERTICES[5] + ICOSAHEDRON_VERTICES[2]) / 2.,
            triangle_center_3,
            (ICOSAHEDRON_VERTICES[6] + ICOSAHEDRON_VERTICES[2]) / 2.,
            triangle_center_4,
            (ICOSAHEDRON_VERTICES[0] + ICOSAHEDRON_VERTICES[2]) / 2.,
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
        // mesh.compute_uv();
        // mesh.compute_tangents();
    }
}

pub fn demo_3d() {
    let window = Window::new(WindowSettings {
        title: "Shapes!".to_string(),
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
    let mut control = OrbitControl::new(camera.target(), 1.0, 50.0);
    let mut gui = three_d::GUI::new(&context);

    let mut numbers = vec![];
    let text_generator = TextGenerator::new(include_bytes!("OldEnglishFive.ttf"), 0, 2.).unwrap();
    for i in 0..=65 {
        let text_mesh =
            text_generator.generate(&format!("{:0>2}", i + 1), TextLayoutOptions::default());
        let mut text = Gm::new(
            Mesh::new(&context, &text_mesh),
            ColorMaterial {
                color: Srgba::BLACK,
                ..Default::default()
            },
        );
        text.material.render_states.cull = Cull::Front;
        numbers.push(text);
    }

    // let mut pbox = Gm::new(
    //     Mesh::new(&context, &CpuMesh::cube()),
    //     PhysicalMaterial::new(&context, &CpuMaterial::default()),
    // );
    // let bounding_box = Gm::new(
    //     BoundingBox::new_with_thickness(&context, pbox.aabb(), 0.1),
    //     ColorMaterial {
    //         color: Srgba::BLACK,
    //         ..Default::default()
    //     },
    // );

    let dodeca_mesh = Polyhedron::regular_dodecahedron().into_mesh();
    let dodeca_mat = CpuMaterial {
        albedo: Srgba {
            r: 240,
            g: 160,
            b: 80,
            a: 255,
        },
        // emissive: Srgba {
        //     r: 20,
        //     g: 20,
        //     b: 0,
        //     a: 255,
        // },
        metallic: 0.8,
        roughness: 0.3,
        ..Default::default()
    };
    let mut dodeca = Gm::new(
        Mesh::new(&context, &dodeca_mesh),
        PhysicalMaterial::new(&context, &dodeca_mat),
    );
    dodeca.material.render_states.cull = Cull::Back;

    // let ico_mesh = Polyhedron::regular_isocahedron().into_mesh();
    // let ico_mat = CpuMaterial {
    //     albedo: Srgba {
    //         r: 80,
    //         g: 240,
    //         b: 160,
    //         a: 255,
    //     },
    //     metallic: 0.8,
    //     roughness: 0.3,
    //     ..Default::default()
    // };
    // let mut ico = Gm::new(
    //     Mesh::new(&context, &ico_mesh),
    //     PhysicalMaterial::new(&context, &ico_mat),
    // );
    // ico.material.render_states.cull = Cull::Back;

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
    let magic_rot_b = magic_rot_a
        * Mat4::new(
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
        Mat4::identity(),
        Mat4::from_angle_y(degrees(180.)),
        Mat4::from_angle_x(degrees(180.)),
        Mat4::from_angle_y(degrees(180.)) * Mat4::from_angle_x(degrees(180.)),
        magic_rot_a,
        Mat4::from_angle_y(degrees(180.)) * magic_rot_a,
        Mat4::from_angle_x(degrees(180.)) * magic_rot_a,
        Mat4::from_angle_y(degrees(180.)) * Mat4::from_angle_x(degrees(180.)) * magic_rot_a,
        magic_rot_b,
        Mat4::from_angle_y(degrees(180.)) * magic_rot_b,
        Mat4::from_angle_x(degrees(180.)) * magic_rot_b,
        Mat4::from_angle_y(degrees(180.)) * Mat4::from_angle_x(degrees(180.)) * magic_rot_b,
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

    let mut cpu_plane = CpuMesh::square();
    cpu_plane
        .transform(
            Mat4::from_translation(vec3(0.0, -GOLD, 0.0))
                * Mat4::from_scale(10.0)
                * Mat4::from_angle_x(degrees(-90.0)),
        )
        .unwrap();
    let mut plane = Gm::new(
        Mesh::new(&context, &cpu_plane),
        PhysicalMaterial::new_opaque(
            &context,
            &CpuMaterial {
                albedo: Srgba::new_opaque(128, 200, 70),
                ..Default::default()
            },
        ),
    );
    plane.material.render_states.cull = Cull::Back;

    let mut ambient = AmbientLight::new(&context, 0.2, Srgba::WHITE);
    let mut directional0 = DirectionalLight::new(&context, 1.0, Srgba::RED, vec3(0.0, -1.0, 0.0));
    let mut directional1 = DirectionalLight::new(&context, 1.0, Srgba::GREEN, vec3(0.0, -1.0, 0.0));
    let mut spot0 = SpotLight::new(
        &context,
        5.0,
        Srgba::BLUE,
        vec3(0.0, 0.0, 0.0),
        vec3(0.0, -1.0, 0.0),
        degrees(25.0),
        Attenuation {
            constant: 0.1,
            linear: 0.001,
            quadratic: 0.0001,
        },
    );
    let mut point0 = PointLight::new(
        &context,
        1.0,
        Srgba::GREEN,
        vec3(0.0, 0.0, 0.0),
        Attenuation {
            constant: 0.5,
            linear: 0.05,
            quadratic: 0.005,
        },
    );
    let mut point1 = PointLight::new(
        &context,
        1.0,
        Srgba::RED,
        vec3(0.0, 0.0, 0.0),
        Attenuation {
            constant: 0.5,
            linear: 0.05,
            quadratic: 0.005,
        },
    );
    // will need 20
    // let colors = HashMap::from([
    //     (0, (255, 0, 0)),
    //     (1, (0, 255, 0)),
    //     (2, (0, 0, 255)),
    //     (3, (255, 255, 0)),
    //     (4, (255, 0, 255)),
    //     (5, (0, 255, 255)),
    //     (6, (110, 110, 110)),
    //     (7, (255, 127, 0)),
    //     (8, (160, 80, 0)),
    // ]);

    // let mut show_numbers = true;

    let mut shadows_enabled = true;
    let mut show_plane = true;
    let mut show_dodeca = false;
    let mut trans_factor = 0.02;
    let mut facet_rot = 0.0;
    let mut material_type = MaterialType::Forward;

    let mut time_d0 = 0.;
    let mut time_d1 = 0.;
    let mut time_s0 = 0.;
    let mut time_p0 = 0.;
    let mut time_p1 = 0.;
    let mut speed_d0 = 3;
    let mut speed_d1 = 3;
    let mut speed_s0 = 3;
    let mut speed_p0 = 3;
    let mut speed_p1 = 3;

    let mut picked_facet_id = None;

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

                    // ui.add(Checkbox::new(&mut show_numbers, "Show numbers"));

                    ui.checkbox(&mut show_dodeca, "Show dodecahedron");
                    if ui.checkbox(&mut show_plane, "Show plane").clicked() && !show_plane {
                        shadows_enabled = false;
                    }
                    if ui
                        .add_enabled(show_plane, Checkbox::new(&mut shadows_enabled, "Shadows"))
                        .clicked()
                        && !shadows_enabled
                    {
                        spot0.clear_shadow_map();
                        directional0.clear_shadow_map();
                        directional1.clear_shadow_map();
                    }
                    ui.add(Slider::new(&mut trans_factor, -2.5..=2.5).text("Facet break out"));
                    ui.add(Slider::new(&mut facet_rot, 0.0..=50.0).text("Facet rotation"));

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
                    ui.add(Slider::new(&mut spot0.intensity, 0.0..=10.0).text("Spot intensity"));
                    ui.add(Slider::new(&mut speed_s0, 0..=10).text("Spot speed"));
                    ui.add(Slider::new(&mut point0.intensity, 0.0..=1.0).text("Point 0 intensity"));
                    ui.add(Slider::new(&mut speed_p0, 0..=10).text("Point 0 speed"));
                    ui.add(Slider::new(&mut point1.intensity, 0.0..=1.0).text("Point 1 intensity"));
                    ui.add(Slider::new(&mut speed_p1, 0..=10).text("Point 1 speed"));

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

        for event in frame_input.events.iter() {
            if let Event::MousePress {
                button, position, ..
            } = *event
            {
                if button == MouseButton::Left {
                    if let Some(pick) = pick(&context, &camera, position, &instanced_facets) {
                        match pick.geometry_id {
                            0 => {
                                picked_facet_id = Some(pick.instance_id);
                            }
                            _ => {
                                unreachable!()
                            }
                        };
                    }
                }
            }
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
        time_s0 += (speed_s0 * speed_s0) as f32 * 0.0001 * frame_input.elapsed_time as f32;
        let c = time_s0.cos();
        let s = time_s0.sin();
        spot0.position = vec3(3.0 + c, 5.0 + s, 3.0 - s);
        spot0.direction = -vec3(3.0 + c, 5.0 + s, 3.0 - s);
        time_p0 += (speed_p0 * speed_p0) as f32 * 0.0001 * frame_input.elapsed_time as f32;
        let c = time_p0.cos();
        let s = time_p0.sin();
        point0.position = vec3(-5.0 * c, 5.0, -5.0 * s);
        time_p1 += (speed_p1 * speed_p1) as f32 * 0.0001 * frame_input.elapsed_time as f32;
        let c = time_p1.cos();
        let s = time_p1.sin();
        point1.position = vec3(5.0 * c, 5.0, 5.0 * s);

        let colors = picked_facet_id.map(|id| {
            (0..transformations_base.len())
                .map(|i| {
                    if i == id as usize {
                        Srgba::GREEN
                    } else {
                        Srgba::WHITE
                    }
                })
                .collect_vec()
        });
        instanced_facets.set_instances(&Instances {
            transformations: transformations_base
                .iter()
                .map(|mat| {
                    mat * Mat4::from_axis_angle(
                        translation_base.normalize(),
                        degrees(7.2 * facet_rot),
                    ) * Mat4::from_translation(translation_base * trans_factor)
                })
                .collect_vec(),
            colors,
            ..Default::default()
        });

        // Draw
        if shadows_enabled {
            if show_dodeca {
                directional0.generate_shadow_map(1024, &dodeca);
                directional1.generate_shadow_map(1024, &dodeca);
                spot0.generate_shadow_map(1024, &dodeca);
            }
            directional0.generate_shadow_map(1024, &instanced_facets);
            directional1.generate_shadow_map(1024, &instanced_facets);
            spot0.generate_shadow_map(1024, &instanced_facets);
        }

        let lights = [
            &ambient as &dyn Light,
            &spot0,
            &directional0,
            &directional1,
            &point0,
            &point1,
        ];

        // hide the numbers away, relevant ones are placed where needed later on
        // for mesh in &mut numbers {
        //     mesh.set_transformation(Mat4::from_translation(vec3(0., -1000., -1000.)));
        // }

        // // used to roughly center the number on the face; measured, varies with font size
        // const TEXT_HALF_WIDTH: f32 = 1.3;
        // const TEXT_HALF_HEIGHT: f32 = 0.9;
        // // so the numbers are an epsilon in front of the face and visible
        // const EPS: f32 = 0.01;

        // if show_numbers {
        //     for face in &b.0.faces {
        //         let mesh = &mut numbers[face.value as usize - 1];
        //         let trans = match face.dir {
        //             Dir::Back => {
        //                 Mat4::from_translation(vec3(
        //                     x + h + TEXT_HALF_WIDTH,
        //                     y + w + TEXT_HALF_HEIGHT,
        //                     z + 2. * d + EPS,
        //                 ))
        //                 * Mat4::from_angle_z(Deg(180.))
        //             }
        //             Dir::Front => {
        //                 Mat4::from_translation(vec3(
        //                     x + h + TEXT_HALF_WIDTH,
        //                     y + w - TEXT_HALF_HEIGHT,
        //                     z - EPS,
        //                 ))
        //                 * Mat4::from_angle_y(Deg(180.))
        //             }
        //             Dir::Right => {
        //                 Mat4::from_translation(vec3(
        //                     x + h + TEXT_HALF_WIDTH,
        //                     y + 2. * w + EPS,
        //                     z + d - TEXT_HALF_HEIGHT,
        //                 ))
        //                 * Mat4::from_angle_x(Deg(-90.))
        //                 * Mat4::from_angle_z(Deg(180.))
        //             }
        //             Dir::Left => {
        //                 Mat4::from_translation(vec3(
        //                     x + h - TEXT_HALF_WIDTH,
        //                     y - EPS,
        //                     z + d - TEXT_HALF_HEIGHT,
        //                 ))
        //                 * Mat4::from_angle_x(Deg(90.))
        //             }
        //             Dir::Top => {
        //                 Mat4::from_translation(vec3(
        //                     x + 2. * h + EPS,
        //                     y + w - TEXT_HALF_WIDTH,
        //                     z + d - TEXT_HALF_HEIGHT,
        //                 ))
        //                 * Mat4::from_angle_y(Deg(90.))
        //                 * Mat4::from_angle_z(Deg(90.))
        //             }
        //             Dir::Bottom => {
        //                 Mat4::from_translation(vec3(
        //                     x - EPS,
        //                     y + w + TEXT_HALF_WIDTH,
        //                     z + d - TEXT_HALF_HEIGHT,
        //                 ))
        //                 * Mat4::from_angle_y(Deg(-90.))
        //                 * Mat4::from_angle_z(Deg(-90.))
        //             }
        //         };
        //         mesh.set_transformation(trans);
        //     }
        // }

        let screen = frame_input.screen();
        screen.clear(ClearState::default());
        // frame_input
        //     .screen()
        //     .clear(ClearState::color_and_depth(0.8, 0.8, 0.8, 1.0, 1.0))
        //     .render(
        //         &camera,
        //         // &ico,
        //         // &dodeca,
        //         ico.into_iter().chain(&dodeca),
        //         &[&ambient, &directional, &light0, &light1],
        //     );
        // if show_numbers {
        //     frame_input.screen().render(&camera, &numbers, &[]);
        // }

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
                        if show_plane {
                            plane.render_with_material(
                                &NormalMaterial::from_physical_material(&plane.material),
                                &camera,
                                &lights,
                            );
                        }
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
                        if show_plane {
                            plane.render_with_material(
                                &ColorMaterial::from_physical_material(&plane.material),
                                &camera,
                                &lights,
                            );
                        }
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
                        if show_plane {
                            plane.render(&camera, &lights);
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
