use sdl3::enums::SDL3RendererType;
use sdl3::{SDL3Wrapper, SDL3RendererWrapper, SDL3WindowWrapper};

use log::{error, info};
use sdl3_sys::{events::SDL_Event};
use sdl3_main::{app_impl, AppResult};

struct ApplicationState {
    current_state: AppResult,
    sdl3: SDL3Wrapper,
    window: SDL3WindowWrapper,
    renderer: SDL3RendererWrapper,
}

#[app_impl]
impl ApplicationState {
    fn app_init() -> Option<Box<std::sync::Mutex<ApplicationState>>> {
        let sdl3_wrapper = SDL3Wrapper { };

        sdl3_wrapper.sdl_set_app_metadata("SDL3 Application", "1.0.0", "com.example.sdl3app");

        if !sdl3_wrapper.sdl_init(sdl3_sys::init::SDL_INIT_VIDEO) {
            error!("Failed to initialize SDL3");
            return None;
        }

        let window_flags = sdl3_sys::everything::SDL_WINDOW_RESIZABLE | sdl3_sys::everything::SDL_WINDOW_HIGH_PIXEL_DENSITY;
        let window = sdl3_wrapper.sdl_create_window("SDL3 Application", 800, 600, window_flags);

        if window.is_none() {
            error!("Failed to create SDL3 window");
            return None;
        }

        let mut window = window.unwrap();

        let renderer = sdl3_wrapper.sdl_create_renderer(&mut window, SDL3RendererType::OpenGL);
        if renderer.is_none() {
            error!("Failed to create SDL3 renderer");
            return None;
        }

        let renderer = renderer.unwrap();

        Some(Box::new(std::sync::Mutex::new(ApplicationState {
            sdl3: sdl3_wrapper,
            current_state: AppResult::Continue,
            window,
            renderer
        })))
    }

    fn app_iterate(&mut self) -> AppResult {
        // println!("Application iterating");

        AppResult::Continue
    }

    fn app_event(&mut self, event: &SDL_Event) -> AppResult {
        let event_type = self.sdl3.sdl_get_sdl_event(event);

        if event_type == sdl3_sys::events::SDL_EVENT_QUIT {
            info!("Received quit event");
            return AppResult::Success;
        }     

        AppResult::Continue
    }
}