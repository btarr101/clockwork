use wgpu::util::DeviceExt;

pub struct Texture {
    #[allow(unused)]
    pub(crate) texture: wgpu::Texture,
    pub(crate) view: wgpu::TextureView,
    pub(crate) bind_group: wgpu::BindGroup,
}

pub(crate) fn load_wgpu_texture_and_view(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    bytes: &[u8],
) -> anyhow::Result<(wgpu::Texture, wgpu::TextureView)> {
    let image = image::load_from_memory(bytes)?;
    let bytes = image.to_rgba8();

    let texture = device.create_texture_with_data(
        queue,
        &(wgpu::TextureDescriptor {
            label: None,
            size: wgpu::Extent3d {
                width: image.width(),
                height: image.height(),
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        }),
        &bytes,
    );

    let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

    Ok((texture, view))
}

pub(crate) fn create_wgpu_depth_texture(
    device: &wgpu::Device,
    size: glam::UVec2,
) -> (wgpu::Texture, wgpu::TextureView) {
    let texture = device.create_texture(
        &(wgpu::TextureDescriptor {
            label: None,
            size: wgpu::Extent3d {
                width: size.x,
                height: size.y,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Depth32Float,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        }),
    );

    let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
    (texture, view)
}
