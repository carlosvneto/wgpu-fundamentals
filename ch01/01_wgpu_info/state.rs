use std::sync::Arc;
use winit::{
    event_loop::ActiveEventLoop,
    keyboard::KeyCode,
    window::Window,
};

use wgpu_fundamentals::wgpu_simplified;

pub struct State {
    pub init: wgpu_simplified::InitWgpu,
}

impl State {
    pub async fn new(window: Arc<Window>) -> Self {
        let init = wgpu_simplified::InitWgpu::init_wgpu(window, 1).await;

        println!("{:#?}", init.adapter.get_info());
        println!("Adapter{:#?}", init.adapter.limits());
        println!("Device{:#?}", init.device.limits());

        Self { init }
    }

    pub fn window(&self) -> &Window {
        &self.init.window
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        if width > 0 && height > 0 {
            // The surface needs to be reconfigured every time the window is resized.
            self.init.config.width = width;
            self.init.config.height = height;
            self.init
                .surface
                .configure(&self.init.device, &self.init.config);
        }
    }

    pub fn handle_key(&mut self, event_loop: &ActiveEventLoop, key: KeyCode, pressed: bool) {
        match (key, pressed) {
            (KeyCode::Escape, true) => {
                event_loop.exit();
            } 
            _ => {},
        }
    }

    pub fn update(&mut self) {
        // We don't have anything to update yet
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        Ok(())
    }
}
