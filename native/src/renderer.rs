//! Write your own renderer.
#[cfg(debug_assertions)]
mod null;
#[cfg(debug_assertions)]
pub use null::Null;

use crate::layout;
use crate::{Background, Color, Element, Point, Rectangle, Vector};

/// A component that can be used by widgets to draw themselves on a screen.
pub trait Renderer: Sized {
    /// The supported theme of the [`Renderer`].
    type Theme;

    /// Lays out the elements of a user interface.
    ///
    /// You should override this if you need to perform any operations before or
    /// after layouting. For instance, trimming the measurements cache.
    fn layout<'a, Message>(
        &mut self,
        element: &Element<'a, Message, Self>,
        limits: &layout::Limits,
    ) -> layout::Node {
        element.as_widget().layout(self, limits)
    }

    /// Draws the primitives recorded in the given closure in a new layer.
    ///
    /// The layer will clip its contents to the provided `bounds`.
    fn with_layer(&mut self, bounds: Rectangle, f: impl FnOnce(&mut Self));

    /// Applies a `translation` to the primitives recorded in the given closure.
    fn with_translation(
        &mut self,
        translation: Vector,
        f: impl FnOnce(&mut Self),
    );

    /// Clears all of the recorded primitives in the [`Renderer`].
    fn clear(&mut self);

    /// Fills a [`Quad`] with the provided [`Background`].
    fn fill_quad(&mut self, quad: Quad, background: impl Into<Background>);

    /// Fills a [`CustomShaderQuad`] with the provided [`Background`].
    fn make_custom_shader_quad(
        &mut self,
        custom_shader_quad: CustomShaderQuad,
        background: impl Into<Background>,
    );
}

/// A polygon with four sides.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Quad {
    /// The bounds of the [`Quad`].
    pub bounds: Rectangle,

    /// The border radius of the [`Quad`].
    pub border_radius: BorderRadius,

    /// The border width of the [`Quad`].
    pub border_width: f32,

    /// The border color of the [`Quad`].
    pub border_color: Color,
}

/// A polygon with four sides.
#[derive(Debug, Clone)]
pub struct CustomShaderQuad {
    /// The bounds of the [`Quad`].
    pub bounds: Rectangle,

    /// Mouse position on the screen.
    pub mouse_position: Point,

    /// The first element is for left mouse click, the second for right mouse click.
    /// 0 if no click and no hover.
    /// 1 if hover.
    /// 2 if left click over.
    /// 3 if right click over.
    pub mouse_click: u32,

    /// time in seconds since the start of the program.
    pub time: f32,

    /// frame number since the start of the program.
    pub frame_number: u32,

    /// Custom shader handle.
    pub handle: crate::shader::Handle,
}

/// The border radi for the corners of a graphics primitive in the order:
/// top-left, top-right, bottom-right, bottom-left.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct BorderRadius([f32; 4]);

impl From<f32> for BorderRadius {
    fn from(w: f32) -> Self {
        Self([w; 4])
    }
}

impl From<[f32; 4]> for BorderRadius {
    fn from(radi: [f32; 4]) -> Self {
        Self(radi)
    }
}

impl From<BorderRadius> for [f32; 4] {
    fn from(radi: BorderRadius) -> Self {
        radi.0
    }
}

/// The styling attributes of a [`Renderer`].
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Style {
    /// The text color
    pub text_color: Color,
}

impl Default for Style {
    fn default() -> Self {
        Style {
            text_color: Color::BLACK,
        }
    }
}
