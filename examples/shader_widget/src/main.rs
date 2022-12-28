use iced::widget::{column, container, text};
use iced::{
    executor, time, Alignment, Application, Command, Settings, Subscription,
    Theme,
};

use iced_native::{shader, widget, Length};

use std::time::{Duration, Instant};

const SHADER_NAME: &str = "custom_shader.wgsl";

pub fn main() -> iced::Result {
    Example::run(Settings {
        ..Settings::default()
    })
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    Leaving,
    Entering,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Tick(Instant),
    HoveredShader(Direction),
    PressedShader,
}

enum State {
    Idle,
    Ticking { last_tick: Instant },
}

struct Example {
    duration_since_last_click: Duration,
    custom_shader_quad: widget::WgslShaderQuad<Message>,
    state: State,
}

impl Example {
    fn get_path() -> std::path::PathBuf {
        let shader_name: std::path::PathBuf = SHADER_NAME.into();

        let relative_shader_path = std::path::Path::new("examples")
            .join("custom_shader_quad")
            .join("src")
            .join(shader_name);

        let absolute_shader_path =
            std::env::current_dir().unwrap().join(relative_shader_path);

        return absolute_shader_path;
    }
}

impl Application for Example {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        let mut shader_quad = widget::WgslShaderQuad::new(
            shader::ShaderContent::Path(Example::get_path()).into(),
            // handle: ShaderContent::Memory(SHADER).into(), // can also use a &str as shader code
            200.0,
            200.0,
        );
        shader_quad = shader_quad.on_press(Message::PressedShader);
        shader_quad = shader_quad
            .on_hover_entering(Message::HoveredShader(Direction::Entering));
        shader_quad = shader_quad
            .on_hover_leaving(Message::HoveredShader(Direction::Leaving));
        (
            Self {
                duration_since_last_click: Duration::default(),
                custom_shader_quad: shader_quad,
                state: State::Idle,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Custom shader - Iced")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            //
            // similar to the stopwatch example
            //
            Message::Tick(now) => {
                if let State::Ticking { last_tick } = &mut self.state {
                    self.duration_since_last_click += now - *last_tick;
                    *last_tick = now;
                    self.custom_shader_quad
                        .set_time(self.duration_since_last_click);
                }
            }

            Message::HoveredShader(dir) => match dir {
                Direction::Entering => {
                    self.state = State::Ticking {
                        last_tick: Instant::now(),
                    };
                }
                Direction::Leaving => {
                    self.state = State::Idle;
                }
            },
            Message::PressedShader => {}
        }

        Command::none()
    }

    fn view(&self) -> iced::Element<Message> {
        let instruction =
            column![text("Interact using mouse presses")].spacing(10);

        let content = column![instruction, self.custom_shader_quad.clone(),]
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
        match self.state {
            State::Idle => Subscription::none(),
            State::Ticking { .. } => {
                time::every(Duration::from_millis(10)).map(Message::Tick)
            }
        }
    }
}
