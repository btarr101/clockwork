use image::io::Reader;
use wgpu::{ Texture, Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages };
use wgpu::util::DeviceExt;

use crate::graphics_context::GraphicsContext;

pub fn load_texture(context: GraphicsContext, path: String) -> Option<Texture> {
    let image = Reader::open(path).ok()?.decode().ok()?;
    let bytes = image.to_rgba8();

    Some(
        context.device.create_texture_with_data(
            &context.queue,
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
        )
    )
}
