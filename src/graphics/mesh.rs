use wgpu::util::DeviceExt;

/// Foundational building block for a mesh.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Vertex {
    /// Position in 3d space.
    pub position: glam::Vec3,
    /// Vector of the direction this vertex points to (for lighting and etc.)
    pub normal: glam::Vec3,
    /// Used to sample the appropriate pixel(s) from a texture during rendering.
    pub texture_coordinates: glam::Vec2,
}

/// Data type to reference a particular [Vertex] in a list of vertices.
pub type Index = u32;

/// Contains data used to construct a mesh.
pub struct MeshData<'a> {
    /// What vertices make up the mesh.
    pub vertices: &'a [Vertex],
    /// Order in which to traverse the vertices.
    pub indices: &'a [Index],
}

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
    pub(crate) fn load(device: &wgpu::Device, mesh_data: MeshData) -> Mesh {
        Self {
            vertex_buffer: device.create_buffer_init(
                &(wgpu::util::BufferInitDescriptor {
                    label: None,
                    contents: bytemuck::cast_slice(mesh_data.vertices),
                    usage: wgpu::BufferUsages::VERTEX,
                })
            ),
            index_buffer: device.create_buffer_init(
                &(wgpu::util::BufferInitDescriptor {
                    label: None,
                    contents: bytemuck::cast_slice(mesh_data.indices),
                    usage: wgpu::BufferUsages::INDEX,
                })
            ),
        }
    }
}
