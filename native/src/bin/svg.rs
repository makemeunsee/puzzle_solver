use itertools::Itertools;
use solvers::dodeca::*;
use svg::{
    node::element::{path::Data, Circle, Group, Path, Text},
    Document,
};

pub fn main() {
    env_logger::init();

    let cos_pi_sixt = (std::f64::consts::PI / 6.).cos();
    let sin_pi_sixt = (std::f64::consts::PI / 6.).sin();
    let i0 = (0., 0.);
    let i1 = (cos_pi_sixt, sin_pi_sixt);
    let i2 = (0., -1.);
    let i3 = (-cos_pi_sixt, sin_pi_sixt);
    let i4 = (2. * cos_pi_sixt, 0.);
    let i5 = (cos_pi_sixt, -1. - sin_pi_sixt);
    let i6 = (-cos_pi_sixt, -1. - sin_pi_sixt);
    let i7 = (-2. * cos_pi_sixt, 0.);
    let i8 = (-cos_pi_sixt, 1. + sin_pi_sixt);
    let i9 = (cos_pi_sixt, 1. + sin_pi_sixt);
    let i10 = (3. * cos_pi_sixt, sin_pi_sixt);
    let i11 = (cos_pi_sixt, -2. - sin_pi_sixt);
    let i12 = (-cos_pi_sixt, -2. - sin_pi_sixt);
    let i13 = (-3. * cos_pi_sixt, sin_pi_sixt);
    let i14 = (-2. * cos_pi_sixt, 1. + 2. * sin_pi_sixt);
    let i15 = (2. * cos_pi_sixt, 1. + 2. * sin_pi_sixt);
    let i16 = (3. * cos_pi_sixt, -3. * sin_pi_sixt);
    let i17 = (-3. * cos_pi_sixt, -3. * sin_pi_sixt);
    let i18 = (0., 3.);
    let i19a = (5. * cos_pi_sixt, -5. * sin_pi_sixt);
    let i19b = (-5. * cos_pi_sixt, -5. * sin_pi_sixt);
    let i19c = (0., 5.);

    let is = [
        i0, i1, i2, i3, i4, i5, i6, i7, i8, i9, i10, i11, i12, i13, i14, i15, i16, i17, i18,
    ];

    let i_dots = is
        .iter()
        .map(|p| {
            Circle::new()
                .set("cx", p.0)
                .set("cy", p.1)
                .set("r", 0.03)
                .set("fill", "mediumblue")
        })
        .collect_vec();

    let i_texts = is
        .iter()
        .enumerate()
        .map(|(i, p)| {
            Text::new(format!("I{}", i))
                .set("x", p.0 + 0.1)
                .set("y", p.1 + 0.1)
                .set("style", "font-size:0.2px;fill:darkblue") //;opacity:1;fill-opacity:1;fill-rule:nonzero;stroke:mediumblue;stroke-width:0.02;stroke-opacity:1")
        })
        .collect_vec();

    let pentas_svg: [[(f64, f64); 5]; 9] = [
        PENTA0, PENTA1, PENTA2, PENTA3, PENTA4, PENTA5, PENTA6, PENTA7, PENTA8,
    ]
    .into_iter()
    .map(|penta| penta.iter().map(|i| is[*i]).collect_array().unwrap())
    .collect_array()
    .unwrap();
    let pentas_inf = [
        [i19a, i16, i11, i12, i17, i19b],
        [i19b, i17, i13, i14, i18, i19c],
        [i19c, i18, i15, i10, i16, i19a],
    ];

    let i_paths = pentas_svg
        .iter()
        .map(|penta| {
            let mut data = Data::new();
            data = data.move_to(penta[0]);
            for p in penta.iter().skip(1) {
                data = data.line_to(*p);
            }
            data = data.close();

            Path::new()
                .set("fill", "none")
                .set("stroke", "mediumblue")
                .set("stroke-width", 0.01)
                .set("d", data)
        })
        .collect_vec();

    let inf_a = Data::new().move_to(i16).line_to(i19a).close();

    let path_inf_a = Path::new()
        .set("fill", "none")
        .set("stroke", "mediumblue")
        .set("stroke-width", 0.01)
        .set("d", inf_a);

    let inf_b = Data::new().move_to(i17).line_to(i19b).close();

    let path_inf_b = Path::new()
        .set("fill", "none")
        .set("stroke", "mediumblue")
        .set("stroke-width", 0.01)
        .set("d", inf_b);

    let inf_c = Data::new().move_to(i18).line_to(i19c).close();

    let path_inf_c = Path::new()
        .set("fill", "none")
        .set("stroke", "mediumblue")
        .set("stroke-width", 0.01)
        .set("d", inf_c);

    let mut ds = pentas_svg
        .iter()
        .map(
            |[(j0_0, j0_1), (j1_0, j1_1), (j2_0, j2_1), (j3_0, j3_1), (j4_0, j4_1)]| {
                (
                    (
                        (j0_0 + j1_0 + j2_0 + j3_0 + j4_0) / 5.,
                        (j0_1 + j1_1 + j2_1 + j3_1 + j4_1) / 5.,
                    ),
                    ((j0_0 + j1_0) / 2., (j0_1 + j1_1) / 2.),
                    ((j1_0 + j2_0) / 2., (j1_1 + j2_1) / 2.),
                    ((j2_0 + j3_0) / 2., (j2_1 + j3_1) / 2.),
                    ((j3_0 + j4_0) / 2., (j3_1 + j4_1) / 2.),
                    ((j4_0 + j0_0) / 2., (j4_1 + j0_1) / 2.),
                )
            },
        )
        .chain(pentas_inf.iter().map(
            |[(j0_0, j0_1), (j1_0, j1_1), (j2_0, j2_1), (j3_0, j3_1), (j4_0, j4_1), (j5_0, j5_1)]| {
                (
                    (0., 0.),
                    ((j0_0 + j1_0) / 2., (j0_1 + j1_1) / 2.),
                    ((j1_0 + j2_0) / 2., (j1_1 + j2_1) / 2.),
                    ((j2_0 + j3_0) / 2., (j2_1 + j3_1) / 2.),
                    ((j3_0 + j4_0) / 2., (j3_1 + j4_1) / 2.),
                    ((j4_0 + j5_0) / 2., (j4_1 + j5_1) / 2.),
                )
            },
        ))
        .collect_vec();
    let d9 = (0., -4.);
    let d10 = (-4. * cos_pi_sixt, 4. * sin_pi_sixt);
    let d11 = (4. * cos_pi_sixt, 4. * sin_pi_sixt);
    ds[9].0 = d9;
    ds[10].0 = d10;
    ds[11].0 = d11;

    let d_dots = ds
        .iter()
        .map(|sxt| sxt.0)
        .chain([d9, d10, d11])
        .map(|p| {
            Circle::new()
                .set("cx", p.0)
                .set("cy", p.1)
                .set("r", 0.03)
                .set("fill", "limegreen")
        })
        .collect_vec();
    let d_paths = ds
        .iter()
        .map(|p| {
            let data = Data::new()
                .move_to(p.0)
                .line_to(p.1)
                .move_to(p.0)
                .line_to(p.2)
                .move_to(p.0)
                .line_to(p.3)
                .move_to(p.0)
                .line_to(p.4)
                .move_to(p.0)
                .line_to(p.5)
                .close();

            Path::new()
                .set("fill", "none")
                .set("stroke", "limegreen")
                .set("stroke-width", 0.01)
                .set("d", data)
        })
        .collect_vec();

    let d_texts = ds
        .iter()
        .enumerate()
        .map(|(i, p)| {
            Text::new(format!("D{}", i))
                .set("x", p.0 .0 - 0.1)
                .set("y", p.0 .1 - 0.1)
                .set("style", "font-size:0.2px;fill:green")
        })
        .collect_vec();

    let f_texts = PENTAS
        .iter()
        .enumerate()
        .flat_map(|(idx, penta)| {
            penta
                .iter()
                .enumerate()
                .map(|(j, i)| {
                    let fidx = idx * 5 + j;
                    if *i < 19 {
                        let (x, y) = (
                            (ds[idx].0 .0 + is[*i].0) / 2.,
                            (ds[idx].0 .1 + is[*i].1) / 2.,
                        );
                        Text::new(format!("f{}", fidx))
                            .set("x", x)
                            .set("y", y)
                            .set("style", "font-size:0.1px")
                    } else {
                        let (x, y) = match idx {
                            9 => ((ds[idx].0 .0 + i19a.0) / 2., (ds[idx].0 .1 + i19a.1) / 2.),
                            10 => ((ds[idx].0 .0 + i19b.0) / 2., (ds[idx].0 .1 + i19b.1) / 2.),
                            11 => ((ds[idx].0 .0 + i19c.0) / 2., (ds[idx].0 .1 + i19c.1) / 2.),
                            _ => panic!(),
                        };
                        Text::new(format!("f{}", fidx))
                            .set("x", x)
                            .set("y", y)
                            .set("style", "font-size:0.1px")
                    }
                })
                .collect_vec()
        })
        .collect_vec();

    let mut i_graph = Group::new();
    for path in i_paths {
        i_graph = i_graph.add(path);
    }
    i_graph = i_graph.add(path_inf_a);
    i_graph = i_graph.add(path_inf_b);
    i_graph = i_graph.add(path_inf_c);

    let mut i_vertices = Group::new();
    for dot in i_dots {
        i_vertices = i_vertices.add(dot);
    }

    let mut i_labels = Group::new();
    for text in i_texts {
        i_labels = i_labels.add(text);
    }
    i_labels = i_labels.add(
        Text::new(format!("(I{})", 19))
            .set("x", i19a.0 + 0.1)
            .set("y", i19a.1 + 0.1)
            .set("style", "font-size:0.2px;fill:darkblue"),
    );
    i_labels = i_labels.add(
        Text::new(format!("(I{})", 19))
            .set("x", i19b.0 + 0.1)
            .set("y", i19b.1 + 0.1)
            .set("style", "font-size:0.2px;fill:darkblue"),
    );
    i_labels = i_labels.add(
        Text::new(format!("(I{})", 19))
            .set("x", i19c.0 + 0.1)
            .set("y", i19c.1 + 0.1)
            .set("style", "font-size:0.2px;fill:darkblue"),
    );

    let mut d_graph = Group::new();
    for path in d_paths {
        d_graph = d_graph.add(path);
    }

    let mut d_vertices = Group::new();
    for dot in d_dots {
        d_vertices = d_vertices.add(dot);
    }

    let mut d_labels = Group::new();
    for text in d_texts {
        d_labels = d_labels.add(text);
    }

    let mut f_labels = Group::new();
    for text in f_texts {
        f_labels = f_labels.add(text);
    }

    let document = Document::new()
        .set("viewBox", (-6, -6, 12, 12))
        .add(i_graph)
        .add(i_vertices)
        .add(d_graph)
        .add(d_vertices)
        .add(i_labels)
        .add(d_labels)
        .add(f_labels);

    svg::save("graph.svg", &document).unwrap();
}
