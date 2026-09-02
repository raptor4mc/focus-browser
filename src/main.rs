use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};

fn main() {
    // HUMAN REVIEW ONLY — per agent.md assembly rules.
    // This links P1 `window` into the root binary. Revert if issues found.
    // Security: isolated window creation; no network, no DOM, no JS, no GPU.
    // No integration with fetch/DOM/CSS until ASM1.
    let event_loop = EventLoop::new().expect("Failed to create event loop");
    let window = WindowBuilder::new()
        .with_title("Focus Browser — P1 Window")
        .with_inner_size(winit::dpi::LogicalSize::new(1024.0, 768.0))
        .build(&event_loop)
        .expect("Failed to build winit window");

    event_loop.run(move |event, active_event_loop| {
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => {
                active_event_loop.exit();
            }
            Event::WindowEvent {
                event: WindowEvent::Resized(physical_size),
                window_id,
            } if window_id == window.id() => {
                println!(
                    "P1 Window resized: {}x{} (physical)",
                    physical_size.width, physical_size.height
                );
            }
            Event::WindowEvent {
                event: WindowEvent::ScaleFactorChanged { scale_factor, inner_size_writer: _ },
                window_id,
            } if window_id == window.id() => {
                println!(
                    "P1 Scale changed: factor={}",
                    scale_factor
                );
            }
            _ => {}
        }
    }).expect("Event loop error");
}
