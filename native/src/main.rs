use log::info;
use solvers::{constraints, volume};

pub fn main() {
    env_logger::init();

    constraints::solve(100);

    // let mut solver = volume::solver(false);
    // while solver.step() {}
    // for solution in solver.solutions() {
    //     info!("solution:\n{}", solution);
    // }
    // info!("total solutions: {}", solver.solutions().len());

    // uncomment to run the visualization
    // gui::demo_3d();
}
