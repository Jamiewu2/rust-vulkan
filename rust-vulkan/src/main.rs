use winit::EventsLoop;
use winit::WindowBuilder;
use winit::{Event, WindowEvent};
use winit::dpi::LogicalSize;
use vulkano::instance::{Instance, InstanceExtensions, ApplicationInfo, Version, layers_list, PhysicalDevice};
use std::sync::Arc;
use vulkano::instance::debug::{DebugCallback, MessageTypes};

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

// a rust struct is basically a Kotlin data class, or more generally a named Tuple
#[allow(unused)]
struct HelloTriangleApp<'a> {
    //vulkan
    instance: Arc<Instance>,
    debug_callback: Option<DebugCallback>,
    physical_device: PhysicalDevice<'a>,

    //winit
    events_loop: EventsLoop,
}

struct QueueFamilyIndices {
    graphics_family: Option<u32>,
}

impl QueueFamilyIndices {
    fn new() -> Self {
        let a: Option<u32> = None;
        Self {
            graphics_family: a
        }
    }

    fn is_complete(&self) -> bool {
        return self.graphics_family.is_some()
    }
}

//Vulkan standard validation layers init
const VALIDATION_LAYERS: &[&str; 1] = &["VK_LAYER_LUNARG_standard_validation"];

#[cfg(all(debug_assertions))]
const ENABLE_VALIDATION_LAYERS: bool = true;
#[cfg(not(debug_assertions))]
const ENABLE_VALIDATION_LAYERS: bool = false;

// associated functions on the struct
impl<'a> HelloTriangleApp<'a> {
    //capital Self = type, HelloTriangleApp in this case
    fn init() -> Self {
        let instance = Self::init_instance();
        let debug_callback = Self::setup_debug_callback(&instance);
        let physical_device = Self::get_physical_device(&instance);
        let events_loop = Self::init_window();

        Self {
            instance,
            debug_callback,
            physical_device,
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

    fn init_instance() -> Arc<Instance> {
        let supported_extensions = InstanceExtensions::supported_by_core()
            .expect("failed to retrieve supported extensions");
        println!("Supported extension: {:?}", supported_extensions);

        let app_info = ApplicationInfo {
            application_name: Some("Hello Triangle".into()),
            application_version: Some(Version { major: 1, minor: 0, patch: 0 }),
            engine_name: Some("No Engine".into()),
            engine_version: Some(Version { major: 1, minor: 0, patch: 0 }),
        };

        let required_extensions = Self::get_required_extensions();

        if ENABLE_VALIDATION_LAYERS && Self::check_validation_layer_support() {
            Instance::new(Some(&app_info), &required_extensions, VALIDATION_LAYERS.iter().cloned())
                .expect("failed to create Vulkan instance")
        } else {
            Instance::new(Some(&app_info), &required_extensions, None)
                .expect("failed to create Vulkan instance")
        }
    }

    fn check_validation_layer_support() -> bool {
        let validation_layers = layers_list().unwrap().map(|layer| layer.name().to_owned()).collect::<Vec<String>>();

        return VALIDATION_LAYERS.iter()
            .all(|layer_name| validation_layers.contains(&layer_name.to_string()))
    }

    fn get_required_extensions() -> InstanceExtensions {
        let mut required_extensions = vulkano_win::required_extensions();
        if ENABLE_VALIDATION_LAYERS {
            // TODO!: this should be ext_debug_utils (_report is deprecated), but that doesn't exist yet in vulkano
            required_extensions.ext_debug_report = true;
        }

        return required_extensions;
    }

    fn setup_debug_callback(instance: &Arc<Instance>) -> Option<DebugCallback> {
        if !ENABLE_VALIDATION_LAYERS {
            return None
        }

        let msg_types = MessageTypes {
            error: true,
            warning: true,
            performance_warning: true,
            information: false,
            debug: true,
        };

        let callback = DebugCallback::new(&instance, msg_types, |msg| {
            println!("validation layer: {:?}", msg.description);
        }).ok();

        return callback;
    }

    fn get_physical_device(instance: &Arc<Instance>) -> PhysicalDevice {
        let physical_device = PhysicalDevice::enumerate(&instance)
            .find(|device| Self::is_physical_device_suitable(device))
            .expect("failed to find a suitable GPU!");

        println!("Using device: {} (type: {:?})", physical_device.name(), physical_device.ty());
        return physical_device;
    }

    fn is_physical_device_suitable(device: &PhysicalDevice) -> bool {
        let indices = Self::find_queue_families(device);
        return indices.is_complete();
    }


    fn find_queue_families(device: &PhysicalDevice) -> QueueFamilyIndices {
        let mut indices = QueueFamilyIndices::new();

        for (i, queue_family) in device.queue_families().enumerate() {
            if queue_family.supports_graphics() {
                indices.graphics_family = Some(i as u32);
            }

            if indices.is_complete() {
                break;
            }
        }

        return indices;
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
