mod custom_shader_quad {
    use iced_native::layout::{self, Layout};
    use iced_native::widget::{self, Widget};
    use iced_native::{
        renderer, Color, Element, Length, Point, Rectangle, Size, Vector,
    };

    use std::time::Duration;

    // import the shader from the src directory
    const SHADER: &str = include_str!("custom_shader.wgsl");

    #[derive(Default)]
    pub struct CustomQuad {
        size: f32,
        mouse_click: Vector,
        duration_since_start: Duration,
    }

    impl CustomQuad {
        pub fn new(
            size: f32,
            mouse_click: Vector,
            duration_since_start: Duration,
        ) -> Self {
            Self {
                duration_since_start,
                size,
                mouse_click,
            }
        }
    }

    impl<Message, Renderer> Widget<Message, Renderer> for CustomQuad
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
                    shader_code: SHADER.to_string(),
                    mouse_position: cursor_position,
                    mouse_click: self.mouse_click,
                    time: self.duration_since_start.as_secs_f32(),
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

    impl<'a, Message, Renderer> From<CustomQuad> for Element<'a, Message, Renderer>
    where
        Renderer: renderer::Renderer,
    {
        fn from(circle: CustomQuad) -> Self {
            Self::new(circle)
        }
    }
}

use iced::widget::{column, container, text};
use iced::{
    executor, mouse, time, Alignment, Application, Command, Element, Length,
    Settings, Subscription, Theme, Vector,
};
use iced_native::subscription;
use std::time::{Duration, Instant};

pub fn main() -> iced::Result {
    Example::run(Settings {
        antialiasing: true,
        ..Settings::default()
    })
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ModifierKeys {
    pub shift: bool,
    pub ctrl: bool,
    pub alt: bool,
    pub logo: bool,
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
    duration_since_start: Duration,

    first_tick: Instant,
    mouse_click: Vector,
    #[allow(dead_code)]
    custom_shader_quad: custom_shader_quad::CustomQuad,
}

impl Application for Example {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            Self {
                duration_since_start: Duration::default(),
                mouse_click: Vector::new(0.0, 0.0),

                first_tick: Instant::now(),
                custom_shader_quad: custom_shader_quad::CustomQuad::new(
                    200.0,
                    Vector::default(),
                    Duration::default(),
                ),
                // modifier_keys: ModifierKeys::default(),
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
                self.mouse_click.y = 0.0;
            }

            Message::LeftMouseReleaseGeneral => {
                self.mouse_click.x = 0.0;
            }

            Message::RightMousePressedGeneral => {
                self.mouse_click.y = 1.0;
            }

            Message::LeftMousePressedGeneral => {
                self.mouse_click.x = 1.0;
            }

            Message::Tick(now) => {
                self.duration_since_start += now - self.first_tick;
                self.first_tick = now;
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
            custom_shader_quad::CustomQuad::new(
                200.0,
                self.mouse_click,
                self.duration_since_start,
            ),
        ]
        .width(Length::Fill)
        .spacing(20)
        .align_items(Alignment::Center);

        // container(scrollable(content))
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
