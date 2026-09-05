use wgpu::util::DeviceExt;
use p5_layout::LayoutBox;

pub const SHADER: &str = r#"
struct LayoutBox {
    x: f32, y: f32, w: f32, h: f32,
    style_index: u32,
    text_offset: u32,
    flags: u32,
}

@group(0) @binding(0) var<storage, read> boxes: array<LayoutBox>;

struct FrameData {
    viewport_w: f32, viewport_h: f32,
    scroll_x: f32, scroll_y: f32,
    scale: f32,
}
var<push_constant> frame: FrameData;

struct VertexOutput {
    @builtin(position) pos: vec4<f32>,
    @location(0) color: vec3<f32>,
};

@vertex
fn vs_main(@builtin(instance_index) ii: u32, @builtin(vertex_index) vi: u32) -> VertexOutput {
    let box = boxes[ii];
    let corner = vi % 4u;
    let cx = box.x + box.w * 0.5;
    let cy = box.y + box.h * 0.5;
    let half_w = box.w * 0.5;
    let half_h = box.h * 0.5;
    var px = cx;
    var py = cy;
    if (corner == 0u) { px = cx - half_w; py = cy - half_h; }
    else if (corner == 1u) { px = cx + half_w; py = cy - half_h; }
    else if (corner == 2u) { px = cx - half_w; py = cy + half_h; }
    else { px = cx + half_w; py = cy + half_h; }
    var out: VertexOutput;
    out.pos = vec4<f32>((px / frame.viewport_w) * 2.0 - 1.0, -(py / frame.viewport_h) * 2.0 + 1.0, 0.0, 1.0);
    out.color = vec3<f32>(0.8, 0.2, 0.2);
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color, 1.0);
}
"#;

pub struct GpuRenderer {
    pub pipeline: wgpu::RenderPipeline,
    pub bind_group_layout: wgpu::BindGroupLayout,
    pub storage_buffer: wgpu::Buffer,
    pub indirect_buffer: wgpu::Buffer,
    pub texture: wgpu::Texture,
    pub texture_view: wgpu::TextureView,
}

impl GpuRenderer {
    pub fn new(device: &wgpu::Device, layout_boxes: &[LayoutBox]) -> Self {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("P6 shader"),
            source: wgpu::ShaderSource::Wgsl(SHADER.into()),
        });

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("P6 bind group"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage { read_only: true },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("P6 pipeline"),
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[wgpu::PushConstantRange {
                stages: wgpu::ShaderStages::VERTEX,
                range: 0..20,
            }],
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("P6 pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: wgpu::TextureFormat::Rgba8UnormSrgb,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
        });

        let storage_data = bytemuck::cast_slice(layout_boxes);
        let storage_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("P6 storage"),
            contents: storage_data,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
        });

        let count = layout_boxes.len() as u32;
        let indirect_data: Vec<u32> = vec![
            4,
            count,
            0,
            0,
        ];
        let indirect_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("P6 indirect"),
            contents: bytemuck::cast_slice(&indirect_data),
            usage: wgpu::BufferUsages::INDIRECT,
        });

        let texture_size = wgpu::Extent3d {
            width: 1024,
            height: 768,
            depth_or_array_layers: 1,
        };
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("P6 render texture"),
            size: texture_size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });
        let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        Self {
            pipeline,
            bind_group_layout,
            storage_buffer,
            indirect_buffer,
            texture,
            texture_view,
        }
    }

    pub fn render(&self, device: &wgpu::Device, _queue: &wgpu::Queue, encoder: &mut wgpu::CommandEncoder) {
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("P6 bind"),
            layout: &self.bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: self.storage_buffer.as_entire_binding(),
            }],
        });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("P6 render pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &self.texture_view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            render_pass.set_pipeline(&self.pipeline);
            render_pass.set_bind_group(0, &bind_group, &[]);
            render_pass.set_push_constants(wgpu::ShaderStages::VERTEX, 0, bytemuck::cast_slice(&[1024.0f32, 768.0f32, 0.0f32, 0.0f32, 1.0f32]));
            render_pass.draw_indirect(&self.indirect_buffer, 0);
        }
    }
}
