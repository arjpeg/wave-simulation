use wgpu::*;

use crate::renderer::{frame::DEPTH_FORMAT, gpu_context::SURFACE_VIEW_FORMAT, shaders::Shaders};

/// Manages the creation and lifecycle of all pipelines and their associated bind group layouts.
pub struct Pipelines {
    /// The pipeline used for rendering a triangle.
    pub triangle_pipeline: RenderPipeline,
    /// The bind group layout for holding a camera's transformation matrix.
    pub camera_bind_group_layout: BindGroupLayout,

    /// The compute pipeline used for advancing the state of the wave simulation by one "tick".
    pub simulation_pipeline: ComputePipeline,
    /// The bind group layout for reading a (non-storage) texture, using its view and sampler.
    pub texture_read_bind_group_layout: BindGroupLayout,
    /// The bind group layout for writing to a (storage) texture, using its view.
    pub texture_write_bind_group_layout: BindGroupLayout,
}

impl Pipelines {
    /// Creates all the [`Pipelines`] given their associated shaders.
    pub fn new(device: &Device, shaders: &Shaders) -> Self {
        let camera_bind_group_layout =
            device.create_bind_group_layout(&BindGroupLayoutDescriptor {
                label: Some("Pipelines::camera_bind_group_layout"),
                entries: &[BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::VERTEX,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
            });

        let triangle_pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("Pipelines::triangle_pipeline_layout"),
            bind_group_layouts: &[&camera_bind_group_layout],
            push_constant_ranges: &[],
        });

        let triangle_pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some("Pipelines::triangle_pipeline"),
            layout: Some(&triangle_pipeline_layout),
            vertex: VertexState {
                module: &shaders.triangle_shader,
                entry_point: Some("vs_main"),
                compilation_options: PipelineCompilationOptions::default(),
                buffers: &[],
            },
            fragment: Some(FragmentState {
                module: &shaders.triangle_shader,
                entry_point: Some("fs_main"),
                compilation_options: PipelineCompilationOptions::default(),
                targets: &[Some(ColorTargetState {
                    format: SURFACE_VIEW_FORMAT,
                    blend: None,
                    write_mask: ColorWrites::ALL,
                })],
            }),
            primitive: PrimitiveState::default(),
            multisample: MultisampleState::default(),
            depth_stencil: Some(DepthStencilState {
                format: DEPTH_FORMAT,
                depth_write_enabled: true,
                depth_compare: CompareFunction::LessEqual,
                stencil: StencilState::default(),
                bias: DepthBiasState::default(),
            }),
            multiview: None,
            cache: None,
        });

        let texture_read_bind_group_layout =
            device.create_bind_group_layout(&BindGroupLayoutDescriptor {
                label: Some("Pipelines::texture_read_bind_group_layout"),
                entries: &[
                    BindGroupLayoutEntry {
                        binding: 0,
                        visibility: ShaderStages::COMPUTE,
                        ty: BindingType::Texture {
                            sample_type: TextureSampleType::Float { filterable: true },
                            view_dimension: TextureViewDimension::D2,
                            multisampled: false,
                        },
                        count: None,
                    },
                    BindGroupLayoutEntry {
                        binding: 1,
                        visibility: ShaderStages::COMPUTE,
                        ty: BindingType::Sampler(SamplerBindingType::Filtering),
                        count: None,
                    },
                ],
            });

        let texture_write_bind_group_layout =
            device.create_bind_group_layout(&BindGroupLayoutDescriptor {
                label: Some("Pipelines::texture_write_bind_group_layout"),
                entries: &[BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::COMPUTE,
                    ty: BindingType::StorageTexture {
                        access: StorageTextureAccess::WriteOnly,
                        format: TextureFormat::Rg32Float,
                        view_dimension: TextureViewDimension::D2,
                    },
                    count: None,
                }],
            });

        let simulation_pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("Pipelines::simulation_pipeline_layout"),
            bind_group_layouts: &[
                &texture_read_bind_group_layout,
                &texture_write_bind_group_layout,
            ],
            push_constant_ranges: &[],
        });

        let simulation_pipeline = device.create_compute_pipeline(&ComputePipelineDescriptor {
            label: Some("Pipelines::simulation_pipeline_layout"),
            layout: Some(&simulation_pipeline_layout),
            module: &shaders.simulation_shader,
            entry_point: Some("main"),
            compilation_options: PipelineCompilationOptions::default(),
            cache: None,
        });

        Self {
            triangle_pipeline,
            camera_bind_group_layout,
            simulation_pipeline,
            texture_read_bind_group_layout,
            texture_write_bind_group_layout,
        }
    }
}
