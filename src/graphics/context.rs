use bytemuck::{ Zeroable, Pod, bytes_of };
use pollster::block_on;
use raw_window_handle::{ HasRawWindowHandle, HasRawDisplayHandle };
use anyhow::Result;
use wgpu::util::DeviceExt;

use super::{ mesh::{ Mesh, VERTEX_BUFFER_LAYOUT, MeshData }, texture::Texture };

/// Id for accessing a mesh resource.
#[derive(Clone, Copy)]
pub struct MeshId(usize);

/// Id for accessing a texture resource.
#[derive(Clone, Copy, Debug)]
pub struct TextureId(usize);

/// Data needed to render a mesh.
#[derive(Clone, Copy)]
pub struct RenderOperation {
    pub transform: [[f32; 4]; 4],
    pub texture: TextureId,
    pub uv_window: [f32; 4],
    pub mesh: MeshId,
}

/// Context for rendering visual elements.
pub struct Context {
    pub(crate) device: wgpu::Device,
    pub(crate) queue: wgpu::Queue,
    pub(crate) surface: wgpu::Surface,
    pub(crate) surface_config: wgpu::SurfaceConfiguration,

    pub(crate) pipeline: wgpu::RenderPipeline,
    bind_group_layout: wgpu::BindGroupLayout,
    texture_bind_group_layout: wgpu::BindGroupLayout,
    global_buffer: wgpu::Buffer,
    sampler: wgpu::Sampler,

    bind_groups_and_buffers: Vec<(wgpu::BindGroup, wgpu::Buffer)>,
    meshes: Vec<Mesh>,
    bind_groups_and_textures: Vec<(wgpu::BindGroup, Texture)>,

    depth_texture: Texture,
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

fn setup_pipeline(
    device: &wgpu::Device
) -> (wgpu::BindGroupLayout, wgpu::BindGroupLayout, wgpu::RenderPipeline) {
    let bind_group_layout = device.create_bind_group_layout(
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
        })
    );

    let texture_bind_group_layout = device.create_bind_group_layout(
        &(wgpu::BindGroupLayoutDescriptor {
            label: None,
            entries: &[
                // texture
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                // sampler
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
        })
    );

    let layout = device.create_pipeline_layout(
        &(wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[&bind_group_layout, &texture_bind_group_layout],
            push_constant_ranges: &[],
        })
    );

    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: None,
        source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
    });

    let pipeline = device.create_render_pipeline(
        &(wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[VERTEX_BUFFER_LAYOUT],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[
                    Some(wgpu::ColorTargetState {
                        format: wgpu::TextureFormat::Bgra8UnormSrgb,
                        blend: Some(wgpu::BlendState::PREMULTIPLIED_ALPHA_BLENDING),
                        write_mask: wgpu::ColorWrites::ALL,
                    }),
                ],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
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
        })
    );

    (bind_group_layout, texture_bind_group_layout, pipeline)
}

impl Context {
    /// Creates a new [GraphicsContext].
    pub(crate) fn new<Window: HasRawWindowHandle + HasRawDisplayHandle>(
        window: &Window,
        width: u32,
        height: u32
    ) -> Self {
        block_on(Self::new_async(window, width, height))
    }

    /// Creates a new [GraphicsContext] asynchronously.
    pub(crate) async fn new_async<Window: HasRawWindowHandle + HasRawDisplayHandle>(
        window: &Window,
        width: u32,
        height: u32
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
                })
            ).await
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                &(wgpu::DeviceDescriptor {
                    label: None,
                    features: wgpu::Features::POLYGON_MODE_LINE,
                    limits: Default::default(),
                }),
                None
            ).await
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

        let (bind_group_layout, texture_bind_group_layout, pipeline) = setup_pipeline(&device);

        let global_buffer = device.create_buffer_init(
            &(wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytes_of(&GlobalBuffer::zeroed()),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            })
        );

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
            })
        );

        let bind_groups_and_buffers = Vec::new();
        let meshes = Vec::new();
        let bind_groups_and_textures = Vec::new();

        let depth_texture = Texture::create_depth_texture(&device, glam::UVec2 { x: 640, y: 480 });

        Self {
            device,
            queue,
            surface,
            surface_config,

            pipeline,
            bind_group_layout,
            texture_bind_group_layout,
            global_buffer,
            sampler,

            bind_groups_and_buffers,
            meshes,
            bind_groups_and_textures,

            depth_texture,
        }
    }

    /// Loads a mesh and returns a [MeshId] that refers to it.
    pub fn load_mesh(&mut self, mesh_data: MeshData) -> MeshId {
        let mesh = Mesh::load(&self.device, mesh_data);
        let id = MeshId(self.meshes.len());
        self.meshes.push(mesh);
        id
    }

    /// Loads a texture and returns a [TextureId] that refers to it.
    pub fn load_texture(&mut self, bytes: &[u8]) -> Result<TextureId> {
        let texture = Texture::load(&self.device, &self.queue, bytes)?;
        let bind_group = self.device.create_bind_group(
            &(wgpu::BindGroupDescriptor {
                label: None,
                layout: &self.texture_bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(&texture.view),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(&self.sampler),
                    },
                ],
            })
        );

        let id = TextureId(self.bind_groups_and_textures.len());
        self.bind_groups_and_textures.push((bind_group, texture));
        Ok(id)
    }

    /// Performs a render pass.
    pub fn perform_render_pass(
        &mut self,
        model_view_projection: [[f32; 4]; 4],
        operations: &[RenderOperation]
    ) {
        // Step 1: Create necessary local buffers.
        let difference = operations
            .len()
            .checked_sub(self.bind_groups_and_buffers.len())
            .filter(|&difference| difference > 0);

        if let Some(difference) = difference {
            self.bind_groups_and_buffers.extend(
                (0..difference).map(|_| {
                    let local_buffer = self.device.create_buffer_init(
                        &(wgpu::util::BufferInitDescriptor {
                            label: None,
                            contents: bytes_of(&LocalBuffer::zeroed()),
                            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                        })
                    );

                    let bind_group = self.device.create_bind_group(
                        &(wgpu::BindGroupDescriptor {
                            label: None,
                            layout: &self.bind_group_layout,
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
                        })
                    );

                    (bind_group, local_buffer)
                })
            );
        }

        // Step 2: Copy over the global buffer data.
        let global_buffer = GlobalBuffer {
            mvp: model_view_projection,
        };
        self.queue.write_buffer(&self.global_buffer, 0, bytes_of(&global_buffer));

        // Step 3: Start the render pass.
        let surface_texture = self.surface.get_current_texture().unwrap();
        let view = surface_texture.texture.create_view(&wgpu::TextureViewDescriptor::default());

        let mut command_encoder = self.device.create_command_encoder(
            &(wgpu::CommandEncoderDescriptor { label: None })
        );

        {
            let mut render_pass = command_encoder.begin_render_pass(
                &(wgpu::RenderPassDescriptor {
                    label: None,
                    color_attachments: &[
                        Some(wgpu::RenderPassColorAttachment {
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
                        }),
                    ],
                    depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                        view: &self.depth_texture.view,
                        depth_ops: Some(wgpu::Operations {
                            load: wgpu::LoadOp::Clear(1.0),
                            store: true,
                        }),
                        stencil_ops: None,
                    }),
                })
            );

            render_pass.set_pipeline(&self.pipeline);

            // Step 4: Copy data from local buffers and render.
            for (index, operation) in operations.iter().copied().enumerate() {
                let (bind_group, buffer) = self.bind_groups_and_buffers
                    .get(index)
                    .expect("should have been sized");

                let local_buffer = LocalBuffer {
                    transform: operation.transform,
                    uv_window: operation.uv_window,
                };
                self.queue.write_buffer(buffer, 0, bytes_of(&local_buffer));
                render_pass.set_bind_group(0, bind_group, &[]);
                render_pass.set_bind_group(
                    1,
                    &self.bind_groups_and_textures[operation.texture.0].0,
                    &[]
                );

                let mesh = self.meshes.get(operation.mesh.0).unwrap();

                render_pass.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
                render_pass.set_index_buffer(
                    mesh.index_buffer.slice(..),
                    wgpu::IndexFormat::Uint32
                );
                render_pass.draw_indexed(
                    0..(mesh.index_buffer.size() as u32) / (std::mem::size_of::<u32>() as u32),
                    0,
                    0..1
                );
            }
        }

        // Step 5: Submit the pass.
        self.queue.submit(std::iter::once(command_encoder.finish()));

        // Present (TEMP)
        surface_texture.present();
    }

    /// Resizes the surface that is rendered to.
    pub(crate) fn resize_surface(&mut self, new_size: glam::UVec2) {
        self.surface_config.width = new_size.x;
        self.surface_config.height = new_size.y;
        self.surface.configure(&self.device, &self.surface_config);

        self.depth_texture = Texture::create_depth_texture(&self.device, new_size);
    }
}
