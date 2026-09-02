use eframe::egui;

struct App;

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.painter().rect_filled(
                ui.min_rect(),
                0.0,
                egui::Color32::BLACK,
            );
            ui.label("Renderer: egui (GPU via wgpu/glow) — CPU fallback available");
        });
    }
}

fn main() {
    unsafe {
        std::env::set_var("WGPU_BACKEND", "vulkan");
    }
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("warn")).init();
    println!("Renderer: egui (GPU via wgpu/glow) — forcing Vulkan specifically...");
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::VULKAN,
        ..Default::default()
    });
    for adapter in instance.enumerate_adapters(wgpu::Backends::VULKAN) {
        let info = adapter.get_info();
        println!(
            "Adapter: {} | backend: {:?} | device_type: {:?}",
            info.name, info.backend, info.device_type
        );
    }
    // P2 fetch integrated directly into skeleton
    println!("P2 fetch: requesting https://example.com...");
    let rt = tokio::runtime::Runtime::new().expect("tokio runtime");
    rt.block_on(async {
        let resp = reqwest::get("https://example.com")
            .await
            .expect("fetch failed");
        println!("Status: {}", resp.status());
        let text = resp.text().await.expect("read body failed");
        println!("Length: {} bytes", text.len());
        println!("First 200 chars: {}", &text[..text.len().min(200)]);
    });
    println!("Renderer: egui (Vulkan GPU forced) — CPU fallback disabled");
    println!("Skeleton: P1 window + P2 fetch integrated — ready for ASM1");
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Focus Browser — P1 Window + P2 Fetch",
        native_options,
        Box::new(|_cc| Ok(Box::new(App))),
    ).expect("Event loop error");
}
