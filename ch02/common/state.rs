use std::sync::Arc;
use wgpu::{IndexFormat, PrimitiveTopology, ShaderSource};
use winit::{event::WindowEvent, window::Window};

use wgpu_fundamentals::wgpu_simplified as ws;

pub struct State {
    init: ws::InitWgpu,
    pipeline: wgpu::RenderPipeline,
    num_vertices: u32,
}

pub struct Inputs<'a> {
    pub source: ShaderSource<'a>,
    pub topology: PrimitiveTopology,
    pub strip_index_format: Option<IndexFormat>,
}

impl State {
    pub async fn new(window: Arc<Window>, inputs: &Inputs<'_>, num_vertices: u32) -> Self {
        let init = ws::InitWgpu::init_wgpu(window.clone(), 1).await;

        // Load the shaders from disk
        let shader = init
            .device
            .create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: inputs.source.clone(),
            });

        let pipeline_layout = init
            .device
            .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[],
                push_constant_ranges: &[],
            });

        let mut ppl = ws::IRenderPipeline {
            shader: Some(&shader),
            pipeline_layout: Some(&pipeline_layout),
            is_depth_stencil: false,
            topology: inputs.topology,
            strip_index_format: inputs.strip_index_format,
            ..Default::default()
        };
        let pipeline = ppl.new(&init);

        Self {
            init,
            pipeline,
            num_vertices,
        }
    }

    pub fn window(&self) -> &Window {
        &self.init.window
    }

    pub fn size(&self) -> winit::dpi::PhysicalSize<u32> {
        self.init.size
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.init.size = new_size;
            // The surface needs to be reconfigured every time the window is resized.
            self.init.config.width = new_size.width;
            self.init.config.height = new_size.height;
            self.init
                .surface
                .configure(&self.init.device, &self.init.config);
        }
    }

    pub fn input(&mut self, _event: &WindowEvent) -> bool {
        // Since there are no events to capture at the moment, we will
        // return false.
        false
    }

    pub fn update(&mut self) {
        // We don't have anything to update yet
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.init.surface.get_current_texture()?;

        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder =
            self.init
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("Render Encoder"),
                });

        {
            let color_attachment = ws::create_color_attachment(&view);
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(color_attachment)],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });

            // Pipeline
            render_pass.set_pipeline(&self.pipeline);
            render_pass.draw(0..self.num_vertices, 0..1);
        }

        // Tell the wgpu to finish the command buffer and send it to the
        // GPU's render queue
        self.init.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}
