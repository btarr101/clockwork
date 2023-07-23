use bytemuck::{ cast_slice, Pod, Zeroable };
use wgpu::{
    Buffer,
    Device,
    util::{ DeviceExt, BufferInitDescriptor },
    BufferUsages,
    VertexBufferLayout,
    VertexAttribute,
    vertex_attr_array,
    BufferAddress,
    VertexStepMode,
};

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Vertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub texture_coordinates: [f32; 2],
}

pub type Index = u32;

pub(crate) struct Mesh {
    pub vertex_buffer: Buffer,
    pub index_buffer: Buffer,
}

unsafe impl Zeroable for Vertex {}
unsafe impl Pod for Vertex {}

pub(crate) const VERTEX_BUFFER_LAYOUT: VertexBufferLayout = {
    const ATTRIBUTES: [VertexAttribute; 3] =
        vertex_attr_array![
        0 => Float32x3,
        1 => Float32x3,
        2 => Float32x2
    ];

    VertexBufferLayout {
        array_stride: std::mem::size_of::<Vertex>() as BufferAddress,
        step_mode: VertexStepMode::Vertex,
        attributes: &ATTRIBUTES,
    }
};

impl Mesh {
    pub(crate) fn load(device: &Device, vertices: &[Vertex], indices: &[Index]) -> Mesh {
        Self {
            vertex_buffer: device.create_buffer_init(
                &(BufferInitDescriptor {
                    label: None,
                    contents: cast_slice(vertices),
                    usage: BufferUsages::VERTEX,
                })
            ),
            index_buffer: device.create_buffer_init(
                &(BufferInitDescriptor {
                    label: None,
                    contents: cast_slice(indices),
                    usage: BufferUsages::INDEX,
                })
            ),
        }
    }
}
