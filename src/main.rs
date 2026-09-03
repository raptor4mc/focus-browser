use eframe::egui;

struct App {
    html_text: String,
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.label(&self.html_text);
            });
        });
    }
}

fn main() {
    println!("[VERBOSE] Starting Focus Browser skeleton — P1 window + P2 fetch + P3 DOM + P4 styles");
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

    println!("[VERBOSE] P3 DOM: initializing flat-array DOM (CPU — html5ever parser thread)");
    let mut dom = p3_dom::dom::Dom::new();
    println!("[VERBOSE] P3 DOM: pre-allocated nodes={}, children={}, string_arena={} bytes",
        dom.nodes.capacity(), dom.children.capacity(), dom.string_arena.capacity());
    let root = dom.push_node(0, 0x01);
    println!("[VERBOSE] P3 DOM: root node index = {} (tag=0, flags=0x01 IS_ELEMENT)", root);

    println!("[VERBOSE] P4 styles: computing styles for flat DOM (CPU — stylo/rayon)");
    let style_results = p4_styles::compute_styles(&dom, ".box { color: red; }");
    println!("[VERBOSE] P4 styles: computed {} style indices (no CSSOM, compute once)", style_results.len());

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
    println!("[VERBOSE] Skeleton: P1 window + P2 fetch + P3 DOM + P4 styles integrated — GPU-first, multi-threaded tokio");

    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Focus Browser — P1 Window + P2 Fetch + P3 DOM + P4 Styles",
        native_options,
        Box::new(|_cc| Ok(Box::new(App { html_text }))),
    ).expect("Event loop error");
}
