use image::load_from_memory;
use anyhow::Result;
use wgpu::{
    Texture as WGPUTexture,
    TextureView,
    Device,
    util::DeviceExt,
    Queue,
    Extent3d,
    TextureDimension,
    TextureFormat,
    TextureUsages,
    TextureDescriptor,
    TextureViewDescriptor,
};

pub(crate) struct Texture {
    #[allow(unused)]
    texture: WGPUTexture,
    pub view: TextureView,
}

impl Texture {
    pub(crate) fn load(device: &Device, queue: &Queue, bytes: &[u8]) -> Result<Texture> {
        let image = load_from_memory(bytes)?;
        let bytes = image.to_rgba8();

        let texture = device.create_texture_with_data(
            queue,
            &(TextureDescriptor {
                label: None,
                size: Extent3d {
                    width: image.width(),
                    height: image.height(),
                    depth_or_array_layers: 1,
                },
                mip_level_count: 1,
                sample_count: 1,
                dimension: TextureDimension::D2,
                format: TextureFormat::Rgba8UnormSrgb,
                usage: TextureUsages::TEXTURE_BINDING,
                view_formats: &[],
            }),
            &bytes
        );

        let view = texture.create_view(&TextureViewDescriptor::default());

        Ok(Self { texture, view })
    }

    /// Creates a [Texture] used for depth buffering.
    pub(crate) fn create_depth_texture(device: &Device, size: (u32, u32)) -> Texture {
        let texture = device.create_texture(
            &(TextureDescriptor {
                label: None,
                size: Extent3d {
                    width: size.0,
                    height: size.1,
                    depth_or_array_layers: 1,
                },
                mip_level_count: 1,
                sample_count: 1,
                dimension: TextureDimension::D2,
                format: TextureFormat::Depth32Float,
                usage: TextureUsages::RENDER_ATTACHMENT | TextureUsages::TEXTURE_BINDING,
                view_formats: &[],
            })
        );

        let view = texture.create_view(&TextureViewDescriptor::default());
        Self { texture, view }
    }
}
