use std::error::Error;
use winit::event_loop::EventLoop;
use app::Application;

#[path = "../common/app.rs"]
mod app;

mod state;
mod vertex;

fn main() {
    let mut sample_count = 1 as u32;
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        sample_count = args[1].parse::<u32>().unwrap();
    }
    
    let title = "ch03 cube wireframe";
    let _ = run(title, sample_count);

    pub fn run(title: &str, sample_count: u32) -> Result<(), Box<dyn Error>> {
        env_logger::init();

        let event_loop = EventLoop::builder().build()?;
        let mut app = Application::new(title, sample_count, None);

        event_loop.run_app(&mut app)?;

        Ok(())
    }
}
