struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) uv: vec2<f32>,
};

struct Global {
    mvp: mat4x4<f32>,
}
@group(0) @binding(0)
var<uniform> global: Global;

struct Local {
    transform: mat4x4<f32>,
    uv_window: vec4<f32>,
}
@group(0) @binding(1)
var<uniform> local: Local;


@group(1) @binding(0)
var texture: texture_2d<f32>;
@group(1) @binding(1)
var texture_sampler: sampler;

@vertex
fn vs_main(
    in: VertexInput,
    @builtin(vertex_index) in_vertex_index: u32,
) -> VertexOutput {
    var out: VertexOutput;
    let vertex_transform = local.transform * vec4<f32>(in.position, 1.0);
    out.clip_position = global.mvp * vertex_transform;
    out.uv = local.uv_window.xy + vec2(0.01, 0.01) + ((local.uv_window.zw - vec2(0.01, 0.01)) * in.uv);
    return out;
}

// Fragment shader
@fragment
fn fs_main(
    in: VertexOutput,    
) -> @location(0) vec4<f32> {
    let sample = textureSample(texture, texture_sampler, in.uv);
    if (sample.w < 0.001) {
        discard;
    }
    
    return sample;
}