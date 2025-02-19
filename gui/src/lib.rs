mod shapes;

use itertools::Itertools;
use shapes::{
    facet_shift_rotation, Polyhedron, ICO_TILE_COUNT, TILE0_FACET0_CENTER, TRANSFORMATIONS_BASE,
};
use solvers::dodeca::{triangles_to_pentas_shuffled, TRI_TO_FACETS};
use three_d::*;

const COLOR_LIGHT_BLUE: Srgba = Srgba::new_opaque(100, 150, 255);
const COLOR_LIGHT_GOLD: Srgba = Srgba::new_opaque(220, 220, 150);
const COLOR_NEON_GREEN: Srgba = Srgba::new_opaque(100, 255, 100);
const COLOR_FIERY_RED: Srgba = Srgba::new_opaque(255, 50, 0);
const COLOR_GOLD: Srgba = Srgba::new_opaque(240, 160, 80);
const COLOR_GRAY_BROWN: Srgba = Srgba::new_opaque(94, 94, 80);

const COLOR_TILE_0: Srgba = Srgba::new_opaque(150, 90, 0);
const COLOR_TILE_BASE: Srgba = COLOR_GOLD;
const COLOR_TILE_PICK: Srgba = COLOR_GRAY_BROWN;
const COLOR_TEXT_PICK: Srgba = COLOR_NEON_GREEN;
const COLOR_TEXT_GOOD: Srgba = COLOR_LIGHT_GOLD;
const COLOR_TEXT_BAD: Srgba = Srgba::BLACK;

const FONT_OLDENG5: &[u8; 45968] = include_bytes!("OldEnglishFive_mod.ttf");

const ANCHOR_TILE_ID: usize = 11;

const SEED0: u64 = 0xACE0FBA5E15DEAD;

fn generate_unused_numbers(unused: &[i32; 5], context: &Context) -> Vec<Gm<Mesh, ColorMaterial>> {
    let text_generator = TextGenerator::new(FONT_OLDENG5, 0, 25.).unwrap();

    let mut numbers_unused = vec![];
    for i in unused {
        let text_mesh =
            text_generator.generate(&format!("{:0>2}", i), TextLayoutOptions::default());
        let mut text = Gm::new(
            Mesh::new(context, &text_mesh),
            ColorMaterial {
                color: COLOR_GRAY_BROWN,
                ..Default::default()
            },
        );
        text.material.render_states.cull = Cull::Front;
        numbers_unused.push(text);
    }
    numbers_unused
}

fn generate_numbers(
    pentas: &[[i32; 5]; ICO_TILE_COUNT],
    context: &Context,
) -> Vec<(Gm<Mesh, ColorMaterial>, usize, Mat4, Mat4)> {
    // common matrices to place numbers on tiles
    let smaller = Mat4::from_scale(0.1);
    let tile_align =
        Mat4::from_axis_angle(Vec3::unit_x(), degrees(-69.1)) * Mat4::from_angle_z(degrees(-60.));
    let tile_center =
        (1. * Polyhedron::ico_tile().positions[0] + 2. * Polyhedron::ico_tile().positions[1]) / 3.;
    let tile_translate = Mat4::from_translation(tile_center * 1.001);

    let text_generator = TextGenerator::new(FONT_OLDENG5, 0, 2.).unwrap();

    let mut numbers = vec![];
    for (i, penta) in pentas.iter().enumerate() {
        for (j, v) in penta.iter().enumerate() {
            let v = *v;
            let string =
                if v == 60 || v == 90 || v == 6 || v == 9 || v == 80 || v == 8 || v == 10 || v == 1
                {
                    format!("{:0>2}\n\u{2009}_", v)
                } else {
                    format!("{:0>2}\n", v)
                };
            let text_mesh =
                text_generator.generate(&string, TextLayoutOptions { line_height: 0.05 });
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
                Mesh::new(context, &text_mesh),
                ColorMaterial {
                    color: Srgba::BLACK,
                    ..Default::default()
                },
            );
            text.material.render_states.cull = Cull::Front;

            // matrix to put the number on the 1st facet of a tile
            let pos_mat = tile_translate * tile_align * smaller * to_origin;
            // matrix to put the number from the 1st facet of a tile to its proper face
            let rot_mat = facet_shift_rotation(i, j);

            numbers.push((text, i, rot_mat, pos_mat));
        }
    }
    numbers
}

pub fn demo_3d(triplets: &[(i32, i32, i32); 20], unused: &[i32; 5]) {
    let triplets = *triplets;
    let mut pentas = triangles_to_pentas_shuffled(&triplets, SEED0, true, true);
    // debug values: value is facet id
    // let pentas = (0..60)
    //     .chunks(5)
    //     .into_iter()
    //     .map(|x| x.collect_array().unwrap())
    //     .collect_array()
    //     .unwrap();

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

    // tiles 3D objects
    let tile_mat = CpuMaterial {
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

    let tile_mesh = Polyhedron::ico_tile().into_mesh();

    let instances = Instances {
        transformations: vec![],
        colors: Some(vec![Srgba::GREEN; ICO_TILE_COUNT]),
        ..Default::default()
    };
    let mut instanced_tiles = Gm::new(
        InstancedMesh::new(&context, &instances, &tile_mesh),
        PhysicalMaterial::new(&context, &tile_mat),
    );
    instanced_tiles.material.render_states.cull = Cull::Back;

    // numbers unused in the tiles
    let mut numbers_unused = generate_unused_numbers(unused, &context);
    // numbers on tiles
    let mut numbers = generate_numbers(&pentas, &context);

    // lights
    let ambient = AmbientLight::new(&context, 0.2, Srgba::WHITE);
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

    // puzzle model
    let mut puzzle_state: [i32; 60] = pentas
        .iter()
        .flat_map(|penta| *penta)
        .collect_array()
        .unwrap();

    // light vars
    let mut time_d0 = 0.;
    let mut time_d1 = 0.;
    let mut time_d2 = 0.;
    let mut time_s0 = 0.;
    let speed_d0 = 0;
    let speed_d1 = 0;
    let speed_d2 = 0;
    let speed_s0 = 0;

    // rendering & animation
    let trans_factor = 0.05;
    let tile_anim_speed = 10.0;
    let mut picked_tile_id = None;
    let mut rotating = [None; ICO_TILE_COUNT];
    let mut swapping = None;
    let mut pressed_on = None;
    let mut change = true;
    let mut tile_colors = vec![COLOR_TILE_BASE; ICO_TILE_COUNT];

    tile_colors[ANCHOR_TILE_ID] = COLOR_TILE_0;

    // GUI states
    let mut gui = three_d::GUI::new(&context);
    let mut anchor_tile = true;
    let mut swap_on = true;
    let mut seed = SEED0;

    // camera control
    let mut control = FreeOrbitControl::new(camera.target(), 1.0, 50.0);

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

                    ui.label("Gameplay options");
                    if ui
                        .add(Checkbox::new(&mut anchor_tile, "Anchor tile"))
                        .clicked()
                    {
                        change = true;
                    }
                    if ui
                        .add(Checkbox::new(
                            &mut swap_on,
                            "Swappable tiles (resets the puzzle)",
                        ))
                        .clicked()
                    {
                        pentas = triangles_to_pentas_shuffled(&triplets, seed, true, swap_on);
                        puzzle_state = pentas
                            .iter()
                            .flat_map(|penta| *penta)
                            .collect_array()
                            .unwrap();
                        numbers = generate_numbers(&pentas, &context);
                        picked_tile_id = None;
                        swapping = None;
                        rotating = [None; 12];
                        anchor_tile = false;
                        change = true;
                    }

                    // ui.add(Slider::new(&mut trans_factor, -2.5..=2.5).text("tile break out"));
                    // ui.add(
                    //     Slider::new(&mut tile_anim_speed, 1.0..=20.0)
                    //         .text("tile animation speed"),
                    // );

                    // ui.add(three_d::egui::Separator::default());

                    // ui.label("Light options");
                    // ui.add(
                    //     Slider::new(&mut ambient.intensity, 0.0..=1.0).text("Ambient intensity"),
                    // );
                    // ui.add(
                    //     Slider::new(&mut directional0.intensity, 0.0..=1.0)
                    //         .text("Directional 0 intensity"),
                    // );
                    // ui.add(Slider::new(&mut speed_d0, 0..=10).text("Directional 0 speed"));
                    // ui.add(
                    //     Slider::new(&mut directional1.intensity, 0.0..=1.0)
                    //         .text("Directional 1 intensity"),
                    // );
                    // ui.add(Slider::new(&mut speed_d1, 0..=10).text("Directional 1 speed"));
                    // ui.add(
                    //     Slider::new(&mut directional2.intensity, 0.0..=1.0)
                    //         .text("Directional 2 intensity"),
                    // );
                    // ui.add(Slider::new(&mut speed_d2, 0..=10).text("Directional 2 speed"));
                    // ui.add(Slider::new(&mut spot0.intensity, 0.0..=10.0).text("Spot intensity"));
                    // ui.add(Slider::new(&mut speed_s0, 0..=10).text("Spot speed"));
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
                        if let Some(pick) = pick(&context, &camera, position, &instanced_tiles) {
                            match pick.geometry_id {
                                0 => pick_id = Some(pick.instance_id as usize),
                                _ => unreachable!(),
                            };
                        } else {
                            // picked out -> unpick current
                            if let Some(id) = picked_tile_id {
                                picked_tile_id = None;
                                tile_colors[id] = COLOR_TILE_BASE;
                                change = true;
                            }
                        }
                    }
                }
            }
        }

        // a geometry was picked
        if let Some(pick_id) = pick_id {
            if !anchor_tile || pick_id != ANCHOR_TILE_ID {
                picked_tile_id = match picked_tile_id {
                    // picked the same tile -> rotate it
                    Some(id) if id == pick_id => {
                        if (rotating[id] as Option<f32>).is_none() {
                            rotating[id] = Some(0.);
                        }
                        Some(id)
                    }

                    // picked another tile -> swap them
                    Some(id) if swap_on => {
                        if swapping.is_none() {
                            swapping = Some((id, pick_id, 0.));
                            change = true;
                        }
                        Some(id)
                    }

                    // picked a new tile
                    _ => {
                        change = true;
                        Some(pick_id)
                    }
                };
            }
        }

        // tile rotation processing
        for (i, rot_opt) in rotating.iter_mut().enumerate() {
            match rot_opt {
                None => (),
                Some(rot) => {
                    let next_rot = f32::min(
                        72.0,
                        *rot + (7.2 * frame_input.elapsed_time as f32 * tile_anim_speed / 200.0),
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

        // swapping processing
        if let Some((picked_id, o_id, prog)) = swapping {
            let next_prog = f32::min(
                100.,
                prog + frame_input.elapsed_time as f32 * tile_anim_speed / 40.,
            );
            // swapping is effective at 50% progress
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
                picked_tile_id = Some(o_id);
                change = true;
            } else if next_prog == 100. {
                swapping = None;
            } else {
                swapping = Some((picked_id, o_id, next_prog));
            }
        }

        // apply visual changes
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
            for tile_color in &mut tile_colors {
                *tile_color = COLOR_TILE_BASE;
            }
            if anchor_tile {
                tile_colors[ANCHOR_TILE_ID] = COLOR_TILE_0;
            } else {
                tile_colors[ANCHOR_TILE_ID] = COLOR_TILE_BASE;
            }
            if let Some(id) = picked_tile_id {
                for num in numbers.iter_mut().skip(id * 5).take(5) {
                    num.0.material.color = COLOR_TEXT_PICK;
                }
                tile_colors[id] = COLOR_TILE_PICK;
            }
            if win {
                // TODO
            }
            change = false;
        }

        // compute tile transformations (rotations, swapping animations)
        let tile_transformations = TRANSFORMATIONS_BASE
            .iter()
            .enumerate()
            .map(|(i, mat)| {
                let rot_mat = if let Some(rot) = rotating[i] {
                    Mat4::from_axis_angle(TILE0_FACET0_CENTER.normalize(), degrees(rot))
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

                mat * rot_mat * Mat4::from_translation(*TILE0_FACET0_CENTER * trans_factor)
            })
            .collect_vec();

        // apply tile transformations to numbers
        for (number, i, rot_mat, pos_mat) in numbers.iter_mut() {
            number.set_transformation(tile_transformations[*i] * *rot_mat * *pos_mat);
        }

        // apply tile transformations to tiles
        instanced_tiles.set_instances(&Instances {
            transformations: tile_transformations,
            colors: Some(tile_colors.clone()),
            ..Default::default()
        });

        // camera controls
        control.handle_events(&mut camera, &mut frame_input.events);

        // lights movements
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

        // shadows
        directional0.generate_shadow_map(1024, &instanced_tiles);
        directional1.generate_shadow_map(1024, &instanced_tiles);
        directional2.generate_shadow_map(1024, &instanced_tiles);
        spot0.generate_shadow_map(1024, &instanced_tiles);

        let lights = [
            &ambient as &dyn Light,
            &spot0,
            &directional0,
            &directional1,
            &directional2,
        ];

        // draw
        let screen = frame_input.screen();
        screen.clear(ClearState::default());

        screen
            .write::<RendererError>(|| {
                instanced_tiles.render(&camera, &lights);
                for number in &numbers {
                    number.0.render(&camera, &[]);
                }
                for (i, number) in numbers_unused.iter_mut().enumerate() {
                    let viewport = frame_input.viewport;
                    number.set_transformation(Mat4::from_translation(Vec3::new(
                        viewport.width as f32 - 50.,
                        viewport.height as f32 - (40. * (i + 1) as f32),
                        0.,
                    )));
                    number.render(&Camera::new_2d(viewport), &[]);
                }
                Ok(())
            })
            .unwrap();

        screen.write(|| gui.render()).unwrap();

        FrameOutput::default()
    });
}
