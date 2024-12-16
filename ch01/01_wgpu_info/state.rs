use winit::{event::WindowEvent, window::Window};

use wgpu_fundamentals::wgpu_simplified;

pub struct State<'a> {
  pub init: wgpu_simplified::InitWgpu<'a>,
}

impl<'a> State<'a> {
  pub async fn new(window: Window) -> Self {

    let init =  wgpu_simplified::InitWgpu::init_wgpu(window, 1).await;
    
    println!("{:#?}", init.adapter.get_info());
    println!("Adapter{:#?}", init.adapter.limits());
    println!("Device{:#?}", init.device.limits());
    
    Self {
      init,
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
      self.init.surface.configure(&self.init.device, &self.init.config);
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

    Ok(())
  }
}