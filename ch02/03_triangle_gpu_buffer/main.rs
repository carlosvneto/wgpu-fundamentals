use app::Application;
use winit::event_loop::EventLoop;

mod app;
mod state;
mod vertex;

fn main() {
    let title = "ch01 triangle gpu buffer";
    let _ = run(title);

    pub fn run(title: &str) -> anyhow::Result<()> {
        env_logger::init();

        let event_loop = EventLoop::builder().build()?;
        let mut app = Application::default();

        app.title = title;

        event_loop.run_app(&mut app)?;

        Ok(())
    }
}
