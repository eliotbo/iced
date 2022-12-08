/// A colored rectangle with a border.
///
/// This type can be directly uploaded to GPU memory.
#[derive(Debug, Clone)]
#[repr(C)]
pub struct CustomShaderQuadWithCode {
    /// The position of the [`Quad`].
    pub position: [f32; 2],

    /// The size of the [`Quad`].
    pub size: [f32; 2],

    /// The color of the [`Quad`], in __linear RGB__.
    pub color: [f32; 4],

    /// The border color of the [`Quad`], in __linear RGB__.
    pub border_color: [f32; 4],

    /// The border radius of the [`Quad`].
    pub border_radius: [f32; 4],

    /// The border width of the [`Quad`].
    pub border_width: f32,

    /// Mouse position on the screen.
    pub mouse_position: [f32; 2],

    /// Mouse click and release: [0.0] = no event, [1.0] = click, [-1.0] = release.
    /// The first element is for left mouse click, the second for right mouse click.
    pub mouse_click: [f32; 2],

    /// time in seconds since the start of the program.
    pub time: f32,

    /// frame number since the start of the program.
    pub frame: u32,

    /// Custom shader code.
    pub shader_code: String,
}

impl From<&CustomShaderQuadWithCode> for CustomShaderQuad {
    fn from(custom_shader_quad: &CustomShaderQuadWithCode) -> Self {
        CustomShaderQuad {
            position: custom_shader_quad.position,
            size: custom_shader_quad.size,
            color: custom_shader_quad.color,
            border_color: custom_shader_quad.border_color,
            border_radius: custom_shader_quad.border_radius,
            mouse_position: custom_shader_quad.mouse_position,
            mouse_click: custom_shader_quad.mouse_click,
            time: custom_shader_quad.time,
            frame: custom_shader_quad.frame,
            border_width: custom_shader_quad.border_width,
        }
    }
}

/// A shadered rectangle with a border.
///
/// This type can be directly uploaded to GPU memory.
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct CustomShaderQuad {
    /// The position of the [`Quad`].
    pub position: [f32; 2],

    /// The size of the [`Quad`].
    pub size: [f32; 2],

    /// The color of the [`Quad`], in __linear RGB__.
    pub color: [f32; 4],

    /// The border color of the [`Quad`], in __linear RGB__.
    pub border_color: [f32; 4],

    /// The border radius of the [`Quad`].
    pub border_radius: [f32; 4],

    /// Mouse position on the screen.
    pub mouse_position: [f32; 2],

    /// Mouse click and release: [0.0] = no event, [1.0] = click, [-1.0] = release.
    /// The first element is for left mouse click, the second for right mouse click.
    pub mouse_click: [f32; 2],

    /// time in seconds since the start of the program.
    pub time: f32,

    /// frame number since the start of the program.
    pub frame: u32,

    /// The border width of the [`Quad`].
    pub border_width: f32,
}

#[allow(unsafe_code)]
unsafe impl bytemuck::Zeroable for CustomShaderQuad {}

#[allow(unsafe_code)]
unsafe impl bytemuck::Pod for CustomShaderQuad {}
