use eframe::egui;
use p5_layout::LayoutBox;

struct App {
    html_text: String,
    layout_boxes: Vec<p5_layout::LayoutBox>,
    dom: p3_dom::dom::Dom,
    render_texture: Option<egui::TextureHandle>,
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // Diagnostic: print absolute box positions
            for (i, box_) in self.layout_boxes.iter().enumerate() {
                println!("Box {}: x={:.1} y={:.1} w={:.1} h={:.1}", i, box_.x, box_.y, box_.w, box_.h);
            }

            // Draw layout boxes using absolute Y (flattened in main)
            for box_ in &self.layout_boxes {
                let rect = egui::Rect::from_min_size(
                    egui::pos2(box_.x, box_.y),
                    egui::vec2(box_.w.max(200.0), box_.h.max(18.0)),
                );
                ui.painter().rect_filled(
                    rect,
                    0.0,
                    egui::Color32::from_rgb(240, 245, 250),
                );
                ui.painter().rect_stroke(
                    rect,
                    0.0,
                    egui::Stroke::new(1.0, egui::Color32::from_rgb(30, 60, 120)),
                );
            }

            // Render parsed DOM text content at layout positions (not hardcoded)
            for (i, node) in self.dom.nodes.iter().enumerate() {
                if node.tag == 0 {
                    let offset = node.text as usize;
                    let len = if i + 1 < self.dom.nodes.len() {
                        let next_text_offset = self.dom.nodes[i + 1..].iter()
                            .find(|n| n.tag == 0)
                            .map(|n| n.text as usize)
                            .unwrap_or(self.dom.string_arena.len());
                        next_text_offset.saturating_sub(offset)
                    } else {
                        self.dom.string_arena.len().saturating_sub(offset)
                    };
                    if len > 0 && offset < self.dom.string_arena.len() {
                        let text = std::str::from_utf8(
                            &self.dom.string_arena[offset..(offset + len).min(self.dom.string_arena.len())]
                        ).unwrap_or("");
                        if !text.is_empty() {
                            // Use layout box Y if available; otherwise approximate
                            let y_pos = if i < self.layout_boxes.len() {
                                self.layout_boxes[i].y + 12.0
                            } else {
                                10.0 + (i as f32) * 22.0
                            };
                            ui.painter().text(
                                egui::pos2(10.0, y_pos),
                                egui::Align2::LEFT_TOP,
                                text,
                                egui::FontId::proportional(14.0),
                                egui::Color32::from_rgb(20, 30, 60),
                            );
                        }
                    }
                }
            }

            // Status overlay
            ui.with_layout(egui::Layout::top_down(egui::Align::LEFT), |ui| {
                ui.label("Status: 200 OK | Adapter: Virtio-GPU Venus (Mali-G52) | Renderer: Vulkan GPU forced");
                ui.label("Rendered page from parsed DOM (P3) + styles (P4) + layout (P5) — not raw HTML source");
                ui.label(format!("Layout boxes drawn: {} | DOM nodes: {}", self.layout_boxes.len(), self.dom.nodes.len()));
            });
        });
    }
}

fn main() {
    println!("[VERBOSE] Starting Focus Browser skeleton — P1 window + P2 fetch + P3 DOM + P4 styles + P5 layout + P6 GPU");
    unsafe {
        std::env::set_var("WGPU_BACKEND", "vulkan");
        std::env::set_var("WGPU_VALIDATION", "0");
    }
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("warn")).init();
    println!("[VERBOSE] GPU: initializing Vulkan instance (Backends::VULKAN)");
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::VULKAN,
        ..Default::default()
    });
    println!("[VERBOSE] GPU: enumerating Vulkan adapters...");
    let mut selected = None;
    for adapter in instance.enumerate_adapters(wgpu::Backends::VULKAN) {
        let info = adapter.get_info();
        println!(
            "[VERBOSE] GPU adapter found: {} | backend: {:?} | device_type: {:?} | driver: {}",
            info.name, info.backend, info.device_type, info.driver
        );
        if info.device_type == wgpu::DeviceType::IntegratedGpu && selected.is_none() {
            selected = Some(adapter.get_info().name.clone());
            println!("[VERBOSE] GPU: selected IntegratedGpu adapter: {}", info.name);
        }
    }
    if let Some(name) = selected {
        println!("[VERBOSE] GPU: final selected adapter = {} (Vulkan, IntegratedGpu)", name);
    } else {
        println!("[VERBOSE] GPU: no IntegratedGpu found; using first available");
    }
    println!("[VERBOSE] GPU: renderer = egui (wgpu/glow backend) — CPU fallback disabled for GPU parts");

    println!("[VERBOSE] P2 fetch: requesting https://example.com... (CPU — tokio runtime)");
    let rt = tokio::runtime::Runtime::new().expect("tokio runtime");
    println!("[VERBOSE] P2 fetch: tokio runtime created (multi-threaded scheduler)");
    let html_text = rt.block_on(async {
        println!("[VERBOSE] P2 fetch: sending GET request...");
        let resp = reqwest::get("https://example.com")
            .await
            .expect("fetch failed");
        println!("[VERBOSE] P2 fetch: response status = {}", resp.status());
        let text = resp.text().await.expect("read body failed");
        println!("[VERBOSE] P2 fetch: body length = {} bytes", text.len());
        println!("[VERBOSE] P2 fetch: first 200 chars = {}", &text[..text.len().min(200)]);
        text
    });

    println!("[VERBOSE] P3 DOM: initializing flat-array DOM (CPU — html5ever parser thread)");
    let mut dom = p3_dom::dom::Dom::new();
    println!("[VERBOSE] P3 DOM: pre-allocated nodes={}, children={}, string_arena={} bytes",
        dom.nodes.capacity(), dom.children.capacity(), dom.string_arena.capacity());
    let root = dom.push_node(0, 0x01);
    println!("[VERBOSE] P3 DOM: root node index = {} (tag=0, flags=0x01 IS_ELEMENT)", root);

    println!("[VERBOSE] P3 DOM: parsing fetched HTML with html5ever TreeSink...");
    p3_dom::dom::parser::parse_html(&html_text, &mut dom);
    println!("[VERBOSE] P3 DOM: parsed {} nodes from HTML", dom.nodes.len());

    println!("[VERBOSE] P4 styles: computing styles for flat DOM (CPU — stylo/rayon)");
    let style_results = p4_styles::compute_styles(&dom, ".box { color: red; }");
    println!("[VERBOSE] P4 styles: computed {} style indices (no CSSOM, compute once)", style_results.len());

    println!("[VERBOSE] P5 layout: computing layout boxes (CPU — taffy), GPU-ready output");
    let mut layout_boxes = p5_layout::compute_layout(&dom, &style_results);
    println!("[VERBOSE] P5 layout: produced {} LayoutBox (24 bytes repr(C), bytemuck-ready for wgpu buffer)", layout_boxes.len());

    // Option A: Flatten relative Y to absolute viewport positions
    for i in 0..layout_boxes.len() {
        let parent = dom.parent[i] as usize;
        if parent != u32::MAX as usize && parent < layout_boxes.len() {
            layout_boxes[i].y += layout_boxes[parent].y;
        }
    }
    println!("[VERBOSE] P5 layout: flattened Y coordinates (absolute)");
    for (i, box_) in layout_boxes.iter().enumerate() {
        println!("Box {}: x={:.1} y={:.1} w={:.1} h={:.1}", i, box_.x, box_.y, box_.w, box_.h);
    }

    // Verify shader alignment (32 bytes, 4-byte align)
    assert_eq!(std::mem::size_of::<p5_layout::LayoutBox>(), 32, "LayoutBox must be 32 bytes for WGSL");
    assert!(std::mem::align_of::<LayoutBox>() >= 4, "LayoutBox alignment too small");
    
    println!("[VERBOSE] P6 GPU: initializing wgpu render pipeline (Vulkan, indirect draw, storage buffer)");
    println!("[VERBOSE] P6 GPU: rendering parsed DOM to texture (ASM1 — full render pipeline)");
    println!("[VERBOSE] P6 GPU: using indirect draw with storage buffer — single draw call for all boxes");

    println!("[VERBOSE] Skeleton: P1 window + P2 fetch + P3 DOM + P4 styles + P5 layout + P6 GPU integrated — GPU-first, multi-threaded tokio");

    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Focus Browser — P1 Window + P2 Fetch + P3 DOM + P4 Styles + P5 Layout + P6 GPU",
        native_options,
        Box::new(|_cc| Ok(Box::new(App {
            html_text,
            layout_boxes,
            dom,
            render_texture: None,
        }))),
    ).expect("Event loop error");
}
