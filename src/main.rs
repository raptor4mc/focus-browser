use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop},
    window::{Window, WindowId},
};

struct App {
    window: Option<Window>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_none() {
            let window = event_loop.create_window(
                Window::default_attributes()
                    .with_title("Focus Browser — P1 Window")
                    .with_inner_size(winit::dpi::LogicalSize::new(1024.0, 768.0))
            ).expect("Failed to build winit window");
            window.request_redraw();
            self.window = Some(window);
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        if let Some(ref window) = self.window {
            if window_id != window.id() {
                return;
            }
        }

        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::Resized(physical_size) => {
                println!(
                    "P1 Window resized: {}x{} (physical)",
                    physical_size.width, physical_size.height
                );
            }
            WindowEvent::ScaleFactorChanged { scale_factor, inner_size_writer: _ } => {
                println!("P1 Scale changed: factor={}", scale_factor);
            }
            _ => {}
        }
    }

    fn redraw_requested(&mut self, _event_loop: &ActiveEventLoop, window_id: WindowId) {
        if let Some(ref window) = self.window {
            if window.id() == window_id {
                // Black screen: default window background is black.
                // No GPU/render integration until P6; this keeps P1 isolated.
            }
        }
    }
}

fn main() {
    // HUMAN REVIEW ONLY — per agent.md assembly rules.
    // This links P1 `window` into the root binary. Revert if issues found.
    // Security: isolated window creation; no network, no DOM, no JS, no GPU.
    // No integration with fetch/DOM/CSS until ASM1.
    let event_loop = EventLoop::new().expect("Failed to create event loop");
    let mut app = App { window: None };
    event_loop.run_app(&mut app).expect("Event loop error");
}
