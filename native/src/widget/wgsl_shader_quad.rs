//! A widget where a WGSL shader is rendered on a quad.

use crate::event::{self, Event};
use crate::layout;
use crate::mouse;
use crate::renderer;
use crate::widget::tree::{self, Tree};
use crate::{shader, Color, Point, Rectangle, Size};
use crate::{Clipboard, Element, Layout, Length, Padding, Shell, Widget};

use std::time::Duration;

#[derive(Clone)]
/// A widget where a WGSL shader is rendered on a quad.
#[allow(missing_debug_implementations)]
pub struct WgslShaderQuad<Message> {
    on_press: Option<Message>,
    on_hover_entering: Option<Message>,
    on_hover_leaving: Option<Message>,
    width: f32,
    height: f32,
    padding: Padding,
    time: Duration,
    handle: shader::Handle,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ShaderMouseState {
    None,
    LeftPressed,
    RightPressed,
}

impl ShaderMouseState {
    fn encode(&self, hovered: bool) -> u32 {
        let mut left_mouse_click = 0;
        let mut right_mouse_click = 0;
        match self {
            ShaderMouseState::None => {}
            ShaderMouseState::LeftPressed => left_mouse_click = 1,
            ShaderMouseState::RightPressed => right_mouse_click = 1,
        };

        let mouse_hover: u32 = hovered.into();
        (mouse_hover << 2) | right_mouse_click << 1 | left_mouse_click
    }
}

impl Default for ShaderMouseState {
    fn default() -> Self {
        ShaderMouseState::None
    }
}

impl<Message> WgslShaderQuad<Message> {
    /// Creates a new [`WgslShaderQuad`] with the given content.
    pub fn new(handle: shader::Handle, width: f32, height: f32) -> Self {
        WgslShaderQuad {
            on_press: None,
            on_hover_entering: None,
            on_hover_leaving: None,
            width,
            height,
            padding: Padding::new(5),

            time: Duration::from_secs(0),
            handle,
        }
    }

    /// Sets the width of the [`WgslShaderQuad`].
    pub fn width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }

    /// Sets the height of the [`WgslShaderQuad`].
    pub fn height(mut self, height: f32) -> Self {
        self.height = height;
        self
    }

    /// Sets the [`Padding`] of the [`WgslShaderQuad`].
    pub fn padding<P: Into<Padding>>(mut self, padding: P) -> Self {
        self.padding = padding.into();
        self
    }

    /// Sets the [`Handle`] of the [`WgslShaderQuad`].
    pub fn handle<P: Into<shader::Handle>>(mut self, handle: P) -> Self {
        self.handle = handle.into();
        self
    }

    /// Sets the time of the [`WgslShaderQuad`] animation.
    pub fn set_time(&mut self, time: Duration) {
        self.time = time;
    }

    /// Sets the message that will be produced when the [`WgslShaderQuad`] is pressed.
    ///
    /// Unless `on_press` is called, the [`WgslShaderQuad`] will be disabled.
    pub fn on_press(mut self, msg: Message) -> Self {
        self.on_press = Some(msg);
        self
    }

    /// Sets the message that will be produced when the [`WgslShaderQuad`] is leaving the hovered state.
    pub fn on_hover_leaving(mut self, msg: Message) -> Self {
        self.on_hover_leaving = Some(msg);
        self
    }

    /// Sets the message that will be produced when the [`WgslShaderQuad`] is entering the hovered state.
    pub fn on_hover_entering(mut self, msg: Message) -> Self {
        self.on_hover_entering = Some(msg);
        self
    }
}

/// The local state of a [`Button`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct State {
    mouse: ShaderMouseState,
    hover: bool,
}

impl State {
    /// Creates a new [`State`].
    pub fn new() -> State {
        State::default()
    }
}

impl<'a, Message, Renderer> Widget<Message, Renderer>
    for WgslShaderQuad<Message>
where
    Message: 'a + Clone,
    Renderer: 'a + crate::Renderer,
{
    fn width(&self) -> Length {
        Length::Shrink
    }

    fn height(&self) -> Length {
        Length::Shrink
    }

    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(State::new())
    }

    fn layout(
        &self,
        _renderer: &Renderer,
        _limits: &layout::Limits,
    ) -> layout::Node {
        layout::Node::new(Size::new(self.width, self.height))
    }

    fn mouse_interaction(
        &self,
        _tree: &Tree,
        layout: layout::Layout<'_>,
        cursor_position: Point,
        _viewport: &Rectangle,
        _renderer: &Renderer,
    ) -> mouse::Interaction {
        let is_mouse_over = layout.bounds().contains(cursor_position);

        if is_mouse_over && self.on_press.is_some() {
            mouse::Interaction::Pointer
        } else {
            mouse::Interaction::default()
        }
    }

    fn on_event(
        &mut self,
        tree: &mut Tree,
        event: Event,
        layout: layout::Layout<'_>,
        cursor_position: Point,
        _renderer: &Renderer,
        _clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
    ) -> event::Status {
        let state = tree.state.downcast_mut::<State>();

        match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
                if self.on_press.is_some() {
                    let bounds = layout.bounds();

                    if bounds.contains(cursor_position) {
                        state.mouse = ShaderMouseState::LeftPressed;
                        return event::Status::Captured;
                    }
                }
            }

            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Right)) => {
                let bounds = layout.bounds();

                if bounds.contains(cursor_position) {
                    state.mouse = ShaderMouseState::RightPressed;
                    return event::Status::Captured;
                }
            }

            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)) => {
                let bounds = layout.bounds();
                if let Some(on_press) = self.on_press.clone() {
                    if state.mouse == ShaderMouseState::LeftPressed {
                        if bounds.contains(cursor_position) {
                            state.mouse = ShaderMouseState::None;
                            shell.publish(on_press);
                        }
                        return event::Status::Captured;
                    }
                }
            }

            Event::Mouse(mouse::Event::ButtonReleased(
                mouse::Button::Right,
            )) => {
                if state.mouse == ShaderMouseState::RightPressed {
                    state.mouse = ShaderMouseState::None;
                    return event::Status::Captured;
                }
            }

            Event::Mouse(mouse::Event::CursorMoved { .. }) => {
                let bounds = layout.bounds();
                let mouse_in_bounds = bounds.contains(cursor_position);

                if let Some(on_hover_entering) = &self.on_hover_entering {
                    if !state.hover && mouse_in_bounds {
                        state.hover = mouse_in_bounds;
                        shell.publish(on_hover_entering.clone());
                        return event::Status::Captured;
                    }
                }

                if let Some(on_hover_leaving) = &self.on_hover_leaving {
                    if state.hover && !mouse_in_bounds {
                        state.hover = mouse_in_bounds;
                        shell.publish(on_hover_leaving.clone());
                        return event::Status::Captured;
                    }
                }
            }
            _ => {}
        }

        event::Status::Ignored
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        _theme: &Renderer::Theme,
        _style: &renderer::Style,
        layout: Layout<'_>,
        cursor_position: Point,
        _viewport: &Rectangle,
    ) {
        let state = tree.state.downcast_ref::<State>();

        renderer.make_custom_shader_quad(
            renderer::CustomShaderQuad {
                bounds: layout.bounds(),
                handle: self.handle.clone(),

                // The following fields have fixed names and types, but users can pass
                // in any data they want the shader to receive as long as the types match.
                mouse_position: cursor_position,
                mouse_click: state.mouse.encode(state.hover),
                time: self.time.as_secs_f32(),
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

impl<'a, Message, Renderer> From<WgslShaderQuad<Message>>
    for Element<'a, Message, Renderer>
where
    Message: Clone + 'a,
    Renderer: crate::Renderer + 'a,
{
    fn from(wgsl_shader_quad: WgslShaderQuad<Message>) -> Self {
        Self::new(wgsl_shader_quad)
    }
}
