#[path = "../common/app.rs"]
mod app;

#[path = "../common/state.rs"]
mod state;

use std::error::Error;
use winit::event_loop::EventLoop;

use app::Application;
use state::Inputs;

fn main() {
    let mut primitive_type = "triangle-list";
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        primitive_type = &args[1];
    }

    let mut topology = wgpu::PrimitiveTopology::TriangleList;
    let mut index_format = None;

    if primitive_type == "triangle-list" {
        topology = wgpu::PrimitiveTopology::TriangleList;
        index_format = None;
    } else if primitive_type == "triangle-strip" {
        topology = wgpu::PrimitiveTopology::TriangleStrip;
        index_format = Some(wgpu::IndexFormat::Uint32);
    }

    let title = "ch01 Primitive ".to_owned() + primitive_type;

    let inputs = Inputs {
        source: wgpu::ShaderSource::Wgsl(include_str!("triangle_primitive.wgsl").into()),
        topology: topology,
        strip_index_format: index_format,
    };

    let _ = run(&title, inputs, 9);

    pub fn run(title: &str, inputs: Inputs, num_vertices: u32) -> Result<(), Box<dyn Error>> {
        env_logger::init();

        let event_loop = EventLoop::builder().build()?;
        let mut app = Application::new(title, inputs, num_vertices);

        event_loop.run_app(&mut app)?;

        Ok(())
    }
}
