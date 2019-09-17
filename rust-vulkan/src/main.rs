use winit::EventsLoop;
use winit::WindowBuilder;
use winit::{Event, WindowEvent};
use winit::dpi::LogicalSize;
use vulkano::instance::{Instance, InstanceExtensions, ApplicationInfo, Version, layers_list, PhysicalDevice, QueueFamily};
use std::sync::Arc;
use vulkano::instance::debug::{DebugCallback, MessageTypes};
use vulkano::device::{Device, DeviceExtensions, Queue, Features};

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

// a rust struct is basically a Kotlin data class, or more generally a named Tuple
#[allow(unused)]
struct HelloTriangleApp {
    //vulkan
    instance: Arc<Instance>,
    debug_callback: Option<DebugCallback>,
    physical_device_index: usize, // can't store PhysicalDevice directly (lifetime issues)
    device: Arc<Device>,
    graphics_queue: Arc<Queue>,

    //winit
    events_loop: EventsLoop,
}

struct QueueFamilyIndices {
    graphics_family: i32
}

impl QueueFamilyIndices {
    const NOT_INITIALIZED: i32 = -1;

    fn new() -> Self {
        Self {
            graphics_family: Self::NOT_INITIALIZED
        }
    }

    fn is_complete(&self) -> bool {
        return self.graphics_family != Self::NOT_INITIALIZED
    }
}

//Vulkan standard validation layers init
const VALIDATION_LAYERS: &[&str; 1] = &["VK_LAYER_LUNARG_standard_validation"];

#[cfg(all(debug_assertions))]
const ENABLE_VALIDATION_LAYERS: bool = true;
#[cfg(not(debug_assertions))]
const ENABLE_VALIDATION_LAYERS: bool = false;

// associated functions on the struct
impl HelloTriangleApp {
    //capital Self = type, HelloTriangleApp in this case
    fn init() -> Self {
        let instance = Self::init_instance();
        let debug_callback = Self::setup_debug_callback(&instance);
        let physical_device_index = Self::get_physical_device_index(&instance);
        let (device, graphics_queue) = Self::create_logical_device(&instance, physical_device_index);
        let events_loop = Self::init_window();

        Self {
            instance,
            debug_callback,
            physical_device_index,
            device,
            graphics_queue,
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

    fn get_physical_device_index(instance: &Arc<Instance>) -> usize {
        let physical_device = PhysicalDevice::enumerate(&instance)
            .find(|device| Self::is_physical_device_suitable(device))
            .expect("failed to find a suitable GPU!");

        println!("Using device: {} (type: {:?})", physical_device.name(), physical_device.ty());
        return physical_device.index();
    }

    fn is_physical_device_suitable(device: &PhysicalDevice) -> bool {
        let indices = Self::find_queue_families(device);
        return indices.is_complete();
    }


    fn find_queue_families(device: &PhysicalDevice) -> QueueFamilyIndices {
        let mut indices = QueueFamilyIndices::new();

        for (i, queue_family) in device.queue_families().enumerate() {
            if queue_family.supports_graphics() {
                indices.graphics_family = i as i32;
            }

            if indices.is_complete() {
                break;
            }
        }

        return indices;
    }

    //I'm not sure I understand why i have to explicitly define the lifetime here?
    fn get_graphics_family_from_physical_device<'a>(physical_device: &'a PhysicalDevice) -> QueueFamily<'a> {
        let indices = Self::find_queue_families(&physical_device);
        let queue_family = physical_device.queue_families().nth(indices.graphics_family as usize).unwrap();
        return queue_family
    }

    fn create_logical_device(instance: &Arc<Instance>, physical_device_index: usize) -> (Arc<Device>, Arc<Queue>) {
        let physical_device = PhysicalDevice::from_index(instance, physical_device_index).unwrap();
        let graphics_family = Self::get_graphics_family_from_physical_device(&physical_device);

        let features = Features::none();
        let extensions = DeviceExtensions::none();

        //priorities, list of pairs, is option some iterable? I don't get how this would work otherwise
        //actually, the code is looking for anything that can implement IntoIterator<Item = (QueueFamily<'a>, f32)>
        //so, option works
        let queue_families = Some((graphics_family, 1.0));

        let (device, mut queues_iter) = Device::new(physical_device, &features, &extensions, queue_families)
            .expect("Couldn't build logical device!");

        //only 1 queue for now
        let queues = queues_iter.next().unwrap();
        return (device, queues)
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
