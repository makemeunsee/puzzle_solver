mod app;
mod common;
mod constraints;
mod volume;

pub fn main() {
    env_logger::init();
    app::run()
}
