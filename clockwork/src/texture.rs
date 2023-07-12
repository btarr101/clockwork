use image::io::Reader;
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
    pub(crate) fn load(device: &Device, queue: &Queue, path: &str) -> Result<Texture> {
        let image = Reader::open(path)?.decode()?;
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
}
