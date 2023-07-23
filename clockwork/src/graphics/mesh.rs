use wgpu::util::DeviceExt;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Vertex {
    pub position: glam::Vec3,
    pub normal: glam::Vec3,
    pub texture_coordinates: glam::Vec2,
}

pub type Index = u32;

pub(crate) struct Mesh {
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
}

unsafe impl bytemuck::Zeroable for Vertex {}
unsafe impl bytemuck::Pod for Vertex {}

pub(crate) const VERTEX_BUFFER_LAYOUT: wgpu::VertexBufferLayout = {
    const ATTRIBUTES: [
        wgpu::VertexAttribute;
        3
    ] = wgpu::vertex_attr_array![
        0 => Float32x3,
        1 => Float32x3,
        2 => Float32x2
    ];

    wgpu::VertexBufferLayout {
        array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
        step_mode: wgpu::VertexStepMode::Vertex,
        attributes: &ATTRIBUTES,
    }
};

impl Mesh {
    pub(crate) fn load(device: &wgpu::Device, vertices: &[Vertex], indices: &[Index]) -> Mesh {
        Self {
            vertex_buffer: device.create_buffer_init(
                &(wgpu::util::BufferInitDescriptor {
                    label: None,
                    contents: bytemuck::cast_slice(vertices),
                    usage: wgpu::BufferUsages::VERTEX,
                })
            ),
            index_buffer: device.create_buffer_init(
                &(wgpu::util::BufferInitDescriptor {
                    label: None,
                    contents: bytemuck::cast_slice(indices),
                    usage: wgpu::BufferUsages::INDEX,
                })
            ),
        }
    }
}
