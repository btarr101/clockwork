use std::collections::HashMap;

use anyhow::Result;
use bytemuck::{bytes_of, Pod, Zeroable};
use glam::UVec2;
use pollster::block_on;
use raw_window_handle::{HasRawDisplayHandle, HasRawWindowHandle};
use wgpu::util::DeviceExt;

use crate::{
    graphics::{mesh::VERTEX_BUFFER_LAYOUT, Mesh, MeshData},
    util::repository::{Repository, ResourceId},
};

use super::texture::Texture;

mod render_operation;
pub use render_operation::*;

/// TextureId for a blank white texture.
const DEFAULT_TEXTURE_ID: ResourceId<Texture> = ResourceId::new(0);

/// Context for rendering visual elements.
pub struct RenderContext {
    pub(crate) device: wgpu::Device,
    pub(crate) queue: wgpu::Queue,
    pub(crate) surface: wgpu::Surface,
    pub(crate) surface_config: wgpu::SurfaceConfiguration,

    // -- BUFFERS --
    /// Bind group layout for the global and local buffers.
    buffers_bind_group_layout: wgpu::BindGroupLayout,

    /// Contains all local buffers and associated bind groups.
    bind_groups_and_buffers: Vec<(wgpu::BindGroup, wgpu::Buffer)>,

    /// Global buffer.
    global_buffer: wgpu::Buffer,
    // -------------

    // -- MESHES --
    /// Mesh resources.
    meshes: Repository<Mesh>,
    // ------------

    // -- TEXTURES --
    /// Bind group layout for textures.
    textures_bind_group_layout: wgpu::BindGroupLayout,

    /// Bind groups for textures.
    // todo: use a faster hashmap!
    textures_bind_groups: HashMap<[ResourceId<Texture>; 1], ([usize; 1], wgpu::BindGroup)>,

    /// Texture resources.
    textures: Repository<Texture>,

    /// Sampler to use with the textures.
    sampler: wgpu::Sampler,

    /// Depth texture.
    depth_texture: Texture,
    // --------------

    // -- RENDER PIPELINES --
    /// Main render pipeline for now.
    pub(crate) render_pipeline: wgpu::RenderPipeline,
    // ----------------------
}

impl RenderContext {
    /// Creates a new [GraphicsContext].
    pub(crate) fn new<Window: HasRawWindowHandle + HasRawDisplayHandle>(
        window: &Window,
        width: u32,
        height: u32,
    ) -> Self {
        block_on(Self::new_async(window, width, height))
    }

    /// Creates a new [GraphicsContext] asynchronously.
    pub(crate) async fn new_async<Window: HasRawWindowHandle + HasRawDisplayHandle>(
        window: &Window,
        width: u32,
        height: u32,
    ) -> Self {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::PRIMARY,
            dx12_shader_compiler: wgpu::Dx12Compiler::Fxc,
        });

        let surface = (unsafe { instance.create_surface(window) }).unwrap();

        let adapter = instance
            .request_adapter(
                &(wgpu::RequestAdapterOptionsBase {
                    power_preference: wgpu::PowerPreference::HighPerformance,
                    force_fallback_adapter: false,
                    compatible_surface: Some(&surface),
                }),
            )
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                &(wgpu::DeviceDescriptor {
                    label: None,
                    features: wgpu::Features::POLYGON_MODE_LINE,
                    limits: Default::default(),
                }),
                None,
            )
            .await
            .unwrap();

        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_capabilities(&adapter).formats[0],
            width,
            height,
            present_mode: wgpu::PresentMode::AutoVsync,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            view_formats: vec![],
        };
        surface.configure(&device, &surface_config);

        // -- BUFFERS --
        let buffers_bind_group_layout = create_buffers_bind_group_layout(&device);
        let bind_groups_and_buffers = Vec::new();
        let global_buffer = device.create_buffer_init(
            &(wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytes_of(&GlobalBuffer::zeroed()),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            }),
        );

        // -- MESHES --
        let meshes = Repository::new();

        // -- TEXTURES --
        let textures_bind_group_layout = create_textures_bind_group_layout(&device);
        let textures_bind_groups = HashMap::new();
        let textures = Repository::new();
        let sampler = device.create_sampler(
            &(wgpu::SamplerDescriptor {
                label: None,
                address_mode_u: wgpu::AddressMode::ClampToEdge,
                address_mode_v: wgpu::AddressMode::ClampToEdge,
                address_mode_w: wgpu::AddressMode::ClampToEdge,
                mag_filter: wgpu::FilterMode::Nearest,
                min_filter: wgpu::FilterMode::Nearest,
                mipmap_filter: wgpu::FilterMode::Nearest,
                lod_min_clamp: 0.0,
                lod_max_clamp: 0.0,
                compare: None,
                anisotropy_clamp: 1,
                border_color: None,
            }),
        );
        let depth_texture = Texture::create_depth_texture(
            &device,
            UVec2 {
                x: width,
                y: height,
            },
        );

        // -- RENDER PIPELINES --
        let render_pipeline = create_render_pipeline(
            &device,
            &create_render_pipeline_layout(
                &device,
                &buffers_bind_group_layout,
                &textures_bind_group_layout,
            ),
            wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
        );

        Self {
            device,
            queue,
            surface,
            surface_config,

            buffers_bind_group_layout,
            bind_groups_and_buffers,
            global_buffer,

            meshes,

            textures_bind_group_layout,
            textures_bind_groups,
            textures,
            sampler,
            depth_texture,

            render_pipeline,
        }
    }

    /// Loads a mesh and returns a [ResourceId<Mesh>] that refers to it.
    pub fn load_mesh(&mut self, mesh_data: MeshData) -> ResourceId<Mesh> {
        let mesh = Mesh::load(&self.device, mesh_data);
        self.meshes.add(mesh, None)
    }

    /// Loads a texture and returns a [TextureId] that refers to it.
    pub fn load_texture(&mut self, bytes: &[u8]) -> Result<ResourceId<Texture>> {
        Ok(self
            .textures
            .add(Texture::load(&self.device, &self.queue, bytes)?, None))
    }

    /// Performs a render pass.
    pub fn perform_render_pass(
        &mut self,
        model_view_projection: [[f32; 4]; 4],
        operations: &[RenderOperation],
    ) {
        let operations: Vec<RawRenderOperation> = operations
            .iter()
            .map(|operation| RawRenderOperation::from(*operation))
            .collect();

        // Step 1: Create necessary local buffers.
        let difference = operations
            .len()
            .checked_sub(self.bind_groups_and_buffers.len())
            .filter(|&difference| difference > 0);

        if let Some(difference) = difference {
            self.bind_groups_and_buffers
                .extend((0..difference).map(|_| {
                    let local_buffer = self.device.create_buffer_init(
                        &(wgpu::util::BufferInitDescriptor {
                            label: None,
                            contents: bytes_of(&LocalBuffer::zeroed()),
                            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                        }),
                    );

                    let bind_group = self.device.create_bind_group(
                        &(wgpu::BindGroupDescriptor {
                            label: None,
                            layout: &self.buffers_bind_group_layout,
                            entries: &[
                                wgpu::BindGroupEntry {
                                    binding: 0,
                                    resource: self.global_buffer.as_entire_binding(),
                                },
                                wgpu::BindGroupEntry {
                                    binding: 1,
                                    resource: local_buffer.as_entire_binding(),
                                },
                            ],
                        }),
                    );

                    (bind_group, local_buffer)
                }));
        }

        // Step 2: Copy over the global buffer data.
        let global_buffer = GlobalBuffer {
            mvp: model_view_projection,
        };
        self.queue
            .write_buffer(&self.global_buffer, 0, bytes_of(&global_buffer));

        // Step 3: Ensure all texture bind groups are created and valid.
        for operation in operations.iter() {
            self.ensure_textures_bind_group_valid(operation.texture_group_ids);
        }

        // Step 3: Start the render pass.
        let surface_texture = self.surface.get_current_texture().unwrap();
        let view = surface_texture
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut command_encoder = self
            .device
            .create_command_encoder(&(wgpu::CommandEncoderDescriptor { label: None }));

        {
            let mut render_pass = command_encoder.begin_render_pass(
                &(wgpu::RenderPassDescriptor {
                    label: None,
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color {
                                r: 0.1,
                                g: 0.2,
                                b: 0.3,
                                a: 1.0,
                            }),
                            store: true,
                        },
                    })],
                    depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                        view: &self.depth_texture.view,
                        depth_ops: Some(wgpu::Operations {
                            load: wgpu::LoadOp::Clear(1.0),
                            store: true,
                        }),
                        stencil_ops: None,
                    }),
                }),
            );

            render_pass.set_pipeline(&self.render_pipeline);

            // Step 4: Copy data from local buffers and render.
            for (index, operation) in operations.iter().copied().enumerate() {
                let (buffers_bind_group, buffer) = self
                    .bind_groups_and_buffers
                    .get(index)
                    .expect("should have been sized");

                // Write data into the local buffer.
                let local_buffer = LocalBuffer {
                    transform: operation.transform.to_cols_array_2d(),
                    uv_window: operation.uv_windows[0].to_array(),
                };
                self.queue.write_buffer(buffer, 0, bytes_of(&local_buffer));

                // Set the local buffers' bind group.
                render_pass.set_bind_group(0, buffers_bind_group, &[]);

                // Set the bind group for the group of textures.
                let textures_bind_group =
                    self.get_textures_bind_group(operation.texture_group_ids[0]);
                render_pass.set_bind_group(1, textures_bind_group, &[]);

                let mesh = &self.meshes[operation.mesh_id];

                render_pass.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
                render_pass
                    .set_index_buffer(mesh.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
                render_pass.draw_indexed(
                    0..(mesh.index_buffer.size() as u32) / (std::mem::size_of::<u32>() as u32),
                    0,
                    0..1,
                );
            }
        }

        // Step 5: Submit the pass.
        self.queue.submit(std::iter::once(command_encoder.finish()));

        // Present (TEMP)
        surface_texture.present();
    }

    /// Resizes the surface that is rendered to.
    pub(crate) fn resize_surface(&mut self, new_size: UVec2) {
        self.surface_config.width = new_size.x;
        self.surface_config.height = new_size.y;
        self.surface.configure(&self.device, &self.surface_config);

        self.depth_texture = Texture::create_depth_texture(&self.device, new_size);
    }

    /// Ensures the bind group for the group of textures is created and valid.
    fn ensure_textures_bind_group_valid(&mut self, texture_ids: [ResourceId<Texture>; 1]) {
        let key = texture_ids;

        let actual_generations =
            texture_ids.map(|texture_id| self.textures.get_generation(texture_id));
        let generate_bind_group_entry = || {
            let entries: Vec<wgpu::BindGroupEntry> = std::iter::once(wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::Sampler(&self.sampler),
            })
            .chain(texture_ids.iter().enumerate().map(|(binding, texture_id)| {
                let texture = &self.textures[*texture_id];
                wgpu::BindGroupEntry {
                    binding: binding as u32 + 1,
                    resource: wgpu::BindingResource::TextureView(&texture.view),
                }
            }))
            .collect();

            let bind_group = self.device.create_bind_group(
                &(wgpu::BindGroupDescriptor {
                    label: None,
                    layout: &self.textures_bind_group_layout,
                    entries: entries.as_slice(),
                }),
            );

            dbg!((&key, &bind_group));
            (actual_generations, bind_group)
        };

        let generations_and_bind_group = self.textures_bind_groups.get(&key);
        if let Some((generations, ..)) = generations_and_bind_group {
            if *generations != actual_generations {
                self.textures_bind_groups
                    .insert(key, generate_bind_group_entry());
            }
        } else {
            self.textures_bind_groups
                .insert(key, generate_bind_group_entry());
        }
    }

    /// Gets the appropriate bind group for the following textures.
    fn get_textures_bind_group(&self, diffuse_texture_id: ResourceId<Texture>) -> &wgpu::BindGroup {
        let key = [diffuse_texture_id];

        &self.textures_bind_groups[&key].1
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
struct GlobalBuffer {
    mvp: [[f32; 4]; 4],
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
struct LocalBuffer {
    transform: [[f32; 4]; 4],
    uv_window: [f32; 4],
}

unsafe impl Zeroable for GlobalBuffer {}
unsafe impl Pod for GlobalBuffer {}

unsafe impl Zeroable for LocalBuffer {}
unsafe impl Pod for LocalBuffer {}

/// Creates the bind group layout for the buffers.
fn create_buffers_bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
    device.create_bind_group_layout(
        &(wgpu::BindGroupLayoutDescriptor {
            label: None,
            entries: &[
                // globals
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                // locals
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
        }),
    )
}

/// Creates the bind group layout for the textures.
fn create_textures_bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
    device.create_bind_group_layout(
        &(wgpu::BindGroupLayoutDescriptor {
            label: None,
            entries: &[
                // sampler
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
                // texture
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
            ],
        }),
    )
}

/// Create the layout for the render pipeline.
fn create_render_pipeline_layout(
    device: &wgpu::Device,
    buffers_bind_group_layout: &wgpu::BindGroupLayout,
    textures_bind_group_layout: &wgpu::BindGroupLayout,
) -> wgpu::PipelineLayout {
    device.create_pipeline_layout(
        &(wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[buffers_bind_group_layout, textures_bind_group_layout],
            push_constant_ranges: &[],
        }),
    )
}

/// Creates the render pipeline.
fn create_render_pipeline(
    device: &wgpu::Device,
    render_pipeline_layout: &wgpu::PipelineLayout,
    shader_source: wgpu::ShaderSource,
) -> wgpu::RenderPipeline {
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: None,
        source: shader_source,
    });

    device.create_render_pipeline(
        &(wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[VERTEX_BUFFER_LAYOUT],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: wgpu::TextureFormat::Bgra8UnormSrgb,
                    blend: Some(wgpu::BlendState::PREMULTIPLIED_ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None, //Some(wgpu::Face::Back),
                unclipped_depth: false,
                polygon_mode: wgpu::PolygonMode::Fill,
                conservative: false,
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: wgpu::TextureFormat::Depth32Float,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::LessEqual,
                stencil: Default::default(),
                bias: Default::default(),
            }),
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        }),
    )
}
