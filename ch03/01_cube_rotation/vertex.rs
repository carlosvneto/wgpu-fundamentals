use wgpu_fundamentals::vertex_data as vd;

// Ensuring memory alignment
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    position: [f32; 3],
    color: [f32; 3],
}

pub fn create_vertices() -> (Vec<Vertex>, Vec<u16>) {
    let (pos, col, _, _, ind, _) = vd::create_cube_data(2.0);
    let mut data:Vec<Vertex> = vec![];
    for i in 0..pos.len() {
        data.push(Vertex { position:pos[i], color:col[i]});
    }
    (data.to_vec(), ind)
}