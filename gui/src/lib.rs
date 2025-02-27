mod shapes;

use std::{
    collections::HashMap,
    hash::{DefaultHasher, Hash, Hasher},
};

use itertools::Itertools;
use log::debug;
use rand::rngs::SmallRng;
use shapes::{
    facet_shift_rotation, Polyhedron, ICO_TILE_COUNT, TILE0_FACET0_CENTER, TRANSFORMATIONS_BASE,
};
use solvers::dodeca::{triangles_to_pentas_shuffled, TRI_TO_FACETS};
use three_d::{
    core::Context, degrees, pick, vec3, AmbientLight, Attenuation, Camera, ClearState,
    ColorMaterial, CpuMaterial, Cull, DirectionalLight, Event, FrameOutput, FreeOrbitControl,
    Geometry, Gm, InnerSpace, InstancedMesh, Instances, Light, Mat3, Mat4, Mesh, MouseButton,
    Object, PhysicalMaterial, PointLight, RendererError, SquareMatrix, Srgba, TextGenerator,
    TextLayoutOptions, Vec3, Vec4, Viewer, Viewport, Window, WindowSettings,
};

const COLOR_LIGHT_BLUE: Srgba = Srgba::new_opaque(100, 150, 255);
const COLOR_LIGHT_GOLD: Srgba = Srgba::new_opaque(220, 220, 150);
const COLOR_NEON_GREEN: Srgba = Srgba::new_opaque(100, 255, 100);
const COLOR_FIERY_RED: Srgba = Srgba::new_opaque(255, 50, 0);
const COLOR_GOLD: Srgba = Srgba::new_opaque(212, 175, 55);
const COLOR_YELLOW: Srgba = Srgba::new_opaque(255, 226, 0);
const COLOR_GRAY_BROWN: Srgba = Srgba::new_opaque(94, 94, 80);
const COLOR_BLACK_BROWN: Srgba = Srgba::new_opaque(30, 30, 25);

const COLOR_TILE_0: Srgba = Srgba::new_opaque(220, 180, 0);
const COLOR_TILE_BASE: Srgba = COLOR_YELLOW;
const COLOR_TILE_PICK: Srgba = COLOR_GRAY_BROWN;
const COLOR_TEXT_PICK: Srgba = COLOR_NEON_GREEN;
const COLOR_TEXT_GOOD: Srgba = COLOR_LIGHT_GOLD;
const COLOR_TEXT_BAD: Srgba = Srgba::BLACK;

const FONT_TYPELIT: &[u8; 7372] = include_bytes!("TypeLightSans_mod.otf");

const PENTA_ANGLE: f32 = 72.;

const GRID_WIDTH: i32 = 650;
const GRID_COL_WIDTH: i32 = 50;
const GRID_HEIGHT: i32 = 200;
const GRID_ROW_HEIGHT: i32 = 40;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Font {
    TypeLightSans,
}

fn font_bytes_and_size(font: Font) -> (&'static [u8], f32) {
    match font {
        Font::TypeLightSans => (FONT_TYPELIT, 3.5),
    }
}

const ANCHOR_TILE_ID: usize = 11;

const SEED0: u64 = 0xACE0FBA5E15DEAD;

fn generate_numbers_and_bounds(
    numbers: &[i32],
    context: &Context,
    font: &[u8],
    font_size: f32,
) -> HashMap<i32, (Gm<Mesh, ColorMaterial>, (f32, f32, f32, f32))> {
    let text_generator = TextGenerator::new(font, 0, font_size * 10.).unwrap();

    let mut result = HashMap::new();
    for n in numbers {
        let text_mesh = text_generator.generate(&format!("{}", n), TextLayoutOptions::default());
        let extrema = text_mesh.positions.to_f32().iter().fold(
            (1000., 0., 1000., 0.),
            |(x_min, x_max, y_min, y_max), pos| {
                (
                    f32::min(x_min, pos.x),
                    f32::max(x_max, pos.x),
                    f32::min(y_min, pos.y),
                    f32::max(y_max, pos.y),
                )
            },
        );
        let mut text = Gm::new(
            Mesh::new(context, &text_mesh),
            ColorMaterial {
                color: COLOR_GRAY_BROWN,
                ..Default::default()
            },
        );
        text.material.render_states.cull = Cull::Front;
        result.insert(*n, (text, extrema));
    }
    result
}

fn generate_numbers(
    pentas: &[[i32; 5]; ICO_TILE_COUNT],
    context: &Context,
    font: &[u8],
    font_size: f32,
) -> Vec<(Gm<Mesh, ColorMaterial>, usize, Mat4, Mat4)> {
    // common matrices to place numbers on tiles
    let smaller = Mat4::from_scale(0.1);
    let tile_align =
        Mat4::from_axis_angle(Vec3::unit_x(), degrees(-69.1)) * Mat4::from_angle_z(degrees(-60.));
    let tile_center =
        (1. * Polyhedron::ico_tile().positions[0] + 2. * Polyhedron::ico_tile().positions[1]) / 3.;
    let tile_translate = Mat4::from_translation(tile_center * 1.001);

    let text_generator = TextGenerator::new(font, 0, font_size).unwrap();

    let mut numbers = vec![];
    for (i, penta) in pentas.iter().enumerate() {
        for (j, v) in penta.iter().enumerate() {
            let v = *v;
            let string = if v == 6 || v == 9 {
                format!("{}\n_", v)
            } else if v == 16 || v == 19 || v == 61 {
                format!("{}\n\u{2009}_", v)
            } else {
                format!("{}\n", v)
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
    run(Model::new(triplets, unused));
}

#[derive(Clone)]
struct Model {
    triplets: [(i32, i32, i32); 20],
    pentas: [[i32; 5]; ICO_TILE_COUNT],
    unused: [i32; 5],
    seed: u64,
    goal_sum: i32,
    puzzle_state: [i32; 5 * ICO_TILE_COUNT],
    swap_on: bool,
    anchor_tile: bool,
    triangle_highlighting: bool,
    rng: SmallRng,
}

impl Model {
    fn new(triplets: &[(i32, i32, i32); 20], unused: &[i32; 5]) -> Self {
        use rand::prelude::*;
        let mut rng = SmallRng::seed_from_u64(SEED0);
        let triplets = *triplets;
        debug!("solution:\n{:?}", triplets);
        let pentas = triangles_to_pentas_shuffled(&triplets, &mut rng, true, true);

        // debug values: value is facet id
        // let pentas = (0..60)
        //     .chunks(5)
        //     .into_iter()
        //     .map(|x| x.collect_array().unwrap())
        //     .collect_array()
        //     .unwrap();

        let puzzle_state: [i32; 60] = pentas
            .iter()
            .flat_map(|penta| *penta)
            .collect_array()
            .unwrap();

        let goal_sum = triplets[0].0 + triplets[0].1 + triplets[0].2;

        Model {
            triplets,
            pentas,
            unused: *unused,
            seed: SEED0,
            goal_sum,
            puzzle_state,
            swap_on: true,
            anchor_tile: true,
            triangle_highlighting: true,
            rng,
        }
    }

    fn reset(&mut self) {
        self.pentas =
            triangles_to_pentas_shuffled(&self.triplets, &mut self.rng, true, self.swap_on);
        self.puzzle_state = self
            .pentas
            .iter()
            .flat_map(|penta| *penta)
            .collect_array()
            .unwrap();
        self.anchor_tile = self.swap_on;
    }
}

#[derive(PartialEq, Eq)]
enum GrabMotion {
    Camera,
    PointLight,
    DirectionalLight,
}

struct UIState {
    picked_tile_id: Option<usize>,
    new_pick: Option<usize>,
    swapping: Option<(usize, usize, f32)>,
    rotating: [Option<(f32, bool)>; ICO_TILE_COUNT],
    pressed_on: Option<(f32, f32)>,
    right_click: bool,
    rotation: f32,
    font: Font,
    picked_number: Option<i32>,
    number_grid: bool,
    grab_motion: GrabMotion,
    light_cam0: (f32, f32),
    light_cam1: (f32, f32),
    has_changes: bool,
}

impl UIState {
    fn new() -> Self {
        UIState {
            picked_tile_id: None,
            new_pick: None,
            swapping: None,
            rotating: [None; ICO_TILE_COUNT],
            pressed_on: None,
            right_click: false,
            rotation: 0.,
            font: Font::TypeLightSans,
            picked_number: None,
            number_grid: true,
            grab_motion: GrabMotion::Camera,
            light_cam0: (0., 0.),
            light_cam1: (0., 0.),
            has_changes: true,
        }
    }

    fn reset(&mut self) {
        self.picked_tile_id = None;
        self.swapping = None;
        self.rotating = [None; ICO_TILE_COUNT];
        self.has_changes = true;
    }

    fn handle_event(
        &mut self,
        event: &mut Event,
        context: &Context,
        camera: &mut Camera,
        tiles: &Gm<InstancedMesh, PhysicalMaterial>,
        unused: &[i32],
    ) {
        match event {
            Event::MouseMotion { delta, button, .. } => {
                if Some(MouseButton::Left) == *button {
                    match self.grab_motion {
                        GrabMotion::Camera => (),
                        GrabMotion::PointLight => {
                            self.light_cam0.0 += 0.2 * delta.0;
                            self.light_cam0.1 += 0.2 * delta.1;
                            println!("point_light angles: {:?}", self.light_cam0);
                        }
                        GrabMotion::DirectionalLight => {
                            self.light_cam1.0 += 0.2 * delta.0;
                            self.light_cam1.1 += 0.2 * delta.1;
                            println!("directional1 angles: {:?}", self.light_cam1);
                        }
                    }
                }
            }
            Event::MouseWheel { delta, handled, .. } => {
                self.rotation = delta.1;
                *handled = true;
            }
            // track left click presses to identify dragging movements
            Event::MousePress { position, .. } => {
                self.pressed_on = Some((position.x, position.y));
                self.right_click = false;
            }
            // maybe pick
            Event::MouseRelease {
                button, position, ..
            } => {
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
                    let x = camera.viewport().x;
                    let y = camera.viewport().y;
                    let w = camera.viewport().width as i32;
                    let h = camera.viewport().height as i32;
                    let mx = position.x as i32 - x - (w - GRID_WIDTH);
                    let my = position.y as i32 - y - (h - GRID_HEIGHT);
                    if MouseButton::Left == *button
                        && self.number_grid
                        && (0..=GRID_WIDTH).contains(&mx)
                        && (0..=GRID_HEIGHT).contains(&my)
                    {
                        let number =
                            mx / GRID_COL_WIDTH * 5 + 1 + (GRID_HEIGHT - my) / GRID_ROW_HEIGHT;
                        debug!("picked number {number}");
                        if unused.contains(&number) && self.picked_number.is_some()
                            || self.picked_number == Some(number)
                        {
                            self.picked_number = None;
                            self.has_changes = true;
                        } else {
                            self.picked_number = Some(number);
                            self.has_changes = true;
                        }
                    } else if let Some(pick) = pick(context, camera, *position, tiles) {
                        match pick.geometry_id {
                            0 => {
                                self.new_pick = Some(pick.instance_id as usize);
                                self.right_click = MouseButton::Right == *button;
                            }
                            _ => unreachable!(),
                        };
                    } else if MouseButton::Left == *button && self.picked_tile_id.is_some() {
                        // picked out -> unpick current
                        self.picked_tile_id = None;
                        self.has_changes = true;
                    }
                }
            }
            _ => (),
        }
    }

    fn handle_picking(&mut self, anchor_on: bool, swap_on: bool) {
        if let Some(pick_id) = self.new_pick {
            if !anchor_on || pick_id != ANCHOR_TILE_ID {
                self.picked_tile_id = match self.picked_tile_id {
                    // picked the same tile -> rotate it
                    Some(id) if id == pick_id => Some(id),

                    // picked another tile -> swap them
                    Some(id) if swap_on && self.right_click => {
                        if self.swapping.is_none() {
                            self.swapping = Some((id, pick_id, 0.));
                            self.has_changes = true;
                        }
                        Some(id)
                    }

                    // picked a new tile
                    _ => {
                        self.has_changes = true;
                        Some(pick_id)
                    }
                };
            }
        }
        self.new_pick = None;
        if self.rotation != 0. {
            if let Some(id) = self.picked_tile_id {
                if (self.rotating[id] as Option<_>).is_none() {
                    self.rotating[id] = Some((0., self.rotation > 0.));
                }
            }
            self.rotation = 0.;
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
        0.1,
        50.0,
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

    let dodeca_mesh = Polyhedron::regular_dodecahedron().into_mesh();
    let mut dodeca = Gm::new(
        Mesh::new(&context, &dodeca_mesh),
        PhysicalMaterial::new(&context, &tile_mat),
    );
    dodeca.material.albedo = COLOR_GOLD;

    let tile = Polyhedron::ico_tile();
    const THIN_TILE_INDEX_COUNT: usize = 30;
    let thin_tile = Polyhedron {
        positions: tile
            .positions
            .iter()
            .take(THIN_TILE_INDEX_COUNT)
            .cloned()
            .collect_vec(),
        indices: tile
            .indices
            .iter()
            .take(THIN_TILE_INDEX_COUNT)
            .cloned()
            .collect_vec(),
    };
    let thin_tile_mesh = thin_tile.into_mesh();
    let thin_instances = Instances {
        transformations: vec![],
        colors: Some(vec![Srgba::GREEN; ICO_TILE_COUNT]),
        ..Default::default()
    };
    let mut thin_tiles = Gm::new(
        InstancedMesh::new(&context, &thin_instances, &thin_tile_mesh),
        PhysicalMaterial::default(),
    );

    let tile_mesh = tile.into_mesh();

    let instances = Instances {
        transformations: vec![],
        colors: Some(vec![Srgba::GREEN; ICO_TILE_COUNT]),
        ..Default::default()
    };
    let mut tiles = Gm::new(
        InstancedMesh::new(&context, &instances, &tile_mesh),
        PhysicalMaterial::new(&context, &tile_mat),
    );
    tiles.material.render_states.cull = Cull::Back;

    let mut ui_state = UIState::new();
    let (font_bytes, font_size) = font_bytes_and_size(ui_state.font);
    // numbers on tiles
    let mut numbers = generate_numbers(&model.pentas, &context, font_bytes, font_size);
    // numbers unused in the tiles
    let mut numbers_2d = generate_numbers_and_bounds(
        &model
            .pentas
            .iter()
            .flat_map(|p| p.iter().cloned())
            .chain(model.unused.iter().cloned())
            .collect_vec(),
        &context,
        font_bytes,
        font_size,
    );
    for unused in model.unused {
        numbers_2d.get_mut(&unused).unwrap().0.material.color = COLOR_BLACK_BROWN;
    }

    // lights
    let mut ambient = AmbientLight::new(&context, 0.35, Srgba::WHITE);

    let mut point_light = PointLight::new(
        &context,
        0.8,
        Srgba::WHITE,
        vec3(0.0, 0.0, -1.0),
        Attenuation {
            constant: 0.5,
            linear: 0.05,
            quadratic: 0.005,
        },
    );

    let mut directional_light =
        DirectionalLight::new(&context, 0.5, COLOR_FIERY_RED, vec3(0.0, -1.0, 0.0));

    // rendering & animation
    let mut trans_factor = 0.05;
    let tile_anim_speed = 10.0;
    let mut thick = false;
    let mut tile_colors = vec![COLOR_TILE_BASE; ICO_TILE_COUNT];
    tile_colors[ANCHOR_TILE_ID] = COLOR_TILE_0;
    let mut tile_col = [
        COLOR_TILE_BASE.r as f32 / 255.,
        COLOR_TILE_BASE.g as f32 / 255.,
        COLOR_TILE_BASE.b as f32 / 255.,
        1.,
    ];
    let mut show_dodeca = false;
    let mut dodeca_col = [
        dodeca.material.albedo.r as f32 / 255.,
        dodeca.material.albedo.g as f32 / 255.,
        dodeca.material.albedo.b as f32 / 255.,
        1.,
    ];

    // camera control
    let mut control = FreeOrbitControl::new(camera.target(), 1.0, 50.0);

    let mut gui = three_d::GUI::new(&context);
    let mut seed_buffer = format!("{}", model.seed);

    // lights control vars
    let mut ambient_color = [
        ambient.color.r as f32 / 255.,
        ambient.color.g as f32 / 255.,
        ambient.color.b as f32 / 255.,
        1.,
    ];
    let mut point_color = [
        point_light.color.r as f32 / 255.,
        point_light.color.g as f32 / 255.,
        point_light.color.b as f32 / 255.,
        1.,
    ];
    let mut directional_color = [
        directional_light.color.r as f32 / 255.,
        directional_light.color.g as f32 / 255.,
        directional_light.color.b as f32 / 255.,
        1.,
    ];
    let mut shadows = true;

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
                    ui.heading("Generation");

                    ui.add(TextEdit::singleline(&mut seed_buffer));
                    if ui.add(Button::new("Randomize tiles from seed")).clicked() {
                        model.seed = if let Ok(number) = seed_buffer.parse() {
                            number
                        } else {
                            let mut hasher = DefaultHasher::new();
                            seed_buffer.hash(&mut hasher);
                            hasher.finish()
                        };
                        model.reset();
                        ui_state.reset();
                        let (font_bytes, font_size) = font_bytes_and_size(ui_state.font);
                        numbers = generate_numbers(&model.pentas, &context, font_bytes, font_size);
                    }
                    // if ui
                    //     .radio_value(&mut ui_state.font, Font::TypeLightSans, "Type Light Sans")
                    //     .clicked()
                    // {
                    //     model.reset();
                    //     ui_state.reset();
                    //     let (font_bytes, font_size) = font_bytes_and_size(ui_state.font);
                    //     numbers = generate_numbers(&model.pentas, &context, font_bytes, font_size);
                    //     numbers_unused =
                    //         generate_unused_numbers(&model.unused, &context, font_bytes, font_size);
                    // }

                    ui.add(three_d::egui::Separator::default());
                    ui.heading("Gameplay");
                    if ui
                        .add_enabled(
                            model.swap_on,
                            Checkbox::new(&mut model.anchor_tile, "Anchor tile"),
                        )
                        .clicked()
                    {
                        ui_state.has_changes = true;
                    }
                    if ui
                        .add(Checkbox::new(
                            &mut model.swap_on,
                            "Swappable tiles (resets the puzzle)",
                        ))
                        .clicked()
                    {
                        model.reset();
                        ui_state.reset();
                        let (font_bytes, font_size) = font_bytes_and_size(ui_state.font);
                        numbers = generate_numbers(&model.pentas, &context, font_bytes, font_size);
                    }
                    if ui
                        .add(Checkbox::new(
                            &mut model.triangle_highlighting,
                            "Highlight valid facets",
                        ))
                        .clicked()
                    {
                        ui_state.has_changes = true;
                    }
                    if ui
                        .checkbox(&mut ui_state.number_grid, "Show number grid")
                        .clicked()
                        && !ui_state.number_grid
                    {
                        ui_state.picked_number = None;
                        ui_state.has_changes = true;
                    }

                    ui.add(three_d::egui::Separator::default());
                    ui.heading("Rendering");

                    ui.add(Slider::new(&mut trans_factor, -2.25..=1.).text("tile break out"));
                    ui.checkbox(&mut show_dodeca, "Put a dodecahedron inside");
                    let dodeca_color_label = ui.label("Dodeca color");
                    ui.color_edit_button_rgba_unmultiplied(&mut dodeca_col)
                        .labelled_by(dodeca_color_label.id);
                    ui.checkbox(&mut thick, "Thick tiles");
                    let tile_color_label = ui.label("Tile color");
                    if ui
                        .color_edit_button_rgba_unmultiplied(&mut tile_col)
                        .labelled_by(tile_color_label.id)
                        .changed()
                    {
                        ui_state.has_changes = true;
                    }

                    ui.add(three_d::egui::Separator::default());
                    ui.heading("Lighting");
                    ui.add(
                        Slider::new(&mut tiles.material.metallic, 0.0..=1.0)
                            .text("Tile metallicity"),
                    );
                    ui.add(
                        Slider::new(&mut tiles.material.roughness, 0.0..=1.0)
                            .text("Tile roughness"),
                    );
                    ui.add(
                        Slider::new(&mut ambient.intensity, 0.0..=1.0).text("Ambient intensity"),
                    );
                    ui.color_edit_button_rgba_unmultiplied(&mut ambient_color);
                    ui.add(
                        Slider::new(&mut point_light.intensity, 0.0..=1.0)
                            .text("Point light intensity"),
                    );
                    ui.color_edit_button_rgba_unmultiplied(&mut point_color);
                    ui.add(
                        Slider::new(&mut directional_light.intensity, 0.0..=1.0)
                            .text("Directional light intensity"),
                    );
                    ui.color_edit_button_rgba_unmultiplied(&mut directional_color);
                    ui.checkbox(&mut shadows, "Shadows");
                    ui.radio_value(
                        &mut ui_state.grab_motion,
                        GrabMotion::PointLight,
                        "Move point light",
                    );
                    ui.radio_value(
                        &mut ui_state.grab_motion,
                        GrabMotion::DirectionalLight,
                        "Move directional light",
                    );
                    ui.radio_value(&mut ui_state.grab_motion, GrabMotion::Camera, "Move camera");
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

        for event in frame_input.events.iter_mut() {
            let tiles = if thick { &tiles } else { &thin_tiles };
            ui_state.handle_event(event, &context, &mut camera, tiles, &model.unused);
        }
        // process all events, then handle picking if any
        ui_state.handle_picking(model.anchor_tile, model.swap_on);

        // tile rotation processing
        for (i, rot_opt) in ui_state.rotating.iter_mut().enumerate() {
            match rot_opt {
                None => (),
                Some((rot, clockwise)) => {
                    let clockwise = *clockwise;
                    let next_rot = f32::min(
                        PENTA_ANGLE,
                        *rot + (PENTA_ANGLE / 10.
                            * frame_input.elapsed_time as f32
                            * tile_anim_speed
                            / 200.0),
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
                            model.puzzle_state.swap(offset0, offset1);
                            numbers.swap(offset0, offset1);
                            let tmp = numbers[offset0].2;
                            numbers[offset0].2 = numbers[offset1].2;
                            numbers[offset1].2 = tmp;
                        }
                        ui_state.has_changes = true;
                    }
                }
            }
        }

        // swapping processing
        if let Some((picked_id, o_id, prog)) = ui_state.swapping {
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
                    model.puzzle_state.swap(offset0, offset1);
                    numbers.swap(offset0, offset1);
                    let tmp = numbers[offset0].1;
                    numbers[offset0].1 = numbers[offset1].1;
                    numbers[offset1].1 = tmp;
                    let tmp = numbers[offset0].2;
                    numbers[offset0].2 = numbers[offset1].2;
                    numbers[offset1].2 = tmp;
                }
                ui_state.swapping = Some((o_id, picked_id, next_prog));
                ui_state.picked_tile_id = Some(o_id);
                ui_state.has_changes = true;
            } else if next_prog == 100. {
                ui_state.swapping = None;
            } else {
                ui_state.swapping = Some((picked_id, o_id, next_prog));
            }
        }

        // apply visual changes
        if ui_state.has_changes {
            let mut win = true;

            for [a, b, c] in TRI_TO_FACETS {
                let num_a = model.puzzle_state[a];
                let num_b = model.puzzle_state[b];
                let num_c = model.puzzle_state[c];

                numbers[a].0.material.color = COLOR_TEXT_BAD;
                numbers[b].0.material.color = COLOR_TEXT_BAD;
                numbers[c].0.material.color = COLOR_TEXT_BAD;

                numbers_2d.get_mut(&num_a).unwrap().0.material.color = COLOR_GRAY_BROWN;
                numbers_2d.get_mut(&num_b).unwrap().0.material.color = COLOR_GRAY_BROWN;
                numbers_2d.get_mut(&num_c).unwrap().0.material.color = COLOR_GRAY_BROWN;

                if model.puzzle_state[a] + model.puzzle_state[b] + model.puzzle_state[c]
                    != model.goal_sum
                {
                    win = false;
                } else if model.triangle_highlighting {
                    numbers[a].0.material.color = COLOR_TEXT_GOOD;
                    numbers[b].0.material.color = COLOR_TEXT_GOOD;
                    numbers[c].0.material.color = COLOR_TEXT_GOOD;

                    numbers_2d.get_mut(&num_a).unwrap().0.material.color = COLOR_TEXT_GOOD;
                    numbers_2d.get_mut(&num_b).unwrap().0.material.color = COLOR_TEXT_GOOD;
                    numbers_2d.get_mut(&num_c).unwrap().0.material.color = COLOR_TEXT_GOOD;
                }
            }

            for tile_color in &mut tile_colors {
                *tile_color = Srgba::from(tile_col);
            }

            if model.anchor_tile {
                tile_colors[ANCHOR_TILE_ID] = COLOR_TILE_0;
            }

            if let Some(num) = ui_state.picked_number {
                numbers_2d.get_mut(&num).unwrap().0.material.color = COLOR_FIERY_RED;
                let idx = model
                    .puzzle_state
                    .iter()
                    .find_position(|&n| *n == num)
                    .unwrap()
                    .0;
                numbers[idx].0.material.color = COLOR_FIERY_RED;
            }
            if let Some(id) = ui_state.picked_tile_id {
                for num in numbers.iter_mut().skip(id * 5).take(5) {
                    num.0.material.color = COLOR_TEXT_PICK;
                }
                tile_colors[id] = COLOR_TILE_PICK;
            }

            if win {
                // TODO
                for number in numbers.iter_mut() {
                    number.0.material.color = COLOR_TEXT_GOOD;
                }
            }

            ui_state.has_changes = false;
        }

        // compute tile transformations (rotations, swapping animations)
        let tile_transformations = TRANSFORMATIONS_BASE
            .iter()
            .enumerate()
            .map(|(i, mat)| {
                let rot_mat = if let Some((rot, clockwise)) = ui_state.rotating[i] {
                    Mat4::from_axis_angle(
                        TILE0_FACET0_CENTER.normalize(),
                        degrees(if clockwise { rot } else { -rot }),
                    )
                } else {
                    Mat4::identity()
                };

                let trans_factor = trans_factor
                    + match ui_state.swapping {
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
        tiles.set_instances(&Instances {
            transformations: tile_transformations.clone(),
            colors: Some(tile_colors.clone()),
            ..Default::default()
        });
        thin_tiles.set_instances(&Instances {
            transformations: tile_transformations,
            colors: Some(tile_colors.clone()),
            ..Default::default()
        });

        // dodeca color
        dodeca.material.albedo = Srgba::from(dodeca_col);

        // camera controls
        if ui_state.grab_motion == GrabMotion::Camera {
            control.handle_events(&mut camera, &mut frame_input.events);
        }

        // lights controls
        ambient.color = Srgba::from(ambient_color);

        let cam_pos = camera.position();
        point_light.position = 3. * cam_pos.normalize();
        point_light.color = Srgba::from(point_color);

        point_light.position =
            Mat3::from_angle_y(degrees(ui_state.light_cam0.0)) * point_light.position;
        point_light.position =
            Mat3::from_angle_x(degrees(ui_state.light_cam0.1)) * point_light.position;

        directional_light.direction = ((-cam_pos - camera.up()) / 2.).normalize();
        directional_light.color = Srgba::from(directional_color);

        directional_light.direction =
            Mat3::from_angle_y(degrees(ui_state.light_cam1.0)) * directional_light.direction;
        directional_light.direction =
            Mat3::from_angle_x(degrees(ui_state.light_cam1.1)) * directional_light.direction;

        if shadows && thick {
            directional_light.generate_shadow_map(1024, &tiles);
        } else {
            directional_light.clear_shadow_map();
        }

        let lights = [&ambient as &dyn Light, &point_light, &directional_light];

        // draw
        let screen = frame_input.screen();
        screen.clear(ClearState::default());

        screen
            .write::<RendererError>(|| {
                if show_dodeca {
                    dodeca.render(&camera, &lights);
                }
                if thick {
                    tiles.render(&camera, &lights);
                } else {
                    thin_tiles.render_with_material(&tiles.material, &camera, &lights);
                }
                for number in &numbers {
                    number.0.render(&camera, &[]);
                }
                if ui_state.number_grid {
                    let viewport = frame_input.viewport;
                    for (n, (number, (x_min, x_max, _, _))) in numbers_2d.iter_mut() {
                        let column = (*n - 1) / 5;
                        let row = (*n - 1) % 5;
                        number.set_transformation(Mat4::from_translation(Vec3::new(
                            (viewport.width as i32 - (GRID_WIDTH - column * GRID_COL_WIDTH)
                                + GRID_COL_WIDTH / 2) as f32
                                - (*x_max - *x_min) / 2.,
                            (viewport.height as i32 - (GRID_ROW_HEIGHT * (row + 1))) as f32,
                            0.,
                        )));
                        number.render(&Camera::new_2d(viewport), &[]);
                    }
                } else {
                    let viewport = frame_input.viewport;
                    for (i, n) in model.unused.iter().enumerate() {
                        let (number, (x_min, x_max, _, _)) = numbers_2d.get_mut(n).unwrap();
                        number.set_transformation(Mat4::from_translation(Vec3::new(
                            (viewport.width as i32 - GRID_COL_WIDTH / 2 + GRID_COL_WIDTH / 2)
                                as f32
                                - (*x_max - *x_min) / 2.,
                            (viewport.height as i32 - (GRID_ROW_HEIGHT * (i as i32 + 1))) as f32,
                            0.,
                        )));
                        number.render(&Camera::new_2d(viewport), &[]);
                    }
                }
                Ok(())
            })
            .unwrap();

        screen.write(|| gui.render()).unwrap();

        FrameOutput::default()
    });
}
