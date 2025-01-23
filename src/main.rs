mod common;
mod constraints;
mod volume;

fn main() {
    env_logger::init();

    // constraints::solve();
    volume::solve();
}
