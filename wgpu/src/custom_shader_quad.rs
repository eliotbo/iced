use crate::Transformation;
use iced_graphics::layer;

use iced_native::Rectangle;

use bytemuck::{Pod, Zeroable};
use iced_native::shader::Handle;
use std::collections::HashMap;
use std::mem;
use wgpu::util::DeviceExt;

const BAREBONES_SHADER: &str = include_str!("shader/default_quad_shader.wgsl");

#[derive(Debug)]
pub struct Pipeline {
    layout: wgpu::PipelineLayout,
    format: wgpu::TextureFormat,
    constants: wgpu::BindGroup,
    constants_buffer: wgpu::Buffer,
    vertices: wgpu::Buffer,
    indices: wgpu::Buffer,
    instances_buffer: wgpu::Buffer,

    pipeline: wgpu::RenderPipeline,
    shader_modules_cache: HashMap<u64, wgpu::ShaderModule>,
}

impl Pipeline {
    pub fn new(device: &wgpu::Device, format: wgpu::TextureFormat) -> Pipeline {
        let constant_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("iced_wgpu::custom shader quad uniforms layout"),
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: wgpu::BufferSize::new(
                            mem::size_of::<Uniforms>() as wgpu::BufferAddress,
                        ),
                    },
                    count: None,
                }],
            });

        let constants_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("iced_wgpu::custom shader quad uniforms buffer"),
            size: mem::size_of::<Uniforms>() as wgpu::BufferAddress,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let constants = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("iced_wgpu::custom shader quad uniforms bind group"),
            layout: &constant_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: constants_buffer.as_entire_binding(),
            }],
        });

        let layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("iced_wgpu::custom shader quad pipeline layout"),
                push_constant_ranges: &[],
                bind_group_layouts: &[&constant_layout],
            });

        let vertices =
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("iced_wgpu::custom shader quad vertex buffer"),
                contents: bytemuck::cast_slice(&QUAD_VERTS),
                usage: wgpu::BufferUsages::VERTEX,
            });

        let indices =
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("iced_wgpu::custom shader quad index buffer"),
                contents: bytemuck::cast_slice(&QUAD_INDICES),
                usage: wgpu::BufferUsages::INDEX,
            });

        let instances = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("iced_wgpu::custom shader quad instance buffer"),
            size: mem::size_of::<layer::Quad>() as u64 * MAX_INSTANCES as u64,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let shader =
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("iced_wgpu::custom shader quad::shader"),
                source: wgpu::ShaderSource::Wgsl(std::borrow::Cow::Borrowed(
                    BAREBONES_SHADER,
                )),
            });

        let pipeline =
            device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("iced_wgpu::custom shader quad pipeline"),
                layout: Some(&layout),
                vertex: wgpu::VertexState {
                    module: &shader,
                    entry_point: "vs_main",
                    buffers: &[
                        wgpu::VertexBufferLayout {
                            array_stride: mem::size_of::<Vertex>() as u64,
                            step_mode: wgpu::VertexStepMode::Vertex,
                            attributes: &[wgpu::VertexAttribute {
                                shader_location: 0,
                                format: wgpu::VertexFormat::Float32x2,
                                offset: 0,
                            }],
                        },
                        wgpu::VertexBufferLayout {
                            array_stride: mem::size_of::<layer::Quad>() as u64,
                            step_mode: wgpu::VertexStepMode::Instance,
                            attributes: &wgpu::vertex_attr_array!(
                                1 => Float32x2,
                                2 => Float32x2,
                                3 => Float32x4,
                                4 => Float32x2,
                                5 => Float32x2,
                                6 => Float32,
                                7 => Uint32,
                            ),
                        },
                    ],
                },
                fragment: Some(wgpu::FragmentState {
                    module: &shader,
                    entry_point: "fs_main",
                    targets: &[Some(wgpu::ColorTargetState {
                        format: format,
                        blend: Some(wgpu::BlendState {
                            color: wgpu::BlendComponent {
                                src_factor: wgpu::BlendFactor::SrcAlpha,
                                dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                                operation: wgpu::BlendOperation::Add,
                            },
                            alpha: wgpu::BlendComponent {
                                src_factor: wgpu::BlendFactor::One,
                                dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                                operation: wgpu::BlendOperation::Add,
                            },
                        }),
                        write_mask: wgpu::ColorWrites::ALL,
                    })],
                }),
                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::TriangleList,
                    front_face: wgpu::FrontFace::Cw,
                    ..Default::default()
                },
                depth_stencil: None,
                multisample: wgpu::MultisampleState {
                    count: 1,
                    mask: !0,
                    alpha_to_coverage_enabled: false,
                },
                multiview: None,
            });

        Pipeline {
            pipeline,
            layout,
            format,
            constants,
            constants_buffer,
            vertices,
            indices,
            instances_buffer: instances,
            shader_modules_cache: HashMap::new(),
        }
    }

    pub fn make_pipeline(
        &self,
        device: &wgpu::Device,
        shader_module: &wgpu::ShaderModule,
    ) -> wgpu::RenderPipeline {
        let pipeline =
            device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("iced_wgpu::custom shader quad pipeline"),
                layout: Some(&self.layout),
                vertex: wgpu::VertexState {
                    module: shader_module,
                    entry_point: "vs_main",
                    buffers: &[
                        wgpu::VertexBufferLayout {
                            array_stride: mem::size_of::<Vertex>() as u64,
                            step_mode: wgpu::VertexStepMode::Vertex,
                            attributes: &[wgpu::VertexAttribute {
                                shader_location: 0,
                                format: wgpu::VertexFormat::Float32x2,
                                offset: 0,
                            }],
                        },
                        wgpu::VertexBufferLayout {
                            array_stride: mem::size_of::<layer::Quad>() as u64,
                            step_mode: wgpu::VertexStepMode::Instance,
                            attributes: &wgpu::vertex_attr_array!(
                                1 => Float32x2,
                                2 => Float32x2,
                                3 => Float32x4,
                                4 => Float32x2,
                                5 => Float32x2,
                                6 => Float32,
                                7 => Uint32,
                            ),
                        },
                    ],
                },
                fragment: Some(wgpu::FragmentState {
                    module: shader_module,
                    entry_point: "fs_main",
                    targets: &[Some(wgpu::ColorTargetState {
                        format: self.format,
                        blend: Some(wgpu::BlendState {
                            color: wgpu::BlendComponent {
                                src_factor: wgpu::BlendFactor::SrcAlpha,
                                dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                                operation: wgpu::BlendOperation::Add,
                            },
                            alpha: wgpu::BlendComponent {
                                src_factor: wgpu::BlendFactor::One,
                                dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                                operation: wgpu::BlendOperation::Add,
                            },
                        }),
                        write_mask: wgpu::ColorWrites::ALL,
                    })],
                }),
                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::TriangleList,
                    front_face: wgpu::FrontFace::Cw,
                    ..Default::default()
                },
                depth_stencil: None,
                multisample: wgpu::MultisampleState {
                    count: 1,
                    mask: !0,
                    alpha_to_coverage_enabled: false,
                },
                multiview: None,
            });

        return pipeline;
    }

    pub fn read_shader(&self, handle: &Handle) -> String {
        use std::io::Read;

        let mut bytes = Vec::new();

        let raw_shader = match std::fs::File::open(&handle.path) {
            Ok(mut file) => {
                if let Ok(_) = file.read_to_end(&mut bytes) {
                    if let Ok(shader_code) = String::from_utf8(bytes) {
                        shader_code
                    } else {
                        panic!("Could not convert to string shader with path: {:?}", &handle.path);
                    }
                } else {
                    panic!(
                        "Could not read shader file with path: {:?}",
                        &handle.path
                    );
                }
            }
            Err(_) => panic!(
                "Could not find shader file with path: {:?}",
                &handle.path
            ),
        };

        return raw_shader;
    }

    // method that checks if the shader is in the ShaderCache, if not it adds it creates the ShaderModule and adds it
    // to the ShaderCache. If it is in the ShaderCache it returns the ShaderModule from the ShaderCache
    pub fn insert_shader_module(
        &mut self,
        device: &wgpu::Device,
        shader_handle: &Handle,
    ) -> bool {
        // -> &wgpu::ShaderModule {
        // if let Some(shader_module) = self.shader_modules.get(&shader_handle.id)
        // {
        //     return shader_module;
        // }

        if self.shader_modules_cache.contains_key(&shader_handle.id) {
            return false;
        }

        let shader_code = self.read_shader(&shader_handle);

        let shader_module =
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("iced_wgpu::custom shader quad::shader"),
                source: wgpu::ShaderSource::Wgsl(std::borrow::Cow::Borrowed(
                    &shader_code,
                )),
            });

        let _res = self
            .shader_modules_cache
            .insert(shader_handle.id, shader_module);

        return true;

        // let shader_mod =
        //     self.shader_modules.get(&shader_handle.id).unwrap().clone();

        // return shader_mod;

        // return shader_module;
    }

    pub fn draw(
        &mut self,
        device: &wgpu::Device,
        staging_belt: &mut wgpu::util::StagingBelt,
        encoder: &mut wgpu::CommandEncoder,
        instances: &[layer::CustomShaderQuadWithHandle],
        serializable_instances: &[layer::CustomShaderQuad],
        transformation: Transformation,
        scale: f32,
        bounds: Rectangle<u32>,
        target: &wgpu::TextureView,
    ) {
        let uniforms = Uniforms::new(transformation, scale);

        {
            let mut constants_buffer = staging_belt.write_buffer(
                encoder,
                &self.constants_buffer,
                0,
                wgpu::BufferSize::new(mem::size_of::<Uniforms>() as u64)
                    .unwrap(),
                device,
            );

            constants_buffer.copy_from_slice(bytemuck::bytes_of(&uniforms));
        }

        let mut i = 0;
        let total = instances.len();

        while i < total {
            let shader_handle: Handle = instances[i].shader_handle.clone();

            let has_new_shader_module =
                self.insert_shader_module(device, &shader_handle);

            if has_new_shader_module {
                let shader_module = self
                    .shader_modules_cache
                    .get(&shader_handle.id)
                    .unwrap()
                    .clone();

                let new_pipeline = self.make_pipeline(&device, shader_module);

                self.pipeline = new_pipeline;
            }

            let end = (i + MAX_INSTANCES).min(total);
            let amount = end - i;

            let instance_bytes =
                bytemuck::cast_slice(&serializable_instances[i..end]);

            let mut instance_buffer = staging_belt.write_buffer(
                encoder,
                &self.instances_buffer,
                0,
                wgpu::BufferSize::new(instance_bytes.len() as u64).unwrap(),
                device,
            );

            instance_buffer.copy_from_slice(instance_bytes);

            {
                let mut render_pass =
                    encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                        label: Some(
                            "iced_wgpu::custom shader quad render pass",
                        ),
                        color_attachments: &[Some(
                            wgpu::RenderPassColorAttachment {
                                view: target,
                                resolve_target: None,
                                ops: wgpu::Operations {
                                    load: wgpu::LoadOp::Load,
                                    store: true,
                                },
                            },
                        )],
                        depth_stencil_attachment: None,
                    });

                // self.pipeline = self.make_pipeline(device, shader_path);

                render_pass.set_pipeline(&self.pipeline);
                render_pass.set_bind_group(0, &self.constants, &[]);
                render_pass.set_index_buffer(
                    self.indices.slice(..),
                    wgpu::IndexFormat::Uint16,
                );
                render_pass.set_vertex_buffer(0, self.vertices.slice(..));
                render_pass
                    .set_vertex_buffer(1, self.instances_buffer.slice(..));

                render_pass.set_scissor_rect(
                    bounds.x,
                    bounds.y,
                    bounds.width,
                    bounds.height,
                );

                render_pass.draw_indexed(
                    0..QUAD_INDICES.len() as u32,
                    0,
                    0..amount as u32,
                );
            }

            i += MAX_INSTANCES;
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Zeroable, Pod)]
pub struct Vertex {
    _position: [f32; 2],
}

const QUAD_INDICES: [u16; 6] = [0, 1, 2, 0, 2, 3];

const QUAD_VERTS: [Vertex; 4] = [
    Vertex {
        _position: [0.0, 0.0],
    },
    Vertex {
        _position: [1.0, 0.0],
    },
    Vertex {
        _position: [1.0, 1.0],
    },
    Vertex {
        _position: [0.0, 1.0],
    },
];

const MAX_INSTANCES: usize = 100_000;

#[repr(C)]
#[derive(Debug, Clone, Copy, Zeroable, Pod)]
struct Uniforms {
    transform: [f32; 16],
    scale: f32,
    _padding: [f32; 3],
}

impl Uniforms {
    fn new(transformation: Transformation, scale: f32) -> Uniforms {
        Self {
            transform: *transformation.as_ref(),
            scale,
            _padding: [0.0; 3],
        }
    }
}

impl Default for Uniforms {
    fn default() -> Self {
        Self {
            transform: *Transformation::identity().as_ref(),
            scale: 1.0,
            _padding: [0.0; 3],
        }
    }
}
