use winit::EventsLoop;
use winit::WindowBuilder;
use winit::{Event, WindowEvent};
use winit::dpi::LogicalSize;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

// a rust struct is basically a Kotlin data class, or more generally a named Tuple
#[allow(unused)]
struct HelloTriangleApp {
    events_loop: EventsLoop,
}

// associated functions on the struct
impl HelloTriangleApp {
    //capital Self = type, HelloTriangleApp in this case
    fn init() -> Self {
        let events_loop = Self::init_window();

        Self {
            events_loop
        }
    }

    fn init_window() -> EventsLoop {
        let event_loop = EventsLoop::new();
        let _window_builder = WindowBuilder::new()
            .with_title("Vulkan")
            .with_dimensions(LogicalSize::new(f64::from(WIDTH), f64::from(HEIGHT)))
            .build(&event_loop)
            .unwrap();
        return event_loop;
    }

    //&mut self = self: &mut Self
    fn main_loop(&mut self) {
        //why is there a builtin infinite loop construct in rust?
        loop {
            let mut done = false;
            self.events_loop.poll_events( |event| {
                if let Event::WindowEvent { event: WindowEvent::CloseRequested, .. } = event {
                    done = true
                }
            });
            if done {
                return;
            }
        }
    }
}





fn main() {
    let mut app = HelloTriangleApp::init();
    app.main_loop();
}
