use bytemuck::cast_slice;
use cgmath::Matrix4;
use rand;
use std::mem;
use wgpu::util::DeviceExt;
use winit::{
    event::ElementState, event::KeyEvent, event::WindowEvent, keyboard::Key, keyboard::NamedKey,
    window::Window,
};

use crate::vertex::{create_vertices, Vertex};
use wgpu_fundamentals::wgpu_simplified as ws;

pub struct State<'a> {
    init: ws::InitWgpu<'a>,
    pipelines: [wgpu::RenderPipeline; 2],
    vertex_buffer: wgpu::Buffer,
    index_buffers: [wgpu::Buffer; 2],
    uniform_bind_groups: [wgpu::BindGroup; 2],
    uniform_buffers: [wgpu::Buffer; 3],
    view_mat: Matrix4<f32>,
    project_mat: Matrix4<f32>,
    msaa_texture_view: wgpu::TextureView,
    depth_texture_view: wgpu::TextureView,
    indices_lens: [u32; 2],
    plot_type: u32,

    radius: f32,
    u_segments: u16,
    v_segments: u16,
    update_buffers: bool,
    recreate_buffers: bool,
    rotation_speed: f32,
}

impl<'a> State<'a> {
    pub async fn new(window: Window, sample_count: u32) -> Self {
        let init = ws::InitWgpu::init_wgpu(window, sample_count).await;

        let shader = init
            .device
            .create_shader_module(wgpu::include_wgsl!("../common/unlit.wgsl"));

        // uniform data
        let camera_position = (3.0, 1.5, 3.0).into();
        let look_direction = (0.0, 0.0, 0.0).into();
        let up_direction = cgmath::Vector3::unit_y();

        let model_mat = ws::create_model_mat([0.0, 0.0, 0.0], [0.0, 0.0, 0.0], [1.0, 1.0, 1.0]);
        let (view_mat, project_mat, vp_mat) = ws::create_vp_mat(
            camera_position,
            look_direction,
            up_direction,
            init.config.width as f32 / init.config.height as f32,
        );
        let mvp_mat = vp_mat * model_mat;

        let mvp_ref: &[f32; 16] = mvp_mat.as_ref();
        let uniform_buffer = init
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Uniform Buffer"),
                contents: bytemuck::cast_slice(mvp_ref),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            });

        // color uniform buffer for object and wireframe
        let color_buffer = init
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Uniform Buffer"),
                contents: bytemuck::cast_slice([1.0 as f32, 0.0, 0.0].as_ref()),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            });
        let color_buffer2 = init
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Uniform Buffer"),
                contents: bytemuck::cast_slice([1.0 as f32, 1.0, 0.0].as_ref()),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            });

        let (layout, uniform_bind_group) = ws::create_bind_group(
            &init.device,
            vec![wgpu::ShaderStages::VERTEX, wgpu::ShaderStages::FRAGMENT],
            &[
                uniform_buffer.as_entire_binding(),
                color_buffer.as_entire_binding(),
            ],
        );

        let (layout2, uniform_bind_group2) = ws::create_bind_group(
            &init.device,
            vec![wgpu::ShaderStages::VERTEX, wgpu::ShaderStages::FRAGMENT],
            &[
                uniform_buffer.as_entire_binding(),
                color_buffer2.as_entire_binding(),
            ],
        );

        let pipeline_layout = init
            .device
            .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[&layout],
                push_constant_ranges: &[],
            });

        let vertex_buffer_layout = wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &wgpu::vertex_attr_array![0 => Float32x3],
        };

        let mut ppl = ws::IRenderPipeline {
            shader: Some(&shader),
            pipeline_layout: Some(&pipeline_layout),
            vertex_buffer_layout: &[vertex_buffer_layout],
            ..Default::default()
        };
        let pipeline = ppl.new(&init);

        let pipeline_layout2 =
            init.device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("Render Pipeline Layout 2"),
                    bind_group_layouts: &[&layout2],
                    push_constant_ranges: &[],
                });

        let vertex_buffer_layout2 = wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &wgpu::vertex_attr_array![0 => Float32x3],
        };

        let mut ppl2 = ws::IRenderPipeline {
            topology: wgpu::PrimitiveTopology::LineList,
            shader: Some(&shader),
            pipeline_layout: Some(&pipeline_layout2),
            vertex_buffer_layout: &[vertex_buffer_layout2],
            ..Default::default()
        };
        let pipeline2 = ppl2.new(&init);

        let msaa_texture_view = ws::create_msaa_texture_view(&init);
        let depth_texture_view = ws::create_depth_view(&init);

        let (vertex_data, index_data, index_data2) = create_vertices(2.0, 20, 30);
        let vertex_buffer = init
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: cast_slice(&vertex_data),
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            });

        let index_buffer = init
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(&index_data),
                usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
            });

        let index_buffer2 = init
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(&index_data2),
                usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
            });

        Self {
            init,
            pipelines: [pipeline, pipeline2],
            vertex_buffer,
            index_buffers: [index_buffer, index_buffer2],
            uniform_bind_groups: [uniform_bind_group, uniform_bind_group2],
            uniform_buffers: [uniform_buffer, color_buffer, color_buffer2],
            view_mat,
            project_mat,
            msaa_texture_view,
            depth_texture_view,
            indices_lens: [index_data.len() as u32, index_data2.len() as u32],
            plot_type: 0,

            radius: 2.0,
            u_segments: 20,
            v_segments: 30,
            update_buffers: false,
            recreate_buffers: false,
            rotation_speed: 1.0,
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

            self.project_mat =
                ws::create_projection_mat(new_size.width as f32 / new_size.height as f32, true);
            self.depth_texture_view = ws::create_depth_view(&self.init);
            if self.init.sample_count > 1 {
                self.msaa_texture_view = ws::create_msaa_texture_view(&self.init);
            }
        }
    }

    pub fn input(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        logical_key: key,
                        state: ElementState::Pressed,
                        ..
                    },
                ..
            } => match key.as_ref() {
                Key::Named(NamedKey::Control) => {
                    let scolor: [f32; 3] = [rand::random(), rand::random(), rand::random()];
                    self.init.queue.write_buffer(
                        &self.uniform_buffers[1],
                        0,
                        bytemuck::cast_slice(scolor.as_ref()),
                    );
                    return true;
                }
                Key::Named(NamedKey::Alt) => {
                    let wcolor: [f32; 3] = [rand::random(), rand::random(), rand::random()];
                    self.init.queue.write_buffer(
                        &self.uniform_buffers[2],
                        0,
                        bytemuck::cast_slice(wcolor.as_ref()),
                    );
                    return true;
                }
                Key::Named(NamedKey::Space) => {
                    self.plot_type = (self.plot_type + 1) % 3;
                    return true;
                }
                Key::Character("q") => {
                    self.radius += 0.1;
                    self.update_buffers = true;
                    return true;
                }
                Key::Character("a") => {
                    self.radius -= 0.1;
                    self.update_buffers = true;
                    return true;
                }
                Key::Character("w") => {
                    self.u_segments += 1;
                    self.recreate_buffers = true;
                    return true;
                }
                Key::Character("s") => {
                    self.u_segments -= 1;
                    self.recreate_buffers = true;
                    return true;
                }
                Key::Character("e") => {
                    self.v_segments += 1;
                    self.recreate_buffers = true;
                    return true;
                }
                Key::Character("d") => {
                    self.v_segments -= 1;
                    self.recreate_buffers = true;
                    return true;
                }
                Key::Character("r") => {
                    self.rotation_speed += 0.1;
                    return true;
                }
                Key::Character("f") => {
                    self.rotation_speed -= 0.1;
                    if self.rotation_speed < 0.0 {
                        self.rotation_speed = 0.0;
                    }
                    return true;
                }
                _ => false,
            },
            _ => false,
        }
    }

    pub fn update(&mut self, dt: std::time::Duration) {
        // update uniform buffer
        let dt = self.rotation_speed * dt.as_secs_f32();
        let model_mat =
            ws::create_model_mat([0.0, 0.0, 0.0], [dt.sin(), dt.cos(), 0.0], [1.0, 1.0, 1.0]);
        let mvp_mat = self.project_mat * self.view_mat * model_mat;
        let mvp_ref: &[f32; 16] = mvp_mat.as_ref();
        self.init
            .queue
            .write_buffer(&self.uniform_buffers[0], 0, bytemuck::cast_slice(mvp_ref));

        // update vertex buffer
        if self.update_buffers {
            let (pos, _ind, _ind2) = create_vertices(self.radius, self.u_segments, self.v_segments);
            self.init
                .queue
                .write_buffer(&self.vertex_buffer, 0, cast_slice(&pos));
            self.update_buffers = false;
        }

        // recreate vertex and index buffers
        if self.recreate_buffers {
            let (pos, ind, ind2) = create_vertices(self.radius, self.u_segments, self.v_segments);
            self.indices_lens = [ind.len() as u32, ind2.len() as u32];

            self.vertex_buffer.destroy();
            self.vertex_buffer =
                self.init
                    .device
                    .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                        label: Some("Vertex Buffer"),
                        contents: cast_slice(&pos),
                        usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                    });

            let indices_data = [ind, ind2];
            for i in 0..2 {
                self.index_buffers[i].destroy();
                self.index_buffers[i] =
                    self.init
                        .device
                        .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                            label: Some("Index Buffer"),
                            contents: cast_slice(&indices_data[i]),
                            usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
                        });
            }
            self.recreate_buffers = false;
        }
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
            let color_attach = ws::create_color_attachment(&view);
            let msaa_attach = ws::create_msaa_color_attachment(&view, &self.msaa_texture_view);
            let color_attachment = if self.init.sample_count == 1 {
                color_attach
            } else {
                msaa_attach
            };
            let depth_attachment = ws::create_depth_stencil_attachment(&self.depth_texture_view);

            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(color_attachment)],
                depth_stencil_attachment: Some(depth_attachment),
                occlusion_query_set: None,
                timestamp_writes: None,
            });

            let plot_type = if self.plot_type == 1 {
                "shape_only"
            } else if self.plot_type == 2 {
                "wireframe_only"
            } else {
                "both"
            };

            if plot_type == "shape_only" || plot_type == "both" {
                render_pass.set_pipeline(&self.pipelines[0]);
                render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
                render_pass
                    .set_index_buffer(self.index_buffers[0].slice(..), wgpu::IndexFormat::Uint16);
                render_pass.set_bind_group(0, &self.uniform_bind_groups[0], &[]);
                render_pass.draw_indexed(0..self.indices_lens[0], 0, 0..1);
            }

            if plot_type == "wireframe_only" || plot_type == "both" {
                render_pass.set_pipeline(&self.pipelines[1]);
                render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
                render_pass
                    .set_index_buffer(self.index_buffers[1].slice(..), wgpu::IndexFormat::Uint16);
                render_pass.set_bind_group(0, &self.uniform_bind_groups[1], &[]);
                render_pass.draw_indexed(0..self.indices_lens[1], 0, 0..1);
            }
        }

        self.init.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}
