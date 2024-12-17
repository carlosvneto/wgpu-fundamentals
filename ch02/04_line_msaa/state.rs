use std::mem;
use wgpu::util::DeviceExt;
use winit::{event::WindowEvent, window::Window};

use wgpu_fundamentals::wgpu_simplified as ws;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    position: [f32; 2],
    color: [f32; 4],
}

pub struct State<'a> {
    init: ws::InitWgpu<'a>,
    pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    vertex_count: u32,
    msaa_texture_view: wgpu::TextureView,
}

impl<'a> State<'a> {
    pub async fn new(window: Window, sample_count: u32) -> Self {
        let init = ws::InitWgpu::init_wgpu(window, sample_count).await;

        let shader = init
            .device
            .create_shader_module(wgpu::include_wgsl!("triangle_gpu_buffer.wgsl"));

        let pipeline_layout = init
            .device
            .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[],
                push_constant_ranges: &[],
            });

        let vertex_buffer_layout = wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &wgpu::vertex_attr_array![0 => Float32x2, 1 => Float32x4],
        };

        let mut ppl = ws::IRenderPipeline {
            shader: Some(&shader),
            topology: wgpu::PrimitiveTopology::LineList,
            pipeline_layout: Some(&pipeline_layout),
            is_depth_stencil: false,
            vertex_buffer_layout: &[vertex_buffer_layout],
            ..Default::default()
        };
        let pipeline = ppl.new(&init);

        let msaa_texture_view = ws::create_msaa_texture_view(&init);

        let mut vertex_data = vec![];

        let max = 50;
        for i in 0..max {
            let percent = i as f32 / max as f32;
            let (sin, cos) = (percent * 2.0 * std::f32::consts::PI).sin_cos();
            vertex_data.push(Vertex {
                position: [0.0, 0.0],
                color: [1.0, -sin, cos, 1.0],
            });
            vertex_data.push(Vertex {
                position: [1.0 * cos, 1.0 * sin],
                color: [sin, -cos, 1.0, 1.0],
            });
        }

        let vertex_buffer = init
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(&vertex_data),
                usage: wgpu::BufferUsages::VERTEX,
            });

        let vertex_count = vertex_data.len() as u32;

        Self {
            init,
            pipeline,
            vertex_buffer,
            vertex_count,
            msaa_texture_view,
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
            if self.init.sample_count > 1 {
                self.msaa_texture_view = ws::create_msaa_texture_view(&self.init);
            }
        }
    }

    pub fn input(&mut self, _event: &WindowEvent) -> bool {
        false
    }

    pub fn update(&mut self) {}

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
            let color_attach = ws::create_color_attachment(&view);
            let msaa_attach = ws::create_msaa_color_attachment(&view, &self.msaa_texture_view);
            let color_attachment = if self.init.sample_count == 1 {
                color_attach
            } else {
                msaa_attach
            };

            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(color_attachment)],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });

            render_pass.set_pipeline(&self.pipeline);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.draw(0..self.vertex_count, 0..1);
        }

        self.init.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}
