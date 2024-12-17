use std::time;
use winit::{
    application::ApplicationHandler,
    event::{ElementState, KeyEvent, WindowEvent},
    event_loop::ActiveEventLoop,
    keyboard::{KeyCode, PhysicalKey},
    window::{Window, WindowId},
};

use crate::state::State;

pub struct Application<'a> {
    state: Option<State<'a>>,
    title: &'a str,
    sample_count: u32,
    render_start_time: Option<time::Instant>,
}

impl<'a> Application<'a> {
    pub fn new(
        title: &'a str,
        sample_count: u32,
        render_start_time: Option<time::Instant>,
    ) -> Self {
        Self {
            state: None,
            title,
            sample_count,
            render_start_time,
        }
    }
}

impl<'a> ApplicationHandler for Application<'a> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window_attributes = Window::default_attributes().with_title(self.title);

        let window = event_loop
            .create_window(window_attributes)
            .expect("Failed to create window");

        let state = pollster::block_on(async { State::new(window, self.sample_count).await });

        self.state = Some(state);

        self.render_start_time = Some(time::Instant::now());
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        let window_state = match &mut self.state {
            Some(state) => state,
            None => return,
        };

        if window_state.input(&event) {
            return;
        }

        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key: PhysicalKey::Code(KeyCode::Escape),
                        state: ElementState::Pressed,
                        ..
                    },
                ..
            } => {
                event_loop.exit();
            }
            WindowEvent::Resized(physical_size) => {
                //println!("Resized: {:?}", physical_size);
                window_state.resize(physical_size);
            }
            WindowEvent::RedrawRequested => {
                window_state.window().request_redraw();
                let now = std::time::Instant::now();
                let dt = now - self.render_start_time.unwrap_or(now);
                window_state.update(dt);
                match window_state.render() {
                    Ok(_) => {}
                    // Rebuild your Surface if it's lost or outdated
                    Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                        window_state.resize(window_state.size());
                    }
                    // Terminate application if memory is low
                    Err(wgpu::SurfaceError::OutOfMemory) => {
                        println!("Out of memory");
                        event_loop.exit();
                    }
                    // If a frame takes too long to display, warn and move on to the next frame
                    Err(wgpu::SurfaceError::Timeout) => {
                        println!("Surface timeout");
                    }
                }
            }
            _ => {}
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        if let Some(state) = &self.state {
            state.window().request_redraw();
        }
    }
}
