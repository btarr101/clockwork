use glam::{vec4, Mat4, Vec4};

use crate::{
    graphics::{texture::Texture, Mesh},
    util::repository::ResourceId,
};

/// Structure to represent a rendering operation that can be executed by a [Context].
#[derive(Clone, Copy)]
pub struct RenderOperation {
    /// Transformation to apply to the mesh.
    pub transform: Mat4,

    /// Mesh to render.
    pub mesh_id: ResourceId<Mesh>,

    /// Material to use with the mesh.
    pub material: Material,
}

/// Types of materials that can be used.
#[derive(Clone, Copy)]
pub enum Material {
    BasicDiffuse(BasicDiffuseMaterial),
}

/// Material to apply a texture multiplied by a solid color to a mesh.
#[derive(Clone, Copy)]
pub struct BasicDiffuseMaterial {
    /// Color to apply.
    pub color: Vec4,
    /// Texture to apply.
    pub texture_parameters: Option<TextureParameters>,
}

/// Parameters to use when applying a texture.
#[derive(Clone, Copy)]
pub struct TextureParameters {
    /// Texture to use.
    pub texture_id: ResourceId<Texture>,

    /// Sub section of texture to use.
    pub uv_window: Vec4,
}

impl RenderOperation {
    /// Creates a [RenderOperation] to render a mesh with a solid color.
    pub fn colored_mesh(
        transform: Mat4,
        mesh_id: ResourceId<Mesh>,
        color: Vec4,
    ) -> RenderOperation {
        RenderOperation {
            transform,
            mesh_id,
            material: Material::BasicDiffuse(BasicDiffuseMaterial {
                color,
                texture_parameters: None,
            }),
        }
    }

    /// Creates a [RenderOperation] to render a mesh with a texture multiplied by a color.
    pub fn textured_mesh(
        transform: Mat4,
        mesh_id: ResourceId<Mesh>,
        texture_id: ResourceId<Texture>,
        uv_window: Option<Vec4>,
        color: Vec4,
    ) -> RenderOperation {
        RenderOperation {
            transform,
            mesh_id,
            material: Material::BasicDiffuse(BasicDiffuseMaterial {
                color,
                texture_parameters: Some(TextureParameters {
                    texture_id,
                    uv_window: uv_window.unwrap_or_default(),
                }),
            }),
        }
    }
}

impl Default for TextureParameters {
    fn default() -> Self {
        TextureParameters {
            texture_id: ResourceId::new(0),
            uv_window: vec4(0., 0., 1., 1.),
        }
    }
}

impl TextureParameters {
    /// Creates new texture parameters where the uv_window can have a default value.
    pub fn new(texture: ResourceId<Texture>, uv_window: Option<Vec4>) -> Self {
        TextureParameters {
            texture_id: texture,
            uv_window: uv_window.unwrap_or(vec4(0., 0., 1., 1.)),
        }
    }
}

/// Raw render operation that is easier to parse.
#[derive(Clone, Copy)]
pub(crate) struct RawRenderOperation {
    pub transform: Mat4,
    pub mesh_id: ResourceId<Mesh>,
    pub texture_group_ids: [ResourceId<Texture>; 1],
    pub uv_windows: [Vec4; 1],
    pub colors: [Vec4; 1],
}

impl From<RenderOperation> for RawRenderOperation {
    fn from(value: RenderOperation) -> Self {
        let (texture_group_ids, uv_windows, colors) = match value.material {
            Material::BasicDiffuse(BasicDiffuseMaterial {
                color,
                texture_parameters,
            }) => {
                let TextureParameters {
                    texture_id,
                    uv_window,
                } = texture_parameters.unwrap_or_default();

                ([texture_id], [uv_window], [color])
            }
        };

        RawRenderOperation {
            transform: value.transform,
            mesh_id: value.mesh_id,
            texture_group_ids,
            uv_windows,
            colors,
        }
    }
}
