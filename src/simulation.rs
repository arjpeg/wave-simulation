use wgpu::*;

use crate::renderer::pipelines::Pipelines;

/// The extent of the simulation across the X and Z axes.
pub const SIMULATION_LENGTH: f32 = 5.0;
/// How many intervals the simulation is subdivided across each axis.
pub const SIMULATION_RESOLUTION: usize = 500;

/// Manages all GPU state to numerically solve the wave equation.
///
/// The wave state is represented by two storage textures in the [`TextureFormat::Rg32Float`] format,
/// with the red channel representing u(x, t) and the green channel representing u(x, t-1).
#[allow(unused)]
pub struct WaveSimulation {
    /// Which texture is currently being read / written to.
    /// - if `active` is even, `a` is the "read" texture and `b` is the "write" texture,
    /// -  if `active` is odd, `a` is the "write" texture and `b` is the "read" texture.
    active: usize,

    /// Storage texture 'a' of the simulation.
    texture_a: Texture,
    /// Storage texture 'b' of the simulation.
    texture_b: Texture,

    /// The bind group holding `texture_a` as the "read" texture and `texture_b` as the "write"
    /// texture.
    a_read_b_write_bind_group: BindGroup,
    /// The bind group holding `texture_a` as the "write" texture and `texture_b` as the "read"
    /// texture.
    b_read_a_write_bind_group: BindGroup,
}

impl WaveSimulation {
    /// Creates all resources to run the [`WaveSimulation`].
    pub fn new(device: &Device, pipelines: &Pipelines) -> Self {
        let texture_a = Self::create_compute_texture(device, "a");
        let texture_b = Self::create_compute_texture(device, "b");

        let a_read_b_write_bind_group = Self::create_read_write_bind_group(
            device,
            pipelines,
            &texture_a,
            &texture_b,
            "a_read_b_write",
        );

        let b_read_a_write_bind_group = Self::create_read_write_bind_group(
            device,
            pipelines,
            &texture_b,
            &texture_a,
            "b_read_a_write",
        );

        Self {
            active: 0,
            texture_a,
            texture_b,
            a_read_b_write_bind_group,
            b_read_a_write_bind_group,
        }
    }

    /// Returns the currently "active" (read) texture's view in a [`BindGroup`] in slot 0.
    pub fn get_active_texture(&self) -> &BindGroup {
        if self.active % 2 == 0 {
            &self.a_read_b_write_bind_group
        } else {
            &self.b_read_a_write_bind_group
        }
    }

    /// Excecutes the simulation compute pipeline, advancing the simulation by one "tick".
    pub fn tick(&mut self, encoder: &mut CommandEncoder, pipelines: &Pipelines) {
        let mut pass = encoder.begin_compute_pass(&ComputePassDescriptor {
            label: Some("WaveSimulation::tick"),
            timestamp_writes: None,
        });

        let active_bind_group = if self.active % 2 == 0 {
            &self.a_read_b_write_bind_group
        } else {
            &self.b_read_a_write_bind_group
        };

        // the shader has a workgroup size of 16x16x1
        let x = (self.texture_a.width() as f32 / 16.0).ceil() as u32;
        let y = (self.texture_a.height() as f32 / 16.0).ceil() as u32;

        pass.set_pipeline(&pipelines.simulation_pipeline);
        pass.set_bind_group(0, active_bind_group, &[]);
        pass.dispatch_workgroups(x, y, 1);

        self.active += 1;
    }

    /// Creates a storage [`Texture`] appropriate for use in the simulation.
    fn create_compute_texture(device: &Device, label: &str) -> Texture {
        let xz_length = (SIMULATION_LENGTH * SIMULATION_RESOLUTION as f32) as u32;

        device.create_texture(&TextureDescriptor {
            label: Some(&format!("WaveSimulation::texture_{label}")),
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
        })
    }

    /// Creates a [`BindGroup`] holding the `left` texture in the read position and the `right`
    /// texture in the write position, corresponding to [`Pipelines::texture_read_write_bind_group_layout`].
    fn create_read_write_bind_group(
        device: &Device,
        pipelines: &Pipelines,
        left: &Texture,
        right: &Texture,
        label: &str,
    ) -> BindGroup {
        let left_view = left.create_view(&TextureViewDescriptor::default());
        let right_view = right.create_view(&TextureViewDescriptor::default());

        device.create_bind_group(&BindGroupDescriptor {
            label: Some(&format!("WaveSimulation::{label}_bind_group")),
            layout: &pipelines.texture_read_write_bind_group_layout,
            entries: &[
                // read texture
                BindGroupEntry {
                    binding: 0,
                    resource: BindingResource::TextureView(&left_view),
                },
                // write texture
                BindGroupEntry {
                    binding: 1,
                    resource: BindingResource::TextureView(&right_view),
                },
            ],
        })
    }
}
