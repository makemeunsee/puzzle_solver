use log::info;
use solvers::{constraints, volume};

pub fn main() {
    env_logger::init();

    constraints::solve();

    let mut solver = volume::solver(false);
    while solver.step() {}
    for solution in solver.solutions() {
        info!("solution:\n{}", solution);
    }
    info!("total solutions: {}", solver.solutions().len());

    // to run the visualization
    // gui::demo_3d();
}
