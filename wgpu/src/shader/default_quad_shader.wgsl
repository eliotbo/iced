struct Globals {
    transform: mat4x4<f32>,
    scale: f32,
}

@group(0) @binding(0) var<uniform> globals: Globals;

struct VertexInput {
    @location(0) v_pos: vec2<f32>,
    @location(1) pos: vec2<f32>,
    @location(2) size: vec2<f32>,
    @location(3) bg_color: vec4<f32>,


    @location(4) mouse_position: vec2<f32>,
    @location(5) mouse_click: vec2<f32>,
    @location(6) time: f32,
    @location(7) frame: u32, // unused in this example
}

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) bg_color: vec4<f32>,
    @location(1) pos: vec2<f32>,
    @location(2) size: vec2<f32>,
    @location(3) mouse_position: vec2<f32>,
    @location(4) mouse_click: vec2<f32>,
    @location(5) time: f32,
    @location(6) frame: u32,
}

@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    var out: VertexOutput;

    var pos: vec2<f32> = input.pos * globals.scale;
    var scale: vec2<f32> = input.size * globals.scale;

    var transform: mat4x4<f32> = mat4x4<f32>(
        vec4<f32>(scale.x + 1.0, 0.0, 0.0, 0.0),
        vec4<f32>(0.0, scale.y + 1.0, 0.0, 0.0),
        vec4<f32>(0.0, 0.0, 1.0, 0.0),
        vec4<f32>(pos - vec2<f32>(0.5, 0.5), 0.0, 1.0)
    );

    out.bg_color = input.bg_color;
    out.pos = pos;
    out.size = scale;
    out.position = globals.transform * transform * vec4<f32>(input.v_pos, 0.0, 1.0);

    out.mouse_position = input.mouse_position;
    out.mouse_click = input.mouse_click;
    out.time = input.time;
    out.frame = input.frame;

    return out;
}

@fragment
fn fs_main(
    input: VertexOutput
) -> @location(0) vec4<f32> {

    return vec4<f32>(0.0, 0.0, 0.0, 0.5);
}
