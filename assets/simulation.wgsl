@group(0) @binding(0)
var current: texture_2d<f32>;
@group(0) @binding(1)
var next: texture_storage_2d<rg32float, write>;

const PI: f32 = 3.14159;

@compute
@workgroup_size(16, 16, 1)
fn main(
    @builtin(global_invocation_id) id: vec3<u32>,
) {
    let current_value = textureLoad(current, id.xy, 0);

    // textureStore(next, id.xy, vec4<f32>(current_value.x + 0.01, 0.0, 0.0,

    let uv = vec2<f32>(id.xy) / vec2<f32>(textureDimensions(current));
    let value = sin(8.0 * PI * uv.x) * sin(8.0 * PI * uv.y) * exp(-4.0 * distance(vec2<f32>(0.5), uv));

    textureStore(
        next,
        id.xy,
        vec4<f32>(value, 0.0, 0.0, 0.0)
    );
}

