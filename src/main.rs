use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

fn main() {
    // Event loop and window
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("TERA_Ventl Engine")
        .build(&event_loop)
        .unwrap();

    // Start main loop
    event_loop.run(move |event, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent { 
                event: WindowEvent::CloseRequested,
                ..
            } => {
                *control_flow = ControlFlow::Exit;
            }
            Event::RedrawRequested(_) => {
                // Renderer stuff - nothing for now
            }
            _ => (),
        }
    });
}

