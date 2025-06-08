use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::{Window, WindowId},
};

use std::env;

struct App {
    window: Option<Window>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        println!("✓ resumed() called");
        
        if self.window.is_some() {
            println!("Window already exists, skipping creation");
            return;
        }
        
        println!("Creating window attributes...");
        let window_attributes = Window::default_attributes()
            .with_title("Debug Winit Window")
            .with_inner_size(winit::dpi::LogicalSize::new(1920, 1080));
        
        println!("Calling create_window...");
        match event_loop.create_window(window_attributes) {
            Ok(window) => {
                println!("✓ Window created! ID: {:?}", window.id());
                println!("Window visible: {:?}", window.is_visible());
                println!("Window inner size: {:?}", window.inner_size());
                
                // Force show the window
                window.set_visible(true);
                window.request_redraw();
                
                self.window = Some(window);
                println!("✓ Window stored in App struct");
            }
            Err(e) => {
                eprintln!("✗ Failed to create window: {}", e);
                event_loop.exit();
            }
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        println!("Window event: {:?} for window {:?}", event, window_id);
        
        match event {
            WindowEvent::CloseRequested => {
                println!("✓ Close requested - exiting");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                println!("✓ Redraw requested");
                if let Some(window) = &self.window {
                    // Keep requesting redraws to maintain window
                    window.request_redraw();
                }
            }
            WindowEvent::Resized(size) => {
                println!("✓ Window resized to: {:?}", size);
            }
            _ => {}
        }
    }

    fn new_events(&mut self, _event_loop: &ActiveEventLoop, _cause: winit::event::StartCause) {
        println!("new_events called");
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        // Don't spam this one
        // println!("about_to_wait called");
    }
}

pub fn Renderer() -> Result<(), Box<dyn std::error::Error>> {

    println!("🚀 Starting winit debug application...");


    println!("Creating event loop...");
    let event_loop = match EventLoop::new() {
        Ok(el) => {
            println!("✓ Event loop created");
            el
        }
        Err(e) => {
            eprintln!("✗ Failed to create event loop: {}", e);
            return Err(e.into());
        }
    };
    
    println!("Setting control flow to Wait...");
    event_loop.set_control_flow(ControlFlow::Wait);
    
    let mut app = App { window: None };
    
    println!("Starting event loop...");
    println!("If you don't see 'resumed() called' next, the issue is with the event loop setup");
    
    match event_loop.run_app(&mut app) {
        Ok(()) => println!("✓ Event loop exited normally"),
        Err(e) => eprintln!("✗ Event loop error: {}", e),
    }
    
    Ok(())
}

fn main()
{
	Renderer();
}