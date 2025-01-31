use log::info;
use solvers::{
    common::{Dir, DEPTH, HEIGHT, WIDTH},
    volume,
};
use std::collections::HashMap;
use three_d::*;

pub fn demo_3d() {
    let window = Window::new(WindowSettings {
        title: "Shapes!".to_string(),
        ..Default::default()
    })
    .unwrap();
    let context = window.gl();

    let mut camera = Camera::new_perspective(
        window.viewport(),
        vec3(20.0, 30.0, 45.0),
        // look at the center of the puzzle (we draw with a 2x scale)
        vec3(HEIGHT as f32, WIDTH as f32, DEPTH as f32),
        vec3(0.0, 0.0, 1.0),
        degrees(90.0),
        0.1,
        1000.0,
    );
    let mut control = OrbitControl::new(camera.target(), 1.0, 100.0);

    let mut numbers = vec![];
    let text_generator = TextGenerator::new(include_bytes!("OldEnglishFive.ttf"), 0, 2.).unwrap();
    for i in 0..54 {
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

    let mut pbox = Gm::new(
        Mesh::new(&context, &CpuMesh::cube()),
        PhysicalMaterial::new(&context, &CpuMaterial::default()),
    );
    pbox.set_transformation(
        // scale by the puzzle size
        Mat4::from_nonuniform_scale(HEIGHT as f32, WIDTH as f32, DEPTH as f32)
            // the base cube is centered on origin, and we want the origin to be a corner of the puzzle
            * Mat4::from_translation(vec3(1., 1., 1.)),
    );
    let bounding_box = Gm::new(
        BoundingBox::new_with_thickness(&context, pbox.aabb(), 0.1),
        ColorMaterial {
            color: Srgba::BLACK,
            ..Default::default()
        },
    );

    let colors = HashMap::from([
        (0, (255, 0, 0)),
        (1, (0, 255, 0)),
        (2, (0, 0, 255)),
        (3, (255, 255, 0)),
        (4, (255, 0, 255)),
        (5, (0, 255, 255)),
        (6, (110, 110, 110)),
        (7, (255, 127, 0)),
        (8, (160, 80, 0)),
    ]);
    let mut solver = volume::solver(true);

    let mut last_step_time = 0.;
    let mut step_freq = 20;
    let mut solving = false;
    let mut step_once = false;
    let mut step_to_sol = false;
    let mut transparency = true;
    let mut monochrome = false;
    let mut show_numbers = true;
    let mut solve_sums = false;
    let mut solver_mode_toggle = false;

    let mut gui = three_d::GUI::new(&context);

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
                    ui.add(Slider::new(&mut step_freq, 1..=120).text("Speed"));
                    if ui
                        .add_enabled(!solve_sums || !step_to_sol, Button::new("Play"))
                        .clicked()
                    {
                        solving = true;
                    };
                    if ui
                        .add_enabled(!solve_sums || !step_to_sol, Button::new("Pause"))
                        .clicked()
                    {
                        solving = false;
                    };
                    if ui.add(Button::new("Step once")).clicked() {
                        step_once = true;
                        solving = false;
                    };
                    ui.add(Checkbox::new(&mut step_to_sol, "Step to solutions only"));
                    if ui
                        .add(Checkbox::new(
                            &mut solve_sums,
                            "Toggle solving with side sums (restarts the solver; stepping past the last solution will freeze the app until the solver has explored all configurations)",
                        ))
                        .clicked()
                    {
                        solver_mode_toggle = true;
                    }
                    ui.add(three_d::egui::Separator::default());
                    ui.add(Checkbox::new(&mut show_numbers, "Show numbers"));
                    ui.add(Checkbox::new(&mut transparency, "Transparency"));
                    ui.add(Checkbox::new(&mut monochrome, "Monochrome"));
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

        control.handle_events(&mut camera, &mut frame_input.events);

        // hide the numbers away, relevant ones are placed where needed later on
        for mesh in &mut numbers {
            mesh.set_transformation(Mat4::from_translation(vec3(0., -1000., -1000.)));
        }

        if solver_mode_toggle {
            solving = false;
            solver_mode_toggle = false;
            solver = volume::solver(!solve_sums);
        }
        // would freeze the UI, also there are too few solutions for animation to make sense
        if solve_sums && step_to_sol {
            solving = false;
        }

        let mut blocks = vec![];
        for b in &solver.stack() {
            let color = if monochrome {
                (100, 100, 80)
            } else {
                colors[&b.1]
            };
            let mut block = block_gm(&context, color.0, color.1, color.2, transparency);

            // small GAP between the blocks; prettier and decreases transparency issues
            const GAP: f32 = 0.8;
            let h = b.0.height as f32 - GAP;
            let w = b.0.width as f32 - GAP;
            let d = b.0.depth as f32 - GAP;
            let shape = Mat4::from_nonuniform_scale(h, w, d);

            // factor 2 as the base cube is 2x2x2
            let x = 2. * b.2 as f32 + GAP;
            let y = 2. * b.3 as f32 + GAP;
            let z = 2. * b.4 as f32 + GAP;
            let position = Mat4::from_translation(vec3(x, y, z));

            // the base cube is centered on origin, and we want the origin to be a corner of the puzzle
            let cube_offset = Mat4::from_translation(vec3(1., 1., 1.));
            block.set_transformation(position * shape * cube_offset);
            blocks.push(block);

            // used to roughly center the number on the face; measured, varies with font size
            const TEXT_HALF_WIDTH: f32 = 1.3;
            const TEXT_HALF_HEIGHT: f32 = 0.9;
            // so the numbers are an epsilon in front of the face and visible
            const EPS: f32 = 0.01;

            if show_numbers {
                for face in &b.0.faces {
                    let mesh = &mut numbers[face.value as usize - 1];
                    let trans = match face.dir {
                        Dir::Back => {
                            Mat4::from_translation(vec3(
                                x + h + TEXT_HALF_WIDTH,
                                y + w + TEXT_HALF_HEIGHT,
                                z + 2. * d + EPS,
                            ))
                            * Mat4::from_angle_z(Deg(180.))
                        }
                        Dir::Front => {
                            Mat4::from_translation(vec3(
                                x + h + TEXT_HALF_WIDTH,
                                y + w - TEXT_HALF_HEIGHT,
                                z - EPS,
                            ))
                            * Mat4::from_angle_y(Deg(180.))
                        }
                        Dir::Right => {
                            Mat4::from_translation(vec3(
                                x + h + TEXT_HALF_WIDTH,
                                y + 2. * w + EPS,
                                z + d - TEXT_HALF_HEIGHT,
                            ))
                            * Mat4::from_angle_x(Deg(-90.))
                            * Mat4::from_angle_z(Deg(180.))
                        }
                        Dir::Left => {
                            Mat4::from_translation(vec3(
                                x + h - TEXT_HALF_WIDTH,
                                y - EPS,
                                z + d - TEXT_HALF_HEIGHT,
                            ))
                            * Mat4::from_angle_x(Deg(90.))
                        }
                        Dir::Top => {
                            Mat4::from_translation(vec3(
                                x + 2. * h + EPS,
                                y + w - TEXT_HALF_WIDTH,
                                z + d - TEXT_HALF_HEIGHT,
                            ))
                            * Mat4::from_angle_y(Deg(90.))
                            * Mat4::from_angle_z(Deg(90.))
                        }
                        Dir::Bottom => {
                            Mat4::from_translation(vec3(
                                x - EPS,
                                y + w + TEXT_HALF_WIDTH,
                                z + d - TEXT_HALF_HEIGHT,
                            ))
                            * Mat4::from_angle_y(Deg(-90.))
                            * Mat4::from_angle_z(Deg(-90.))
                        }
                    };
                    mesh.set_transformation(trans);
                }
            }
        }

        frame_input
            .screen()
            .clear(ClearState::color_and_depth(0.8, 0.8, 0.8, 1.0, 1.0))
            .render(&camera, &blocks, &[]);
        if show_numbers {
            frame_input.screen().render(&camera, &numbers, &[]);
        }
        frame_input.screen().render(&camera, &bounding_box, &[]);
        frame_input.screen().write(|| gui.render()).unwrap();

        if solving || step_once {
            step_once = false;
            let delta = frame_input.accumulated_time - last_step_time;
            if !solver.done() && (step_freq == 120 || delta >= 1000. / step_freq as f64) {
                if step_to_sol {
                    solver.step_to_solution();
                } else {
                    solver.step();
                }
                last_step_time = frame_input.accumulated_time;
                if solver.done() {
                    info!("solved, {} solutions found", solver.solutions().len());
                }
            }
        }

        FrameOutput::default()
    });
}

fn block_gm(
    context: &Context,
    r: u8,
    g: u8,
    b: u8,
    transparency: bool,
) -> Gm<Mesh, PhysicalMaterial> {
    Gm::new(
        Mesh::new(context, &CpuMesh::cube()),
        if transparency {
            transparent_mat(context, r, g, b)
        } else {
            opaque_mat(context, r, g, b)
        },
    )
}

fn transparent_mat(context: &Context, r: u8, g: u8, b: u8) -> PhysicalMaterial {
    PhysicalMaterial::new_transparent(
        context,
        &CpuMaterial {
            albedo: Srgba::new(r, g, b, 128),
            emissive: Srgba::new(r, g, b, 50),
            ..Default::default()
        },
    )
}

fn opaque_mat(context: &Context, r: u8, g: u8, b: u8) -> PhysicalMaterial {
    PhysicalMaterial::new_opaque(
        context,
        &CpuMaterial {
            albedo: Srgba::new(r, g, b, 128),
            emissive: Srgba::new(r, g, b, 50),
            ..Default::default()
        },
    )
}
