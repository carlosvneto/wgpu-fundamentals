use wgpu_fundamentals::vertex_data as vd;

// Ensuring memory alignment
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    position: [f32; 3],
}

pub fn create_vertices(rin: f32, rout: f32, h: f32, n: u16) -> (Vec<Vertex>, Vec<u16>, Vec<u16>) {
    let (pos, ind, ind2) = vd::create_cylinder_data(rin, rout, h, n);
    let mut data: Vec<Vertex> = vec![];
    for i in 0..pos.len() {
        data.push(Vertex { position: pos[i] });
    }
    (data.to_vec(), ind, ind2)
}