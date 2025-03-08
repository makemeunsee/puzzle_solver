mod shapes;

use cgmath::{Angle, Matrix};
use egui_commonmark::{CommonMarkCache, CommonMarkViewer};
use itertools::Itertools;
use shapes::{
    facet_shift_rotation, Polyhedron, FACET0_CENTER, FACET_COUNT, ICO_TILE_COUNT, TILE0_CENTER,
    TILES_CENTERS, TRANSFORMATIONS_BASE,
};
use solvers::dodeca::{offset_to_rot, FACETS, PENTAS_GRAPH, TRI_TO_FACETS};
use three_d::{
    core::Context, degrees, pick, vec3, AmbientLight, Attenuation, Camera, ClearState, Cull,
    DirectionalLight, Event, FrameOutput, FreeOrbitControl, Gm, InnerSpace, InstancedMesh,
    Instances, Light, Mat4, Mesh, MouseButton, Object, PhysicalMaterial, PointLight, RendererError,
    Srgba, TextGenerator, TextLayoutOptions, Vec3, Vec4, Viewport, Window, WindowSettings,
};

const COLOR_LIGHT_BLUE: Srgba = Srgba::new_opaque(100, 150, 255);
const COLOR_LIGHT_GOLD: Srgba = Srgba::new_opaque(220, 210, 140);
const COLOR_NEON_GREEN: Srgba = Srgba::new_opaque(100, 255, 100);
const COLOR_FIERY_RED: Srgba = Srgba::new_opaque(255, 80, 0);
const COLOR_GOLD: Srgba = Srgba::new_opaque(212, 175, 55);
const COLOR_YELLOW: Srgba = Srgba::new_opaque(255, 226, 0);
const COLOR_GRAY_BROWN: Srgba = Srgba::new_opaque(94, 94, 80);
const COLOR_BLACK_BROWN: Srgba = Srgba::new_opaque(30, 30, 25);

const FONT_TYPELIT: &[u8; 7372] = include_bytes!("TypeLightSans_mod.otf");

const PENTA_ANGLE: f32 = 72.;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Font {
    TypeLightSans,
}

fn font_bytes_and_size(font: Font) -> (&'static [u8], f32) {
    match font {
        Font::TypeLightSans => (FONT_TYPELIT, 5.),
    }
}

const ANCHOR_TILE_ID: usize = 11;

const SEED0: u64 = 0xACE0FBA5E15DEAD;

fn generate_glyphs(
    context: &Context,
    font: &[u8],
    font_size: f32,
) -> Vec<(Gm<Mesh, PhysicalMaterial>, usize, Mat4)> {
    // common matrices to place numbers on facets
    let smaller = Mat4::from_scale(0.1);
    let facet_align =
        Mat4::from_axis_angle(Vec3::unit_x(), degrees(-69.1)) * Mat4::from_angle_z(degrees(-60.));
    let facet_center =
        (1. * Polyhedron::ico_tile().positions[0] + 2. * Polyhedron::ico_tile().positions[2]) / 3.;
    let facet_translate = Mat4::from_translation(facet_center * 1.001);

    let text_generator = TextGenerator::new(font, 0, font_size).unwrap();

    let mut glyphs = vec![];
    //     "•", "°", "+", "=", "-", "|", ".", "…", ":", "o", "^", "�", "¦",
    let patterns = [
        ".",      // white
        "..\n..", // blue
        "…",      // red
        "..",     // green
        ".",      // gray
        "..\n..", // yellow
        "..",     // purple
        "…",      // teal
        "..\n..", // dark gray
        "..",     // orange
        "…",      // teal
        "..",     // dark gray
        ".",      // orange
        "..\n..", // gray
        "…",      // yellow
        ".",      // purple
        ".",      // green
        "…",      // blue
        "..\n..", // red
        "..",     // white
    ];
    for (facet, (_, tri)) in FACETS.iter().enumerate() {
        // let glyph = format!("{} {}", facet, patterns[*tri]);
        let glyph = patterns[*tri];
        let text_mesh = text_generator.generate(glyph, TextLayoutOptions { line_height: 0.2 });
        let (x_min, x_max, y_min, y_max, z_min, z_max) = text_mesh.positions.to_f32().iter().fold(
            (1000., 0., 1000., 0., 1000., 0.),
            |mut acc, p| {
                acc.0 = f32::min(acc.0, p.x);
                acc.1 = f32::max(acc.1, p.x);
                acc.2 = f32::min(acc.2, p.y);
                acc.3 = f32::max(acc.3, p.y);
                acc.4 = f32::min(acc.4, p.z);
                acc.5 = f32::max(acc.5, p.z);
                acc
            },
        );
        let to_origin = Mat4::from_translation(Vec3::new(
            -(x_min + x_max) / 2.,
            -(y_min + y_max) / 2.,
            -(z_min + z_max) / 2.,
        ));
        let mut text = Gm::new(
            Mesh::new(context, &text_mesh),
            PhysicalMaterial {
                albedo: Srgba::BLACK,
                emissive: Srgba {
                    r: 5,
                    g: 5,
                    b: 0,
                    a: 255,
                },
                metallic: 0.6,
                roughness: 0.3,
                ..Default::default()
            },
        );
        text.material.render_states.cull = Cull::Front;

        // matrix to put the number on the 1st facet of a tile
        let pos_mat = facet_translate * facet_align * smaller * to_origin;

        glyphs.push((text, facet, pos_mat));
    }
    glyphs
}

pub fn demo_3d() {
    run(Model::new());
}

#[derive(Clone)]
struct Model {
    seed: u64,
}

impl Model {
    fn new() -> Self {
        // let mut rng = SmallRng::seed_from_u64(SEED0);

        Model { seed: SEED0 }
    }

    fn reset(&mut self) {}
}

struct UIState {
    picked_facet_id: Option<usize>,
    new_pick: Option<usize>,
    rotating: [Option<(f32, bool)>; ICO_TILE_COUNT],
    pressed_on: Option<(f32, f32)>,
    new_rotation: f32,
    font: Font,
    has_changes: bool,
    win_anim: Option<f32>,
}

impl UIState {
    fn new() -> Self {
        UIState {
            picked_facet_id: None,
            new_pick: None,
            rotating: [None; ICO_TILE_COUNT],
            pressed_on: None,
            new_rotation: 0.,
            font: Font::TypeLightSans,
            has_changes: true,
            win_anim: None,
        }
    }

    fn reset(&mut self) {
        self.picked_facet_id = None;
        self.rotating = [None; ICO_TILE_COUNT];
        self.has_changes = true;
        self.win_anim = None;
    }

    fn handle_event(
        &mut self,
        event: &mut Event,
        context: &Context,
        camera: &mut Camera,
        facets: &Gm<InstancedMesh, PhysicalMaterial>,
    ) {
        match event {
            Event::MouseWheel { delta, handled, .. } => {
                self.new_rotation = delta.1;
                *handled = true;
            }
            // track left click presses to identify dragging movements
            Event::MousePress {
                button, position, ..
            } if *button == MouseButton::Left => {
                self.pressed_on = Some((position.x, position.y));
            }
            // maybe pick
            Event::MouseRelease {
                button, position, ..
            } if *button == MouseButton::Left => {
                // pick only if not a dragging movement
                let moved = if let Some((x, y)) = self.pressed_on {
                    let delta_x = position.x - x;
                    let delta_y = position.y - y;
                    delta_x * delta_x + delta_y * delta_y > 50.
                } else {
                    false
                };
                self.pressed_on = None;
                if !moved {
                    if let Some(pick) = pick(context, camera, *position, facets, Cull::Back) {
                        // TODO fix pick culling when https://github.com/asny/three-d/pull/542 is released
                        match pick.geometry_id {
                            0 => {
                                self.new_pick = Some(pick.instance_id as usize);
                            }
                            _ => unreachable!(),
                        };
                    } else if self.picked_facet_id.is_some() {
                        // picked out -> unpick current
                        self.picked_facet_id = None;
                        self.has_changes = true;
                    }
                }
            }
            _ => (),
        }
    }

    fn handle_picking(&mut self) {
        if self.win_anim.is_some() {
            return;
        }
        if let Some(pick_id) = self.new_pick {
            self.picked_facet_id = match self.picked_facet_id {
                // picked the same facet
                Some(id) if id == pick_id => Some(id),

                // picked a new facet
                _ => {
                    self.has_changes = true;
                    Some(pick_id)
                }
            };
        }
        self.new_pick = None;
        if self.new_rotation != 0. {
            if let Some(id) = self.picked_facet_id {
                if self.rotating[id / 5].is_none() {
                    self.rotating[id / 5] = Some((0., self.new_rotation > 0.));
                }
            }
            self.new_rotation = 0.;
        }
    }
}

fn run(mut model: Model) {
    let window = Window::new(WindowSettings {
        title: "Dodeca".to_string(),
        ..Default::default()
    })
    .unwrap();
    let context = window.gl();

    let up = -Polyhedron::ico_tile().positions[0];
    let up = TRANSFORMATIONS_BASE[ANCHOR_TILE_ID] * Vec4::new(up.x, up.y, up.z, 0.0);
    let mut camera = Camera::new_perspective(
        window.viewport(),
        vec3(4.0, 4.0, 8.0),
        vec3(0.0, 0.0, 0.0),
        up.truncate().normalize(),
        degrees(45.0),
        1.,
        20.0,
    );
    camera.alt_proj = variable_projection(window.viewport().aspect(), 0.5, 45.);

    fn facet_colors_init() -> Vec<Srgba> {
        let colors = [
            Srgba::WHITE,
            Srgba::BLUE,
            Srgba::RED,
            Srgba::GREEN,
            Srgba::new_opaque(178, 178, 178),
            Srgba::new_opaque(255, 255, 0),
            Srgba::new_opaque(255, 0, 255),
            Srgba::new_opaque(0, 255, 255),
            Srgba::new_opaque(50, 50, 50),
            Srgba::new_opaque(255, 140, 0),
            Srgba::new_opaque(0, 255, 255),   //10
            Srgba::new_opaque(50, 50, 50),    //11
            Srgba::new_opaque(255, 140, 0),   //12
            Srgba::new_opaque(178, 178, 178), //13
            Srgba::new_opaque(255, 255, 0),   //14
            Srgba::new_opaque(255, 0, 255),   //15
            Srgba::GREEN,                     //16
            Srgba::BLUE,                      //17
            Srgba::RED,                       //18
            Srgba::WHITE,                     //19
        ];
        let mut facet_colors = vec![Srgba::WHITE; FACET_COUNT];
        for (i, facets) in TRI_TO_FACETS.iter().enumerate() {
            let color = colors[i];
            for facet in facets {
                facet_colors[*facet] = color;
            }
        }
        facet_colors
    }
    let mut facet_colors = facet_colors_init();

    let mat_facets = PhysicalMaterial {
        metallic: 0.6,
        roughness: 0.3,
        emissive: Srgba::new_opaque(30, 30, 30),
        ..Default::default()
    };

    let tile = Polyhedron::ico_tile();
    const FACET_INDEX_COUNT: usize = 6;
    let facet = Polyhedron {
        positions: tile
            .positions
            .iter()
            .take(FACET_INDEX_COUNT)
            .cloned()
            .collect_vec(),
        indices: tile
            .indices
            .iter()
            .take(FACET_INDEX_COUNT)
            .cloned()
            .collect_vec(),
    };
    let facet_mesh = facet.into_mesh();
    let facet_instances = Instances {
        transformations: vec![],
        colors: Some(vec![Srgba::WHITE; FACET_COUNT]),
        ..Default::default()
    };
    let mut facets = Gm::new(
        InstancedMesh::new(&context, &facet_instances, &facet_mesh),
        mat_facets,
    );
    facets.material.render_states.cull = Cull::Back;

    // let mut orbiting_cube = Gm::new(
    //     Mesh::new(&context, &CpuMesh::cube()),
    //     PhysicalMaterial::default(),
    // );
    // orbiting_cube
    //     .set_transformation(Mat4::from_scale(0.5) * Mat4::from_translation(Vec3::unit_z() * -8.));

    let mut ui_state = UIState::new();
    let (font_bytes, font_size) = font_bytes_and_size(ui_state.font);
    // numbers on facets
    let mut glyphs = generate_glyphs(&context, font_bytes, font_size);

    // lights
    let ambient = AmbientLight::new(&context, 0.6, Srgba::WHITE);

    let mut point_light = PointLight::new(
        &context,
        1.,
        Srgba::WHITE,
        vec3(0.0, 0.0, -1.0),
        Attenuation {
            constant: 0.,
            linear: 0.,
            quadratic: 0.,
        },
    );

    let mut directional_light =
        DirectionalLight::new(&context, 0.7, COLOR_FIERY_RED, vec3(0.0, -1.0, 0.0));

    // rendering & animation
    let mut trans_factor = 0.05;
    let rot_speed = 10.0 / 200.;

    // camera control
    let mut control = FreeOrbitControl::new(camera.target(), 1.0, 50.0);

    let mut gui = three_d::GUI::new(&context);
    let mut _seed_buffer = format!("{}", model.seed);

    let mut modal_rules = false;

    // let mut perspective_factor = 0.5;
    // let mut fov_y = 45.;

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
                    if ui.button("How to play").clicked() {
                        modal_rules = true;
                    }
                    // ui.add_space(50.);
                    // ui.label("projection");
                    // ui.add(Slider::new(&mut perspective_factor, 0.0..=1.0));
                    // ui.label("FOV y");
                    // ui.add(Slider::new(&mut fov_y, 0.0..=180.0));
                    ui.add_space(50.);

                    if ui.button("Reset").clicked() {
                        model.reset();
                        ui_state.reset();
                        let (font_bytes, font_size) = font_bytes_and_size(ui_state.font);
                        glyphs = generate_glyphs(&context, font_bytes, font_size);
                        facet_colors = facet_colors_init();
                        trans_factor = 0.05;
                    };

                    // ui.separator();
                    // ui.heading("Generation");
                    // ui.text_edit_singleline(&mut seed_buffer);
                    // if ui.button("Randomize tiles from seed").clicked() {
                    //     model.seed = if let Ok(number) = seed_buffer.parse() {
                    //         number
                    //     } else {
                    //         let mut hasher = DefaultHasher::new();
                    //         seed_buffer.hash(&mut hasher);
                    //         hasher.finish()
                    //     };
                    // }

                    if modal_rules {
                        let modal = Modal::new(Id::new("rules")).show(ui.ctx(), |ui| {
                            ui.set_width(frame_input.viewport.width as f32 / 2.);

                            let markdown = r"# How to play

* `left click` to select
* `mouse wheel` to rotate";

                            let mut cache = CommonMarkCache::default();
                            CommonMarkViewer::new().show(ui, &mut cache, markdown);
                        });

                        if modal.should_close() {
                            modal_rules = false;
                        }
                    }
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
        // camera.alt_proj = variable_projection(viewport.aspect(), perspective_factor, fov_y);

        for event in frame_input.events.iter_mut() {
            ui_state.handle_event(event, &context, &mut camera, &facets);
        }
        // process all events, then handle picking if any
        ui_state.handle_picking();

        // facet rotation processing
        for (i, rot_opt) in ui_state.rotating.iter_mut().enumerate() {
            match rot_opt {
                None => (),
                Some((rot, clockwise)) => {
                    let clockwise = *clockwise;
                    let next_rot = f32::min(
                        PENTA_ANGLE,
                        *rot + (PENTA_ANGLE / 10. * frame_input.elapsed_time as f32 * rot_speed),
                    );
                    *rot_opt = if next_rot == PENTA_ANGLE {
                        None
                    } else {
                        Some((next_rot, clockwise))
                    };

                    if next_rot == PENTA_ANGLE {
                        let base_offset = i * 5;
                        for j in 0..4 {
                            let j = if clockwise { j } else { 4 - j };
                            let new_j = (j + 1) % 5;
                            let offset0 = base_offset + j;
                            let offset1 = base_offset + new_j;
                            facet_colors.swap(offset0, offset1);
                            glyphs.swap(offset0, offset1);
                            let tmp = glyphs[offset0].1;
                            glyphs[offset0].1 = glyphs[offset1].1;
                            glyphs[offset1].1 = tmp;

                            let (penta_from, penta_to, rot_offset) =
                                offset_to_rot(offset0, offset1, clockwise);
                            for k in 0..5 {
                                let offset0 = penta_from * 5 + k;
                                let offset1 = penta_to * 5 + (rot_offset + offset0) % 5;
                                facet_colors.swap(offset0, offset1);
                                glyphs.swap(offset0, offset1);
                                let tmp = glyphs[offset0].1;
                                glyphs[offset0].1 = glyphs[offset1].1;
                                glyphs[offset1].1 = tmp;
                            }
                        }

                        ui_state.has_changes = true;
                    }
                }
            }
        }

        // apply visual changes
        if ui_state.win_anim.is_none() && ui_state.has_changes {
            let win = false;

            for num in glyphs.iter_mut() {
                num.0.material.albedo = Srgba::BLACK;
                num.0.material.emissive = Srgba::BLACK;
            }
            if let Some(id) = ui_state.picked_facet_id {
                let id = id / 5;
                for num in glyphs.iter_mut().skip(id * 5).take(5) {
                    num.0.material.emissive = Srgba::WHITE;
                }
            }

            if win {
                ui_state.win_anim = Some(0.);
                if let Some(id) = ui_state.picked_facet_id {
                    for num in glyphs.iter_mut().skip(id * 5).take(5) {
                        num.0.material.emissive = COLOR_FIERY_RED;
                    }
                }
            }

            ui_state.has_changes = false;
        }
        if let Some(prog) = ui_state.win_anim {
            let delta = frame_input.elapsed_time as f32 / 10.;
            let next_prog = f32::min(100., prog + delta);
            ui_state.win_anim = Some(next_prog);

            trans_factor *= 1. + delta / 35.;
        }

        // compute facet transformations (rotations, swapping animations)
        let mut facet_transformations = TRANSFORMATIONS_BASE
            .iter()
            .flat_map(|trans| (0..5).map(|_| *trans))
            .collect_vec();
        for (i, base_transf) in facet_transformations.iter_mut().enumerate() {
            if let Some((rot, clockwise)) = ui_state.rotating[i / 5] {
                let rot_mat = Mat4::from_axis_angle(
                    TILE0_CENTER.normalize(),
                    degrees(if clockwise { rot } else { -rot }),
                );
                *base_transf = *base_transf * rot_mat;
            };

            *base_transf = *base_transf
                * facet_shift_rotation(i / 5, i % 5)
                * Mat4::from_translation(*FACET0_CENTER * trans_factor * 2.);
        }
        for i in 0..facet_transformations.len() {
            if let Some((rot, clockwise)) = ui_state.rotating[i / 5] {
                let rot_mat = Mat4::from_axis_angle(
                    TILES_CENTERS[i / 5].normalize(),
                    degrees(if clockwise { rot } else { -rot }),
                );
                if i % 5 == 0 {
                    for j in PENTAS_GRAPH[i / 5] {
                        for k in 0..5 {
                            facet_transformations[j * 5 + k] =
                                rot_mat * facet_transformations[j * 5 + k];
                        }
                    }
                }
            };
        }

        // apply facet transformations to glyphs
        for (glyph, i, pos_mat) in glyphs.iter_mut() {
            glyph.set_transformation(facet_transformations[*i] * *pos_mat);
        }

        // apply facet transformations to facets
        facets.set_instances(&Instances {
            transformations: facet_transformations,
            colors: Some(facet_colors.clone()),
            ..Default::default()
        });

        control.handle_events(&mut camera, &mut frame_input.events);

        // fix lights to camera
        let cam_pos = camera.position();
        point_light.position = 3. * cam_pos.normalize();
        directional_light.direction = ((-cam_pos - camera.up()) / 2.).normalize();

        let lights = [&ambient as &dyn Light, &point_light, &directional_light];

        // draw
        let screen = frame_input.screen();
        screen.clear(ClearState::default());

        screen
            .write::<RendererError>(|| {
                // orbiting_cube.render(&camera, &lights);
                facets.render(&camera, &lights);
                for glyph in &glyphs {
                    glyph.0.render(&camera, &lights);
                }
                Ok(())
            })
            .unwrap();

        screen.write(|| gui.render()).unwrap();

        FrameOutput::default()
    });
}

// from https://math.stackexchange.com/questions/3677516/what-is-the-projection-matrix-of-reverse-byzantine-perspective/3747701#3747701
// f: perspective to antiperspective factor, 0..=1
// fov_y: degrees, <180
fn variable_projection(aspect: f32, f: f32, fov_y: f32) -> Mat4 {
    let far = 20.;
    let near = 1.;
    let fov_y = degrees(fov_y / 2.);
    let tan = fov_y.tan();
    let top = near * tan;
    let right = top * aspect;

    let depth = far - near;

    let ws = near + depth * f;

    let a = -(far + near) / depth;
    let b = -2. * near * far / depth - f * depth;
    let c = 2. * f - 1.;
    let d = f * (far + near);

    let px = ws / right;
    let qy = ws / top;

    Mat4::new(
        px, 0., 0., 0., //
        0., qy, 0., 0., //
        0., 0., a, b, //
        0., 0., c, d,
    )
    .transpose()
}
