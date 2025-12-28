use wgpu::{Device, ShaderModule, include_wgsl};

/// All compiled and hot reloadable shaders used in the application.
pub struct Shaders {
    /// The shader used for drawing a triangle.
    pub triangle_shader: ShaderModule,

    /// The shader used for running a wave simulation compute pass.
    pub simulation_shader: ShaderModule,
}

impl Shaders {
    /// Creates and compiles all shaders.
    pub fn new(device: &Device) -> Self {
        let triangle_shader =
            device.create_shader_module(include_wgsl!("../../assets/triangle_shader.wgsl"));

        let simulation_shader =
            device.create_shader_module(include_wgsl!("../../assets/simulation.wgsl"));

        Self {
            triangle_shader,
            simulation_shader,
        }
    }
}
