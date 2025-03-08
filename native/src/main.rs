fn main() {
    env_logger::init();

    // let args = env::args().collect_vec();
    // let seed: u64 = args[1].parse().unwrap();

    // see graph.svg for the pentagons/triangles/facets arrangement

    gui::demo_3d();
}
