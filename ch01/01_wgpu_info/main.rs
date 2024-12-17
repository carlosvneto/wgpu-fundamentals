mod app;
mod state;

use std::error::Error;
use winit::event_loop::EventLoop;

use app::Application;

fn main() {
    let title = "ch01 wgpu info";

    let _ = run(&title);
}

pub fn run(title: &str) -> Result<(), Box<dyn Error>> {
    env_logger::init();

    let event_loop = EventLoop::builder().build()?;
    let mut app = Application::new(title);

    event_loop.run_app(&mut app)?;

    Ok(())
}
