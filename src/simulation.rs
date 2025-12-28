use wgpu::*;

use crate::renderer::pipelines::Pipelines;

/// The extent of the simulation across the X and Z axes.
pub const SIMULATION_LENGTH: f32 = 5.0;
/// How many intervals the simulation is subdivided across each axis.
pub const SIMULATION_RESOLUTION: usize = 10;

/// Manages all GPU state to numerically solve the wave equation.
///
/// The wave state is represented by two storage textures in the [`TextureFormat::Rg32Float`] format,
/// with the red channel representing u(x, t) and the green channel representing u(x, t-1).
pub struct WaveSimulation {
    /// Which texture is currently being read / written to.
    /// - if `active` is even, `a` is the "read" texture and `b` is the "write" texture,
    /// -  if `active` is odd, `a` is the "write" texture and `b` is the "read" texture.
    active: usize,

    /// Storage texture 'a' of the simulation.
    texture_a: Texture,
    /// Storage texture 'b' of the simulation.
    texture_b: Texture,

    /// The bind group for storing a view and sampler for *reading* from `texture_a`.
    bind_group_read_a: BindGroup,
    /// The bind group for storing a view and sampler for *reading* from `texture_b`.
    bind_group_read_b: BindGroup,

    /// The bind group for storing a view for *writing* to `texture_a`.
    bind_group_write_a: BindGroup,
    /// The bind group for storing a view for *writing* to `texture_b`.
    bind_group_write_b: BindGroup,
}

impl WaveSimulation {
    /// Creates all resources to run the [`WaveSimulation`].
    pub fn new(device: &Device, pipelines: &Pipelines) -> Self {
        let (texture_a, bind_group_read_a, bind_group_write_a) =
            Self::create_texture_resources(device, pipelines, "a");

        let (texture_b, bind_group_read_b, bind_group_write_b) =
            Self::create_texture_resources(device, pipelines, "b");
        Self {
            active: 0,
            texture_a,
            texture_b,
            bind_group_read_a,
            bind_group_read_b,
            bind_group_write_a,
            bind_group_write_b,
        }
    }

    /// Creates a storage texture and its corresponding bind groups (in the order: read, write).
    fn create_texture_resources(
        device: &Device,
        pipelines: &Pipelines,
        name: &str,
    ) -> (Texture, BindGroup, BindGroup) {
        let xz_length = (SIMULATION_LENGTH * SIMULATION_RESOLUTION as f32) as u32;

        let texture = device.create_texture(&TextureDescriptor {
            label: Some(&format!("WaveSimulation::texture_{name}")),
            size: Extent3d {
                width: xz_length,
                height: xz_length,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: TextureFormat::Rg32Float,
            usage: TextureUsages::STORAGE_BINDING | TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });

        let view = texture.create_view(&TextureViewDescriptor::default());
        let sampler = device.create_sampler(&SamplerDescriptor::default());

        let bind_group_read = device.create_bind_group(&BindGroupDescriptor {
            label: Some(&format!("WaveSimulation::texture_bind_group_read_{name}")),
            layout: &pipelines.texture_read_bind_group_layout,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: BindingResource::TextureView(&view),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: BindingResource::Sampler(&sampler),
                },
            ],
        });

        let bind_group_write = device.create_bind_group(&BindGroupDescriptor {
            label: Some(&format!("WaveSimulation::texture_bind_group_write_{name}")),
            layout: &pipelines.texture_write_bind_group_layout,
            entries: &[BindGroupEntry {
                binding: 0,
                resource: BindingResource::TextureView(&view),
            }],
        });

        (texture, bind_group_read, bind_group_write)
    }
}
