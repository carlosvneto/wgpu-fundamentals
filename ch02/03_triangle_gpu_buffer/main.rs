use app::Application;
use std::error::Error;
use winit::event_loop::EventLoop;

mod app;
mod state;
mod vertex;

fn main() {
    let title = "ch01 triangle gpu buffer";
    let _ = run(title);

    pub fn run(title: &str) -> Result<(), Box<dyn Error>> {
        env_logger::init();

        let event_loop = EventLoop::builder().build()?;
        let mut app = Application::default();

        app.app_title = title;

        event_loop.run_app(&mut app)?;

        Ok(())
    }
}
