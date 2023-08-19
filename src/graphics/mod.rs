pub(crate) mod mesh;
pub(crate) mod render_context;
pub(crate) mod texture;

pub use mesh::{Index, Mesh, MeshData, Vertex};
pub use render_context::{
    BasicDiffuseMaterial, Material, RenderContext, RenderOperation, TextureParameters,
};

/// Contains data for typical meshes.
pub mod default_meshes;
