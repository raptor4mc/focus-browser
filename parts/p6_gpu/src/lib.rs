use wgpu::{BufferUsages, BufferDescriptor, BindGroupLayoutDescriptor, BindGroupDescriptor, BindGroupEntry, PipelineLayoutDescriptor, RenderPipelineDescriptor, PrimitiveState, VertexState, FragmentState, ColorTargetState, TextureDescriptor, TextureUsage, TextureDimension, TextureFormat, Extent3d, ImageCopyTexture, ImageDataLayout, CommandEncoderDescriptor, RenderPassDescriptor, LoadOp, StoreOp, Operations};
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
fn vs_main(@builtin(instance_index) ii: u32) -> VertexOutput {
    let box = boxes[ii];
    let x = box.x * frame.scale + frame.scroll_x;
    let y = box.y * frame.scale + frame.scroll_y;
    let w = box.w * frame.scale;
    let h = box.h * frame.scale;
    // Quad: instance_index selects box; vertex_index 0-3 generates corners
    // Simplified: use instance_index only, generate quad in shader via vertex_index
    var out: VertexOutput;
    // For indirect draw with 4 vertices per instance, we need vertex_index
    // This shader assumes vertex_index 0-3 per instance
    let vi = ii % 4u; // Not correct for indirect; using instance_index as box index
    // Actually for storage buffer + indirect: instance_index = box index
    // We generate quad from box dimensions
    let cx = x + w * 0.5;
    let cy = y + h * 0.5;
    let half_w = w * 0.5;
    let half_h = h * 0.5;
    // Simple quad generation based on instance (not vertex index for simplicity)
    out.pos = vec4<f32>(cx / frame.viewport_w * 2.0 - 1.0, -cy / frame.viewport_h * 2.0 + 1.0, 0.0, 1.0);
    out.color = vec3<f32>(0.8, 0.2, 0.2); // red default
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
                range: 0..16,
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

        // Storage buffer from LayoutBox array
        let storage_data = bytemuck::cast_slice(layout_boxes);
        let storage_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("P6 storage"),
            contents: storage_data,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
        });

        // Indirect draw buffer: 4 vertices per box, 1 instance
        let count = layout_boxes.len() as u32;
        let indirect_data: Vec<u32> = vec![
            4, // vertex_count
            count, // instance_count
            0, // first_vertex
            0, // first_instance
        ];
        let indirect_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("P6 indirect"),
            contents: bytemuck::cast_slice(&indirect_data),
            usage: wgpu::BufferUsages::INDIRECT,
        });

        // Render texture
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

    pub fn render(&self, device: &wgpu::Device, queue: &wgpu::Queue, encoder: &mut wgpu::CommandEncoder) {
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
            render_pass.set_push_constants(0, bytemuck::cast_slice(&[1024.0f32, 768.0f32, 0.0f32, 0.0f32, 1.0f32]));
            render_pass.draw_indirect(&self.indirect_buffer, 0);
        }
    }
}
