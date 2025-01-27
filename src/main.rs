use log::info;

mod app;
mod common;
mod constraints;
mod volume;

pub fn main() {
    env_logger::init();
    // app::run()
    let mut solver = volume::solver();
    let mut count = 0;
    while solver.step_to_solution() {
        count += 1;
    }
    info!("geometric solutions: {}", count);
}
