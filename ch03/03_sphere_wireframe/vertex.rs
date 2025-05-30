use wgpu_fundamentals::vertex_data as vd;

// Ensuring memory alignment
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    position: [f32; 3],
}

pub fn create_vertices(r: f32, u: u16, v: u16) -> (Vec<Vertex>, Vec<u16>, Vec<u16>) {
    let (pos, _, _, ind, ind2) = vd::create_sphere_data(r, u, v);
    let mut data: Vec<Vertex> = vec![];
    for i in 0..pos.len() {
        data.push(Vertex { position: pos[i] });
    }
    (data.to_vec(), ind, ind2)
}
