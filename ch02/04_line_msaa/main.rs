use std::error::Error;
use winit::event_loop::EventLoop;

mod app;
mod state;

use app::Application;

fn main() {
    let mut sample_count = 1 as u32;
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        sample_count = args[1].parse::<u32>().unwrap();
    }

    let title = "ch02 line msaa";

    let _ = run(&title, sample_count); 
}

pub fn run(title: &str, sample_count: u32) -> Result<(), Box<dyn Error>> {
    env_logger::init();

    let event_loop = EventLoop::builder().build()?;
    let mut app = Application::new(title, sample_count);

    event_loop.run_app(&mut app)?;

    Ok(())
}