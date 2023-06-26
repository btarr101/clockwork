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
}
@group(0) @binding(1)
var<uniform> local: Local;


@vertex
fn vs_main(
    in: VertexInput,
    @builtin(vertex_index) in_vertex_index: u32,
) -> VertexOutput {
    var out: VertexOutput;
    let vertex_transform = local.transform * vec4<f32>(in.position, 1.0);
    out.clip_position = global.mvp * vertex_transform;
    out.uv = in.uv;
    return out;
}

// Fragment shader
@fragment
fn fs_main(
    in: VertexOutput,    
) -> @location(0) vec4<f32> {
    return vec4<f32>(in.uv, 0.1, 1.0);
}