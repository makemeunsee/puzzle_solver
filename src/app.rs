use log::info;
use std::collections::HashMap;
use three_d::*;

use crate::{constraints, volume};

pub fn run() {
    constraints::solve();

    demo_3d();
}

fn demo_3d() {
    let window = Window::new(WindowSettings {
        title: "Shapes!".to_string(),
        max_size: Some((2550, 1440)),
        ..Default::default()
    })
    .unwrap();
    let context = window.gl();

    let mut camera = Camera::new_perspective(
        window.viewport(),
        vec3(0.0, 10.0, 20.0),
        vec3(0.0, 0.0, 0.0),
        vec3(0.0, 1.0, 0.0),
        degrees(90.0),
        0.1,
        1000.0,
    );
    let mut control = OrbitControl::new(camera.target(), 1.0, 100.0);

    let colors = HashMap::from([
        (0, (255, 0, 0)),
        (1, (0, 255, 0)),
        (2, (0, 0, 255)),
        (3, (255, 255, 0)),
        (4, (255, 0, 255)),
        (5, (0, 255, 255)),
        (6, (160, 160, 160)),
        (7, (255, 127, 0)),
        (8, (160, 80, 0)),
    ]);
    let mut solver = volume::solver();

    let mut last_step_time = 0.;
    let mut step_freq = 20;

    // TODO controls: start, pause, step, step until sol, speed

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
                    ui.heading("Control Panel");
                    ui.add(Slider::new(&mut step_freq, 1..=120).text("Speed"));
                    // ui.add(Checkbox::new(&mut is_instanced, "Use Instancing"));
                    // ui.add(Label::new(
                    //     "Increase the cube count until the cubes don't rotate \
                    //                    smoothly anymore, then toggle on instancing. The rotations \
                    //                    should become smooth again.",
                    // ));
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

        let mut meshes = vec![];
        for b in &solver.stack() {
            let color = colors[&b.1];
            let mesh = block(
                &context,
                b.0.height as f32,
                b.0.width as f32,
                b.0.depth as f32,
                b.2 as f32,
                b.3 as f32,
                b.4 as f32,
                color.0,
                color.1,
                color.2,
            );
            meshes.push(mesh);
        }

        frame_input
            .screen()
            .clear(ClearState::color_and_depth(0.8, 0.8, 0.8, 1.0, 1.0))
            .render(&camera, &meshes, &[]);

        let delta = frame_input.accumulated_time - last_step_time;
        if !solver.done() && (step_freq == 120 || delta >= 1000. / step_freq as f64) {
            solver.step();
            last_step_time = frame_input.accumulated_time;
            if solver.done() {
                info!("solved, {} solutions found", solver.solution_count());
            }
        }

        frame_input.screen().write(|| gui.render()).unwrap();

        FrameOutput::default()
    });
}

fn block(
    context: &Context,
    h: f32,
    w: f32,
    d: f32,
    x: f32,
    y: f32,
    z: f32,
    r: u8,
    g: u8,
    b: u8,
) -> Gm<Mesh, PhysicalMaterial> {
    let mut cube = Gm::new(
        Mesh::new(context, &CpuMesh::cube()),
        PhysicalMaterial::new_transparent(
            context,
            &CpuMaterial {
                albedo: Srgba { r, g, b, a: 150 },
                emissive: Srgba { r, g, b, a: 150 },
                ..Default::default()
            },
        ),
    );
    cube.set_transformation(
        Mat4::from_translation(vec3(x - 6.0, y - 5.5, z - 4.5)) // puzzle is 12x11x9 -> center
            * Mat4::from_nonuniform_scale(h, w, d)
            * Mat4::from_scale(0.5)
            * Mat4::from_translation(vec3(1., 1., 1.)),
    );
    cube
}
