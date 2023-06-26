use bytemuck::{ Zeroable, Pod, bytes_of };
use pollster::block_on;
use raw_window_handle::{ HasRawWindowHandle, HasRawDisplayHandle };
use wgpu::{
    Instance,
    InstanceDescriptor,
    Backends,
    Dx12Compiler,
    PowerPreference,
    Features,
    DeviceDescriptor,
    RequestAdapterOptionsBase,
    Device,
    Surface,
    Queue,
    SurfaceConfiguration,
    TextureUsages,
    PresentMode,
    CompositeAlphaMode,
    Buffer,
    RenderPipeline,
    BindGroup,
    PipelineLayoutDescriptor,
    BindGroupLayout,
    BindGroupLayoutDescriptor,
    BindGroupLayoutEntry,
    ShaderStages,
    BindingType,
    BufferBindingType,
    RenderPipelineDescriptor,
    ShaderModuleDescriptor,
    ShaderSource,
    VertexState,
    PrimitiveState,
    PrimitiveTopology,
    FrontFace,
    PolygonMode,
    MultisampleState,
    FragmentState,
    ColorTargetState,
    TextureFormat,
    BlendState,
    ColorWrites,
    util::{ DeviceExt, BufferInitDescriptor },
    BufferUsages,
    BindGroupDescriptor,
    BindGroupEntry,
    CommandEncoderDescriptor,
    RenderPassDescriptor,
    RenderPassColorAttachment,
    TextureViewDescriptor,
    Operations,
    LoadOp,
    Color,
    IndexFormat,
};

use crate::mesh::{ Vertex, Index, Mesh, VERTEX_BUFFER_LAYOUT };

/// MeshId for a square mesh.
// Note that an actually qaud mesh is pushed onto meshes
// when the context is created, and this is tied to that.
pub const QUAD_MESH: MeshId = MeshId(0);

/// Id for accessing a mesh resource.
#[derive(Clone, Copy)]
pub struct MeshId(usize);

/// Id for accessing a texture resource.
#[derive(Clone, Copy)]
pub struct TextureId(usize);

/// Data needed to render a mesh.
#[derive(Clone, Copy)]
pub struct RenderOperation {
    pub transform: [[f32; 4]; 4],
    pub mesh: MeshId,
}

pub struct GraphicsContext {
    pub(crate) device: Device,
    pub(crate) queue: Queue,
    pub(crate) surface: Surface,
    pub(crate) surface_config: SurfaceConfiguration,

    pub(crate) pipeline: RenderPipeline,
    bind_group_layout: BindGroupLayout,
    global_buffer: Buffer,
    bind_groups_and_buffers: Vec<(BindGroup, Buffer)>,

    meshes: Vec<Mesh>,
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
}

unsafe impl Zeroable for GlobalBuffer {}
unsafe impl Pod for GlobalBuffer {}

unsafe impl Zeroable for LocalBuffer {}
unsafe impl Pod for LocalBuffer {}

fn setup_pipeline(device: &Device) -> (BindGroupLayout, RenderPipeline) {
    let bind_group_layout = device.create_bind_group_layout(
        &(BindGroupLayoutDescriptor {
            label: None,
            entries: &[
                // globals
                BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::VERTEX_FRAGMENT,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                // locals
                BindGroupLayoutEntry {
                    binding: 1,
                    visibility: ShaderStages::VERTEX_FRAGMENT,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
        })
    );

    let layout = device.create_pipeline_layout(
        &(PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        })
    );

    let shader = device.create_shader_module(ShaderModuleDescriptor {
        label: None,
        source: ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
    });

    let pipeline = device.create_render_pipeline(
        &(RenderPipelineDescriptor {
            label: None,
            layout: Some(&layout),
            vertex: VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[VERTEX_BUFFER_LAYOUT],
            },
            fragment: Some(FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[
                    Some(ColorTargetState {
                        format: TextureFormat::Bgra8UnormSrgb,
                        blend: Some(BlendState::ALPHA_BLENDING),
                        write_mask: ColorWrites::ALL,
                    }),
                ],
            }),
            primitive: PrimitiveState {
                topology: PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: FrontFace::Ccw,
                cull_mode: None,
                unclipped_depth: false,
                polygon_mode: PolygonMode::Fill,
                conservative: false,
            },
            depth_stencil: None,
            multisample: MultisampleState { count: 1, mask: !0, alpha_to_coverage_enabled: false },
            multiview: None,
        })
    );

    (bind_group_layout, pipeline)
}

impl GraphicsContext {
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
        let instance = Instance::new(InstanceDescriptor {
            backends: Backends::PRIMARY,
            dx12_shader_compiler: Dx12Compiler::Fxc,
        });

        let surface = (unsafe { instance.create_surface(window) }).unwrap();

        let adapter = instance
            .request_adapter(
                &(RequestAdapterOptionsBase {
                    power_preference: PowerPreference::HighPerformance,
                    force_fallback_adapter: false,
                    compatible_surface: Some(&surface),
                })
            ).await
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                &(DeviceDescriptor {
                    label: None,
                    features: Features::POLYGON_MODE_LINE,
                    limits: Default::default(),
                }),
                None
            ).await
            .unwrap();

        let surface_config = SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_capabilities(&adapter).formats[0],
            width,
            height,
            present_mode: PresentMode::AutoVsync,
            alpha_mode: CompositeAlphaMode::Auto,
            view_formats: vec![],
        };
        surface.configure(&device, &surface_config);

        let (bind_group_layout, pipeline) = setup_pipeline(&device);

        let global_buffer = device.create_buffer_init(
            &(BufferInitDescriptor {
                label: None,
                contents: bytes_of(&GlobalBuffer::zeroed()),
                usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
            })
        );

        let quad_mesh = Mesh::load(
            &device,
            &[
                Vertex {
                    position: [-0.5, 0.5, 0.0],
                    normal: [0.0, 0.0, 1.0],
                    texture_coordinates: [0.0, 0.0],
                },
                Vertex {
                    position: [0.5, 0.5, 0.0],
                    normal: [0.0, 0.0, 1.0],
                    texture_coordinates: [1.0, 0.0],
                },
                Vertex {
                    position: [-0.5, -0.5, 0.0],
                    normal: [0.0, 0.0, 1.0],
                    texture_coordinates: [0.0, 1.0],
                },
                Vertex {
                    position: [0.5, -0.5, 0.0],
                    normal: [0.0, 0.0, 1.0],
                    texture_coordinates: [1.0, 1.0],
                },
            ],
            &[0, 1, 2, 1, 3, 2]
        );

        let meshes = vec![quad_mesh];

        Self {
            device,
            queue,
            surface,
            surface_config,

            pipeline,
            bind_group_layout,
            global_buffer,
            bind_groups_and_buffers: Vec::new(),

            meshes,
        }
    }

    /// Loads a mesh and returns a [MeshId] that refers to it.
    pub fn load_mesh(&mut self, vertices: &[Vertex], indices: &[Index]) -> MeshId {
        let mesh = Mesh::load(&self.device, vertices, indices);
        let id = MeshId(self.meshes.len());
        self.meshes.push(mesh);
        id
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
                        &(BufferInitDescriptor {
                            label: None,
                            contents: bytes_of(&LocalBuffer::zeroed()),
                            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
                        })
                    );

                    let bind_group = self.device.create_bind_group(
                        &(BindGroupDescriptor {
                            label: None,
                            layout: &self.bind_group_layout,
                            entries: &[
                                BindGroupEntry {
                                    binding: 0,
                                    resource: self.global_buffer.as_entire_binding(),
                                },
                                BindGroupEntry {
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
        let view = surface_texture.texture.create_view(&TextureViewDescriptor::default());

        let mut command_encoder = self.device.create_command_encoder(
            &(CommandEncoderDescriptor { label: None })
        );

        {
            let mut render_pass = command_encoder.begin_render_pass(
                &(RenderPassDescriptor {
                    label: None,
                    color_attachments: &[
                        Some(RenderPassColorAttachment {
                            view: &view,
                            resolve_target: None,
                            ops: Operations {
                                load: LoadOp::Clear(Color {
                                    r: 0.1,
                                    g: 0.2,
                                    b: 0.3,
                                    a: 1.0,
                                }),
                                store: true,
                            },
                        }),
                    ],
                    depth_stencil_attachment: None,
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
                };
                self.queue.write_buffer(buffer, 0, bytes_of(&local_buffer));
                render_pass.set_bind_group(0, bind_group, &[]);

                let mesh = self.meshes.get(operation.mesh.0).unwrap();

                render_pass.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
                render_pass.set_index_buffer(mesh.index_buffer.slice(..), IndexFormat::Uint32);
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
    pub(crate) fn resize_surface(&mut self, width: u32, height: u32) {
        self.surface_config.width = width;
        self.surface_config.height = height;
        self.surface.configure(&self.device, &self.surface_config);
    }
}
