// Ensuring memory alignment
#[repr(C)]
// Vertex needs to be copied to create a buffer
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    position: [f32; 2],
    color: [f32; 3],
}

pub const VERTICES: &[Vertex] = &[
    Vertex {
        // vertex a
        position: [0.0, 0.5],
        color: [1.0, 0.0, 0.0],
    },
    Vertex {
        // vertex b
        position: [-0.5, -0.5],
        color: [0.0, 1.0, 0.0],
    },
    Vertex {
        // vertex c
        position: [0.5, -0.5],
        color: [0.0, 0.0, 1.0],
    },
];
