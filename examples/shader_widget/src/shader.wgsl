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


/////////////////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////////////////

// 2d SDFs (surface distance functions)
// see more at https://gist.github.com/munrocket/30e645d584b5300ee69295e54674b3e4


// MIT License. © 2020 Inigo Quilez, Munrocket
// Permission is hereby granted, free of charge, to any person obtaining a copy of 
// this software and associated documentation files (the "Software"), to deal in the 
// Software without restriction, including without limitation the rights to use, copy, 
// modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, 
// and to permit persons to whom the Software is furnished to do so, subject to the 
// following conditions:

// The above copyright notice and this permission notice shall be included in all copies 
// or substantial portions of the Software.

// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, 
// INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A 
// PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT 
// HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF 
// CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE 
// OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

fn sdStar(p: vec2<f32>, r: f32, n: u32, m: f32) -> f32 {
    let an = 3.141593 / f32(n);
    let en = 3.141593 / m;
    let acs = vec2<f32>(cos(an), sin(an));
    let ecs = vec2<f32>(cos(en), sin(en));
    let bn = (atan2(abs(p.x), p.y) % (2. * an)) - an;
    var q: vec2<f32> = length(p) * vec2<f32>(cos(bn), abs(sin(bn)));
    q = q - r * acs;
    q = q + ecs * clamp(-dot(q, ecs), 0., r * acs.y / ecs.y);
    return length(q) * sign(q.x);
}

fn sdMoon(p: vec2<f32>, d: f32, ra: f32, rb: f32) -> f32 {
    let q = vec2<f32>(p.x, abs(p.y));
    let a = (ra * ra - rb * rb + d * d) / (2. * d);
    let b = sqrt(max(ra * ra - a * a, 0.));
    if d * (q.x * b - q.y * a) > d * d * max(b - q.y, 0.) { return length(q - vec2<f32>(a, b)); }
    return max((length(q) - ra), -(length(q - vec2<f32>(d, 0.)) - rb));
}

fn sdCircle(p: vec2<f32>, r: f32) -> f32 {
    return length(p) - r;
}

/////////////////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////////////////


@fragment
fn fs_main(
    input: VertexOutput
) -> @location(0) vec4<f32> {
    let bg_color: vec4<f32> = input.bg_color;

    // normalize fragment position
    let p = (input.position.xy - input.pos.xy - input.size.xy / 2.0) / input.size.xy;

    // upon right mouse button press, change shape size
    var breathing = 1.0;
    if input.mouse_click.x > 0.5 {
        breathing = (1.0 + cos(input.time * 5.0)) / 2.0;
    }

    // surface distance function (SDF) for the star
    let d_star = sdStar(p - vec2<f32>(0.05, 0.0), 0.23 * breathing, 5u, 0.35);


    let md = 0.3 ;
    let mra = 0.35 ;
    let mrb = 0.35 ;

    // SDF for the moon
    let d_moon = sdMoon(p, md, mra, mrb);

    // Union of the star and the moon
    let d_min = min(d_star, d_moon);

    // only render the border of the shape
    let d = smoothstep(0.0, 0.01, abs(d_min));


    var shape_color = vec4<f32>(0.75, 0.5, 0.0, 1.0); // orange
    var mouse_color = vec4<f32>(0.01, 0.2, 0.3, 1.0); // bluish

    // upon left mouse button press, change border color
    if input.mouse_click.y > 0.5 {
        shape_color = vec4<f32>(0.84, 0.05, 0.92, 1.); // purple
    }

    var color = mix(bg_color, shape_color, 1. - d);

    // Add circle around mouse position. 
    // The units are in pixels instead of normalized coordinates.
    var mouse_radius = 8.0;
    if input.mouse_click.y > 0.5 || input.mouse_click.x > 0.5 {
        mouse_radius = 3.0;
    }
    let d_mouse_circle = sdCircle(input.position.xy - input.mouse_position, mouse_radius);
    let mouse_contribution = (1.0 - smoothstep(0.0, 2.0, d_mouse_circle));


    color = mix(color, mouse_color, mouse_contribution);
    return color;
}
