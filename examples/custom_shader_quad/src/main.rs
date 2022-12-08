//! This example showcases custom shaders.
mod custom_shader_quad {
    use iced_native::layout::{self, Layout};
    use iced_native::renderer;
    use iced_native::widget::{self, Widget};

    use iced_native::{Color, Element, Length, Point, Rectangle, Size};

    use std::time::Instant;

    pub struct CustomQuad {
        start_time: Instant,
        size: f32,
        radius: [f32; 4],
        border_width: f32,
        mouse_click: Point,
    }

    impl CustomQuad {
        pub fn new(
            size: f32,
            radius: [f32; 4],
            border_width: f32,
            mouse_click: Point,
        ) -> Self {
            Self {
                start_time: Instant::now(),
                size,
                radius,
                border_width,
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
            // println!("{:?}", cursor_position);
            renderer.make_custom_shader_quad(
                renderer::CustomShaderQuad {
                    bounds: layout.bounds(),
                    border_radius: self.radius.into(),
                    border_width: self.border_width,
                    border_color: Color::from_rgb(1.0, 0.0, 0.0),
                    shader_code: include_str!("custom_shader.wgsl").to_string(),
                    mouse_position: cursor_position,
                    mouse_click: self.mouse_click,
                    time: self.start_time.elapsed().as_secs_f32(),
                    frame: 1,
                },
                Color::BLACK,
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

use iced::widget::canvas;
use iced::widget::canvas::{
    Cache, Canvas, Cursor, Frame, Geometry, Path, Text,
};
use iced::widget::{button, column, container, scrollable, slider, text};
use iced::{executor, mouse, touch, Application, Command, Subscription};

use iced::widget::canvas::event::{self, Event};
use iced::widget::pane_grid::{self, PaneGrid};
use iced::{
    Alignment, Element, Length, Point, Rectangle, Settings, Theme, Vector,
};
use iced_lazy::responsive;

// use iced_graphics::Renderer;
use iced_native::{event as event_native, subscription, Event as EventNative};
// use iced_wgpu::Backend;
use std::{f32::consts::PI, time::Instant};

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

struct Example {
    radius: [f32; 4],
    border_width: f32,
    mouse_click: Point,
    modifier_keys: ModifierKeys,
    uniform: Uniform,
    panes: pane_grid::State<Pane>,
}

#[derive(Debug, Clone, Copy)]
#[allow(clippy::enum_variant_names)]
enum Message {
    RadiusTopLeftChanged(f32),
    RadiusTopRightChanged(f32),
    RadiusBottomRightChanged(f32),
    RadiusBottomLeftChanged(f32),
    BorderWidthChanged(f32),
    LeftMousePress(Point),
    LeftMouseRelease(Point),
    RightMousePress(Point),
    RightMouseRelease(Point),
    RightMouseReleaseGeneral,
    LeftMouseReleaseGeneral,
    Tick,
    ArrowPressed,
    PaneClicked(pane_grid::Pane),
    PaneReleased(pane_grid::Pane),
    None,
}

struct Pane {
    id: usize,
    pub is_pinned: bool,
}

impl Pane {
    fn new(id: usize) -> Self {
        Self {
            id,
            is_pinned: false,
        }
    }
}

pub fn pane_focused(theme: &Theme) -> container::Appearance {
    let palette = theme.extended_palette();

    container::Appearance {
        background: Some(palette.background.weak.color.into()),
        border_width: 2.0,
        border_color: palette.primary.strong.color,
        ..Default::default()
    }
}

impl Application for Example {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        let (panes, _) = pane_grid::State::new(Pane::new(0));
        (
            Self {
                radius: [50.0; 4],
                border_width: 0.0,
                mouse_click: Point::new(0.0, 0.0),
                modifier_keys: ModifierKeys::default(),
                uniform: Uniform {
                    mouse_click: Vector::new(0.0, 0.0),
                },
                panes,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Custom shader - Iced")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        let [tl, tr, br, bl] = self.radius;

        match message {
            Message::RadiusTopLeftChanged(radius) => {
                self.radius = [radius, tr, br, bl];
            }

            Message::RadiusTopRightChanged(radius) => {
                self.radius = [tl, radius, br, bl];
            }

            Message::RadiusBottomRightChanged(radius) => {
                self.radius = [tl, tr, radius, bl];
            }

            Message::RadiusBottomLeftChanged(radius) => {
                self.radius = [tl, tr, br, radius];
            }

            Message::BorderWidthChanged(width) => {
                self.border_width = width;
            }

            Message::LeftMousePress(_point) => {
                println!("Left mouse press");
            }

            Message::RightMousePress(_point) => {
                self.mouse_click.y = 1.0;
                println!("Right mouse press");
            }

            Message::LeftMouseRelease(_point) => {
                self.mouse_click.x = 0.0;
                // return Command();
            }

            Message::RightMouseRelease(_point) => {
                self.mouse_click.y = 0.0;
            }

            Message::ArrowPressed => {
                println!("Arrow pressed");
            }

            Message::PaneClicked(_pane) => {
                self.mouse_click.x = 1.0;
                println!("Pane clicked");
            }
            Message::RightMouseReleaseGeneral => {
                self.mouse_click.y = 0.0;
                println!("general y");
            }

            Message::LeftMouseReleaseGeneral => {
                self.mouse_click.x = 0.0;
                println!("general x ");
            }

            _ => (),
        }

        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let [tl, tr, br, bl] = self.radius;

        let pane_grid = PaneGrid::new(&self.panes, |id, pane, is_maximized| {
            pane_grid::Content::new(responsive(move |size| {
                // view_content(id, total_panes, pane.is_pinned, size)

                // let pin_button = button(
                //     text(if pane.is_pinned { "Unpin" } else { "Pin" }).size(14),
                // )
                // .on_press(Message::ArrowPressed)
                // .padding(3);

                let content = column![
                    Canvas::new(&self.uniform)
                        .width(Length::Fill)
                        .height(Length::Fill),
                    custom_shader_quad::CustomQuad::new(
                        200.0,
                        self.radius,
                        self.border_width,
                        self.mouse_click
                    ),
                    text(format!("Radius: {tl:.2}/{tr:.2}/{br:.2}/{bl:.2}")),
                    slider(1.0..=100.0, tl, Message::RadiusTopLeftChanged)
                        .step(0.01),
                    slider(1.0..=100.0, tr, Message::RadiusTopRightChanged)
                        .step(0.01),
                    slider(1.0..=100.0, br, Message::RadiusBottomRightChanged)
                        .step(0.01),
                    slider(1.0..=100.0, bl, Message::RadiusBottomLeftChanged)
                        .step(0.01),
                    slider(
                        1.0..=10.0,
                        self.border_width,
                        Message::BorderWidthChanged
                    )
                    .step(0.01),
                ]
                .width(Length::Fill)
                .spacing(20)
                .align_items(Alignment::Center);
                // .padding(20)
                // .spacing(20)
                // .max_width(500)
                // .align_items(Alignment::Center);

                // container(scrollable(content))
                container(content)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .padding(5)
                    .center_y()
                    .into()
            }))
            // .title_bar(title_bar)
            // .style(pane_focused)
        })
        .on_click(Message::PaneClicked)
        .width(Length::Fill)
        .height(Length::Fill)
        .spacing(10);
        // .on_click(Message::PaneClicked);

        // .on_drag(Message::Dragged);
        // .on_resize(10, Message::Resized);

        container(pane_grid)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(10)
            // .center_x()
            // .center_y()
            .into()
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }

    // fn subscription(&self) -> Subscription<Message> {
    //     iced::time::every(std::time::Duration::from_millis(10))
    //         .map(|_| Message::Tick)
    // }

    fn subscription(&self) -> Subscription<Message> {
        subscription::events_with(|event, status| {
            // if let event::Status::Captured = status {
            //     return None;
            // }

            // if
            match event {
                iced_native::Event::Keyboard(
                    iced::keyboard::Event::KeyPressed {
                        modifiers,
                        key_code,
                    },
                    // only sends a message when an arrow is pressed while a modifier key is pressed as well
                ) if modifiers.command()
                    | modifiers.shift()
                    | modifiers.alt() =>
                {
                    handle_hotkey(key_code)
                }

                iced_native::Event::Mouse(mouse_event) => match mouse_event {
                    mouse::Event::ButtonReleased(button) => {
                        let message = match button {
                            mouse::Button::Left => {
                                Message::LeftMouseReleaseGeneral
                            }
                            mouse::Button::Right => {
                                Message::RightMouseReleaseGeneral
                            }
                            _ => return None,
                        };
                        Some(message)
                    }
                    _ => None,
                },
                _ => None,
            }
        })
    }
}

fn handle_hotkey(key_code: iced::keyboard::KeyCode) -> Option<Message> {
    use iced::keyboard::KeyCode;

    match key_code {
        KeyCode::Left | KeyCode::Right | KeyCode::Up | KeyCode::Down => {
            Some(Message::ArrowPressed)
        }
        _ => None,
    }
}

struct Uniform {
    mouse_click: Vector,
}

impl canvas::Program<Message> for Uniform {
    type State = ();

    fn update(
        &self,
        // interaction: &mut Interaction,
        _state: &mut (),
        event: Event,
        bounds: Rectangle,
        cursor: Cursor,
    ) -> (event::Status, Option<Message>) {
        // if let Event::Mouse(mouse::Event::ButtonReleased(_)) = event {
        //     *interaction = Interaction::None;
        // }

        let cursor_position =
            if let Some(position) = cursor.position_in(&bounds) {
                position
            } else {
                return (event::Status::Ignored, None);
            };

        match event {
            Event::Touch(touch::Event::FingerMoved { .. }) => {
                (event::Status::Ignored, None)
            }

            Event::Mouse(mouse_event) => match mouse_event {
                mouse::Event::ButtonPressed(button) => {
                    let message = match button {
                        mouse::Button::Left => {
                            Some(Message::LeftMousePress(cursor_position))
                        }
                        mouse::Button::Right => {
                            Some(Message::RightMousePress(cursor_position))
                        }
                        _ => None,
                    };
                    (event::Status::Captured, message)
                }
                mouse::Event::ButtonReleased(button) => {
                    let message = match button {
                        mouse::Button::Left => {
                            Some(Message::LeftMouseRelease(cursor_position))
                        }
                        mouse::Button::Right => {
                            Some(Message::RightMouseRelease(cursor_position))
                        }
                        _ => None,
                    };
                    (event::Status::Captured, message)
                }
                mouse::Event::CursorMoved { .. } => {
                    (event::Status::Ignored, None)
                }
                mouse::Event::WheelScrolled { delta } => match delta {
                    mouse::ScrollDelta::Lines { .. }
                    | mouse::ScrollDelta::Pixels { .. } => {
                        (event::Status::Ignored, None)
                    }
                },
                _ => (event::Status::Ignored, None),
            },
            _ => (event::Status::Ignored, None),
        }
    }

    fn draw(
        &self,
        // _interaction: &Interaction,
        _state: &(),
        _theme: &Theme,
        bounds: Rectangle,
        cursor: Cursor,
    ) -> Vec<Geometry> {
        vec![]
    }

    fn mouse_interaction(
        &self,
        // interaction: &Interaction,
        _state: &(),
        bounds: Rectangle,
        cursor: Cursor,
    ) -> mouse::Interaction {
        mouse::Interaction::default()
    }
}
