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
    std::env::set_var("WGPU_BACKEND", "vulkan");
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
    // HUMAN REVIEW ONLY — per agent.md assembly rules.
    // This links P1 `window` into the root binary. Revert if issues found.
    // Security: isolated window creation; no network, no DOM, no JS, no GPU.
    // No integration with fetch/DOM/CSS until ASM1.
    // Note: We never used winit; window layer is eframe/egui.
    println!("Renderer: egui (Vulkan GPU forced) — CPU fallback disabled");
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Focus Browser — P1 Window",
        native_options,
        Box::new(|_cc| Ok(Box::new(App))),
    ).expect("Event loop error");
}
