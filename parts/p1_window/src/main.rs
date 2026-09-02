use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

fn main() {
    // Isolated P1 window: no integration with fetch, DOM, CSS, or GPU.
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Focus Browser — P1 Window")
        .with_inner_size(winit::dpi::LogicalSize::new(1024.0, 768.0))
        .build(&event_loop)
        .expect("Failed to build winit window");

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => {
                // Explicit close handling per agent.md P1 success criteria.
                *control_flow = ControlFlow::Exit;
            }
            Event::WindowEvent {
                event: WindowEvent::Resized(physical_size),
                window_id,
            } if window_id == window.id() => {
                // Resize handled; dimensions logged for verification.
                println!(
                    "P1 Window resized: {}x{} (physical)",
                    physical_size.width, physical_size.height
                );
            }
            Event::WindowEvent {
                event: WindowEvent::ScaleFactorChanged { new_inner_size, scale_factor },
                window_id,
            } if window_id == window.id() => {
                println!(
                    "P1 Scale changed: factor={}, inner_size={:?}",
                    scale_factor, new_inner_size
                );
            }
            _ => {}
        }
    });
}
