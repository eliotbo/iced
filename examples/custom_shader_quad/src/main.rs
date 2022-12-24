mod custom_shader_quad {
    use iced_native::layout::{self, Layout};

    use iced_native::widget::{self, Widget};
    use iced_native::{
        renderer, Color, Element, Length, Point, Rectangle, Size, Vector,
    };

    use std::time::Duration;

    #[derive(Default)]
    pub struct StarMoon {
        size: f32,
        mouse_click: Vector,
        duration_since_click: Duration,
        shader_path: std::path::PathBuf,
    }

    impl StarMoon {
        pub fn new(
            size: f32,
            mouse_click: Vector,
            duration_since_click: Duration,
            shader_path: std::path::PathBuf,
        ) -> Self {
            Self {
                size,
                mouse_click,
                duration_since_click,
                shader_path: shader_path,
            }
        }
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
                    shader_path: self.shader_path.clone(),

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
use iced_native::subscription;
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
    mouse_click: Vector,
    #[allow(dead_code)]
    custom_shader_quad: custom_shader_quad::StarMoon,
}

impl Example {
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
                mouse_click: Vector::new(0.0, 0.0),
                custom_shader_quad: custom_shader_quad::StarMoon::new(
                    200.0,
                    Vector::default(),
                    Duration::default(),
                    Example::get_path(),
                ),
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
            custom_shader_quad::StarMoon::new(
                200.0,
                self.mouse_click,
                self.duration_since_last_click,
                Example::get_path(),
            ),
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
