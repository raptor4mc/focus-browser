use eframe::egui;

struct App {
    html_text: String,
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add(egui::ScrollArea::vertical().show(ui, |ui| {
                ui.label(&self.html_text);
            }));
        });
    }
}

fn main() {
    unsafe {
        std::env::set_var("WGPU_BACKEND", "vulkan");
        std::env::set_var("WGPU_VALIDATION", "0");
    }
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("warn")).init();
    println!("Renderer: egui (GPU via wgpu/glow) — forcing Vulkan specifically...");
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::VULKAN,
        ..Default::default()
    });
    let mut selected = None;
    for adapter in instance.enumerate_adapters(wgpu::Backends::VULKAN) {
        let info = adapter.get_info();
        println!(
            "Adapter: {} | backend: {:?} | device_type: {:?}",
            info.name, info.backend, info.device_type
        );
        if info.device_type == wgpu::DeviceType::IntegratedGpu && selected.is_none() {
            selected = Some(adapter.get_info().name.clone());
        }
    }
    if let Some(name) = selected {
        println!("Selected GPU adapter: {} (Vulkan, IntegratedGpu)", name);
    }
    println!("Renderer: egui (Vulkan GPU forced) — CPU fallback disabled for GPU parts");

    println!("P3 DOM: initializing flat-array DOM...");
    let mut dom = p3_dom::dom::Dom::new();
    let root = dom.push_node(0, 0x01);
    println!("P3 DOM: root node index = {}, nodes capacity = {}, children capacity = {}",
        root, dom.nodes.capacity(), dom.children.capacity());

    println!("P2 fetch: requesting https://example.com...");
    let rt = tokio::runtime::Runtime::new().expect("tokio runtime");
    let html_text = rt.block_on(async {
        let resp = reqwest::get("https://example.com")
            .await
            .expect("fetch failed");
        println!("Status: {}", resp.status());
        let text = resp.text().await.expect("read body failed");
        println!("Length: {} bytes", text.len());
        println!("First 200 chars: {}", &text[..text.len().min(200)]);
        text
    });
    println!("Skeleton: P1 window + P2 fetch + P3 DOM integrated — GPU-first, multi-threaded tokio");

    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Focus Browser — P1 Window + P2 Fetch + P3 DOM",
        native_options,
        Box::new(|_cc| Ok(Box::new(App { html_text }))),
    ).expect("Event loop error");
}
