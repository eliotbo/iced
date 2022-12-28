/// A colored rectangle with a border.
///
/// This type can be directly uploaded to GPU memory.
#[derive(Debug, Clone)]
#[repr(C)]
pub struct CustomShaderQuadWithHandle {
    /// The position of the [`Quad`].
    pub position: [f32; 2],

    /// The size of the [`Quad`].
    pub size: [f32; 2],

    /// The color of the [`Quad`], in __linear RGB__.
    pub color: [f32; 4],

    /// Mouse position on the screen.
    pub mouse_position: [f32; 2],

    /// Shader mouse click encoding logic:
    /// 0 if no click and no hover.
    /// 1 if left click over.
    /// 2 if right click over.
    /// 3 if hover.
    pub mouse_click: u32,

    /// time in seconds since the start of the program.
    pub time: f32,

    /// frame number since the start of the program.
    pub frame: u32,

    /// Custom shader code.
    pub handle: iced_native::shader::Handle,
}

impl From<&CustomShaderQuadWithHandle> for CustomShaderQuad {
    fn from(custom_shader_quad: &CustomShaderQuadWithHandle) -> Self {
        CustomShaderQuad {
            position: custom_shader_quad.position,
            size: custom_shader_quad.size,
            color: custom_shader_quad.color,
            mouse_position: custom_shader_quad.mouse_position,
            mouse_click: custom_shader_quad.mouse_click,
            time: custom_shader_quad.time,
            frame: custom_shader_quad.frame,
            dummy: 0,
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

    /// Mouse position on the screen.
    pub mouse_position: [f32; 2],

    /// Shader mouse click encoding logic:
    /// 0 if no click and no hover.
    /// 1 if left click over.
    /// 2 if right click over.
    /// 3 if hover.
    pub mouse_click: u32,

    /// time in seconds since the start of the program.
    pub time: f32,

    /// frame number since the start of the program.
    pub frame: u32,

    /// dummy variable
    pub dummy: u32,
}

#[allow(unsafe_code)]
unsafe impl bytemuck::Zeroable for CustomShaderQuad {}

#[allow(unsafe_code)]
unsafe impl bytemuck::Pod for CustomShaderQuad {}
