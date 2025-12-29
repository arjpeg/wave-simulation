struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) uv: vec2<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) normal: vec3<f32>,
    @location(1) position: vec3<f32>,
    @location(2) uv: vec2<f32>,
}

@group(0) @binding(0)
var<uniform> camera_view_projection: mat4x4<f32>;

@group(1) @binding(0)
var displacement_map: texture_2d<f32>;

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;

    let sample_coords = vec2<u32>(vec2<f32>(textureDimensions(displacement_map)) * in.uv);

    let height = height_at(sample_coords);
    let position = vec3<f32>(in.position.x, height, in.position.z);

    out.clip_position = camera_view_projection * vec4<f32>(position, 1.0);
    out.normal = approximate_normal(sample_coords);
    out.position = in.position;
    out.uv = in.uv;

    return out;
}

/// Approximates the normal vector at the given coordinates based on finite differences.
fn approximate_normal(coord: vec2<u32>) -> vec3<f32> {
    let dims = textureDimensions(displacement_map);

    let x0 = max(coord.x, 1u) - 1u;
    let x1 = min(coord.x + 1u, dims.x - 1u);
    let y0 = max(coord.y, 1u) - 1u;
    let y1 = min(coord.y + 1u, dims.y - 1u);

    let dh_dx = (height_at(vec2<u32>(x1, coord.y)) - height_at(vec2<u32>(x0, coord.y))) * 0.5;

    let dh_dz = (height_at(vec2<u32>(coord.x, y1)) - height_at(vec2<u32>(coord.x, y0))) * 0.5;

    let n = vec3<f32>(-dh_dx, 0.0, -dh_dz);
    return normalize(n);
}

/// Returns the height at the given coordinates.
fn height_at(coord: vec2<u32>) -> f32 {
    return textureLoad(displacement_map, coord, 0).r;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let normal = in.normal;
    let light_dir = normalize(vec3<f32>(0.5, 1.0, 0.3));
    let light_color = vec3<f32>(1.0, 1.0, 1.0);

    let diffuse = lambertian_shading(normal, light_dir, light_color);

    return vec4<f32>(vec3<f32>(in.uv, 0.0) * diffuse, 1.0);
}

fn lambertian_shading(normal: vec3<f32>, light_dir: vec3<f32>, light_color: vec3<f32>) -> vec3<f32> {
    let n_dot_l = max(dot(normal, light_dir), 0.05); // cosine factor
    return light_color * n_dot_l;
}
