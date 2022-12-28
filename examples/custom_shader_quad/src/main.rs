mod custom_shader_quad {
    use iced_native::{
        layout::{self, Layout},
        renderer, shader,
        widget::{self, Widget},
        Color, Element, Length, Point, Rectangle, Size,
    };

    use std::time::Duration;

    pub const SHADER: &str = r#"
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
    @location(5) mouse_click: u32,
    @location(6) time: f32,
    @location(7) frame: u32, // unused in this example
}

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) bg_color: vec4<f32>,
    @location(1) pos: vec2<f32>,
    @location(2) size: vec2<f32>,
    @location(3) mouse_position: vec2<f32>,
    @location(4) mouse_click: u32,
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
    if input.mouse_click == 1 {
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
    if input.mouse_click == 2 {
        shape_color = vec4<f32>(0.84, 0.05, 0.92, 1.); // purple
    }

    var color = mix(bg_color, shape_color, 1. - d);

    // Add circle around mouse position. 
    // The units are in pixels instead of normalized coordinates.
    var mouse_radius = 8.0;
    if input.mouse_click == 1 || input.mouse_click == 2 {
        mouse_radius = 3.0;
    }
    let d_mouse_circle = sdCircle(input.position.xy - input.mouse_position, mouse_radius);
    let mouse_contribution = (1.0 - smoothstep(0.0, 2.0, d_mouse_circle));


    color = mix(color, mouse_color, mouse_contribution);
    return color;
}

        "#;

    pub struct StarMoon {
        pub size: f32,
        pub mouse_click: u32,
        pub duration_since_click: Duration,
        pub handle: shader::Handle,
    }

    impl<Message, Renderer> Widget<Message, Renderer> for StarMoon
    where
        Renderer: renderer::Renderer,
    {
        fn width(&self) -> Length {
            Length::Shrink
        }

        fn height(&self) -> Length {
            Length::Shrink
        }

        fn layout(
            &self,
            _renderer: &Renderer,
            _limits: &layout::Limits,
        ) -> layout::Node {
            layout::Node::new(Size::new(self.size, self.size))
        }

        fn draw(
            &self,
            _state: &widget::Tree,
            renderer: &mut Renderer,
            _theme: &Renderer::Theme,
            _style: &renderer::Style,
            layout: Layout<'_>,
            cursor_position: Point,
            _viewport: &Rectangle,
        ) {
            renderer.make_custom_shader_quad(
                renderer::CustomShaderQuad {
                    bounds: layout.bounds(),
                    handle: self.handle.clone(),

                    // The following fields have fixed names and types, but users can pass
                    // in any data they want the shader to receive as long as the types match.
                    mouse_position: cursor_position,
                    mouse_click: self.mouse_click,
                    time: self.duration_since_click.as_secs_f32(),
                    frame_number: 1,
                },
                // same color as background
                Color::from_rgb(
                    32 as f32 / 255.0,
                    34 as f32 / 255.0,
                    37 as f32 / 255.0,
                ),
            );
        }
    }

    impl<'a, Message, Renderer> From<StarMoon> for Element<'a, Message, Renderer>
    where
        Renderer: renderer::Renderer,
    {
        fn from(circle: StarMoon) -> Self {
            Self::new(circle)
        }
    }
}

use iced::widget::{column, container, text};
use iced::{
    executor, mouse, time, Alignment, Application, Command, Element, Length,
    Settings, Subscription, Theme, Vector,
};
use iced_native::{shader::ShaderContent, subscription};
use std::time::{Duration, Instant};

// import the shader from the src directory
const SHADER_NAME: &str = "custom_shader.wgsl";

pub fn main() -> iced::Result {
    Example::run(Settings {
        ..Settings::default()
    })
}

#[derive(Debug, Clone, Copy)]
#[allow(clippy::enum_variant_names)]
enum Message {
    RightMouseReleaseGeneral,
    LeftMouseReleaseGeneral,
    RightMousePressedGeneral,
    LeftMousePressedGeneral,
    Tick(Instant),
    None,
}

struct Example {
    duration_since_last_click: Duration,
    tick_at_last_click: Instant,
    mouse_click: u32,
    #[allow(dead_code)]
    custom_shader_quad: custom_shader_quad::StarMoon,
}

impl Example {
    #[allow(dead_code)]
    fn get_path() -> std::path::PathBuf {
        let relative_shader_path: std::path::PathBuf = SHADER_NAME.into();

        let relative_module_path = std::path::Path::new("examples")
            .join("custom_shader_quad")
            .join("src")
            .join(relative_shader_path);

        let absolute_path =
            std::env::current_dir().unwrap().join(relative_module_path);

        return absolute_path;
    }
}

impl Application for Example {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            Self {
                duration_since_last_click: Duration::default(),
                tick_at_last_click: Instant::now(),
                mouse_click: 0,
                custom_shader_quad: custom_shader_quad::StarMoon {
                    size: 200.0,
                    mouse_click: 0,
                    duration_since_click: Duration::default(),
                    handle: ShaderContent::Path(Example::get_path()).into(),
                    // handle: ShaderContent::Memory(custom_shader_quad::SHADER)
                    //     .into(),
                },
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Custom shader - Iced")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::RightMouseReleaseGeneral => {
                self.mouse_click = 1;
            }

            Message::LeftMouseReleaseGeneral => {
                self.mouse_click = 0;
            }

            Message::RightMousePressedGeneral => {
                self.mouse_click = 2;
            }

            Message::LeftMousePressedGeneral => {
                self.mouse_click = 0;
                self.tick_at_last_click = Instant::now();
            }

            Message::Tick(now) => {
                self.duration_since_last_click = now - self.tick_at_last_click;
            }

            _ => (),
        }

        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let instruction =
            column![text("Interact using mouse presses")].spacing(10);

        let content = column![
            instruction,
            custom_shader_quad::StarMoon {
                size: 200.0,
                mouse_click: self.mouse_click,
                duration_since_click: self.duration_since_last_click,
                // shader_content: Example::get_path(),
                handle: self.custom_shader_quad.handle.clone(),
            },
        ]
        .width(Length::Fill)
        .spacing(20)
        .align_items(Alignment::Center);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(5)
            .center_y()
            .into()
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }

    fn subscription(&self) -> Subscription<Message> {
        let mouse_sub =
            subscription::events_with(|event, _status| match event {
                iced_native::Event::Mouse(mouse_event) => match mouse_event {
                    mouse::Event::ButtonPressed(button) => {
                        let message = match button {
                            mouse::Button::Left => {
                                Message::LeftMousePressedGeneral
                            }
                            mouse::Button::Right => {
                                Message::RightMousePressedGeneral
                            }
                            _ => Message::None,
                        };
                        Some(message)
                    }
                    mouse::Event::ButtonReleased(button) => {
                        let message = match button {
                            mouse::Button::Left => {
                                Message::LeftMouseReleaseGeneral
                            }
                            mouse::Button::Right => {
                                Message::RightMouseReleaseGeneral
                            }
                            _ => Message::None,
                        };
                        Some(message)
                    }
                    _ => None,
                },

                _ => None,
            });

        let time_sub =
            time::every(Duration::from_millis(10)).map(Message::Tick);

        Subscription::batch(vec![mouse_sub, time_sub])
    }
}
