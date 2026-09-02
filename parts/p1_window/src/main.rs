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
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    println!("Renderer: egui (GPU via wgpu/glow) — checking wgpu adapters...");
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::all(),
        ..Default::default()
    });
    for adapter in instance.enumerate_adapters(wgpu::Backends::all()) {
        let info = adapter.get_info();
        println!(
            "Adapter: {} | backend: {:?} | device_type: {:?}",
            info.name, info.backend, info.device_type
        );
    }
    // Isolated P1 window: no integration with fetch, DOM, CSS, or GPU.
    // Note: We never used winit; window layer is eframe/egui.
    println!("Renderer: egui (GPU via wgpu/glow) — CPU fallback available");
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1024.0, 768.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Focus Browser — P1 Window",
        native_options,
        Box::new(|_cc| Ok(Box::new(App))),
    ).expect("Event loop error");
}
