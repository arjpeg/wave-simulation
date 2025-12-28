use bytemuck::{Pod, Zeroable};
use itertools::{Itertools, iproduct};
use wgpu::{
    Buffer, BufferUsages, Device, VertexBufferLayout, VertexStepMode,
    util::{BufferInitDescriptor, DeviceExt},
    vertex_attr_array,
};

use crate::simulation::{SIMULATION_LENGTH, SIMULATION_RESOLUTION};

/// A vertex on a [`SurfaceMesh`].
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Zeroable, Pod)]
#[repr(C)]
pub struct SurfaceVertex {
    /// The world space position of the vertex.
    pub position: [f32; 3],
    /// The uv of the vertex along the surface.
    pub uv: [f32; 2],
}

/// The mesh for a flat, subdivided plane centered at the origin.
pub struct SurfaceMesh {
    /// The vertices making up the mesh stored on the GPU.
    pub vertex_buffer: Buffer,
    /// The indices making up the mesh stored on the GPU (stored as a collection of [`u32`]'ss).
    pub index_buffer: Buffer,

    /// The count of indices to render.
    pub index_count: u32,
}

impl SurfaceMesh {
    /// Creates a new [`SurfaceMesh`].
    pub fn new(device: &Device) -> Self {
        // the vertex positions along a single axis
        let axis_vertices = (0..SIMULATION_RESOLUTION)
            .map(|i| (i as f32 / SIMULATION_RESOLUTION as f32)) // map to [0, 1]
            .map(|t| (t, t * SIMULATION_LENGTH));

        let vertices = iproduct!(axis_vertices.clone(), axis_vertices)
            .map(|((uv_x, x), (uv_z, z))| SurfaceVertex {
                position: [x, 0.0, z],
                uv: [uv_x, uv_z],
            })
            .collect_vec();

        let indices = (0..SIMULATION_RESOLUTION - 1)
            .flat_map(move |row| {
                (0..SIMULATION_RESOLUTION - 1).flat_map(move |col| {
                    let row_offset = row * SIMULATION_RESOLUTION;
                    [
                        // triangle 1
                        row_offset + col,                         // bottom left
                        row_offset + SIMULATION_RESOLUTION + col, // top left
                        row_offset + col + 1,                     // bottom right
                        // triangle 2
                        row_offset + col + 1, // bottom right
                        row_offset + SIMULATION_RESOLUTION + col + 1, // top right
                        row_offset + SIMULATION_RESOLUTION + col, // top left
                    ]
                })
            })
            .map(|index| index as u32)
            .collect_vec();

        let vertex_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("SurfaceMesh::vertex_buffer"),
            contents: bytemuck::cast_slice(&vertices),
            usage: BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("SurfaceMesh::index_buffer"),
            contents: bytemuck::cast_slice(&indices),
            usage: BufferUsages::INDEX,
        });

        let index_count = indices.len() as u32;

        Self {
            vertex_buffer,
            index_buffer,
            index_count: index_count,
        }
    }
}

impl SurfaceVertex {
    /// A descriptor on how to interpret each [`SurfaceVertex`] in vertex [`Buffer`].
    pub const LAYOUT: VertexBufferLayout<'static> = VertexBufferLayout {
        array_stride: size_of::<Self>() as _,
        step_mode: VertexStepMode::Vertex,
        attributes: &vertex_attr_array![
            0 => Float32x3,
            1 => Float32x2
        ],
    };
}
