#[path="../common/app.rs"]
mod app;

#[path="../common/state.rs"]
mod state;

use std::error::Error;
use winit::event_loop::EventLoop;

use app::Application;
use state::Inputs;

fn main() {
    let title = "ch01 triangle vertex color";

    let inputs = state::Inputs{
        source: wgpu::ShaderSource::Wgsl(include_str!("triangle_vertex_color.wgsl").into()),
        topology: wgpu::PrimitiveTopology::TriangleList,
        strip_index_format: None
    };

    let _ = run(&title, inputs, 3); 
}

pub fn run(title: &str, inputs: Inputs, num_vertices: u32) -> Result<(), Box<dyn Error>> {
  env_logger::init();

  let event_loop = EventLoop::builder().build()?;
  let mut app = Application::new(title, inputs, num_vertices);

  event_loop.run_app(&mut app)?;

  Ok(())
}