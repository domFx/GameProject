use std::ptr::NonNull;

use sdl3_sys::events::SDL_Event;
use sdl3_main::{app_impl, AppResult};

struct ApplicationState {
    current_state: AppResult
}

#[app_impl]
impl ApplicationState {
    fn app_init() -> Option<Box<std::sync::Mutex<ApplicationState>>> {
        let sdl3_wrapper = SDL3Wrapper { };

        if !sdl3_wrapper.sdl_init(sdl3_sys::init::SDL_INIT_VIDEO) {
            println!("Failed to initialize SDL3");
            return None;
        }

        let window_flags = sdl3_sys::everything::SDL_WINDOW_RESIZABLE | sdl3_sys::everything::SDL_WINDOW_HIGH_PIXEL_DENSITY;
        if sdl3_wrapper.sdl_create_window("SDL3 Application", 800, 600, window_flags).is_none() {
            println!("Failed to create SDL3 window");
            return None;
        }

        Some(Box::new(std::sync::Mutex::new(ApplicationState {
            current_state: AppResult::Continue
        })))
    }

    fn app_iterate(&mut self) -> AppResult {
        println!("Application iterating");

        AppResult::Success
    }

    fn app_event(&mut self, _event: &SDL_Event) -> AppResult {
        println!("Application event received");

        AppResult::Continue
    }
}


struct SDL3Wrapper{}

struct SDL3WindowWrapper {
    ptr: NonNull<sdl3_sys::everything::SDL_Window>
}

impl SDL3Wrapper {
    pub fn sdl_init(&self, init_flags: sdl3_sys::init::SDL_InitFlags) -> bool {
        unsafe {
            sdl3_sys::init::SDL_Init(init_flags)
        }
    }

    pub fn sdl_create_window(&self, title: &str, width: i32, height: i32, window_flags: sdl3_sys::everything::SDL_WindowFlags) -> Option<SDL3WindowWrapper> {
        let title_cstr = std::ffi::CString::new(title).unwrap();
        unsafe {
            let window = sdl3_sys::video::SDL_CreateWindow(
                title_cstr.as_ptr(),
                width,
                height,
                window_flags
            );

            if window.is_null() {
                None
            } else {
                let window_ptr = NonNull::new_unchecked(window);
                Some(SDL3WindowWrapper { ptr: window_ptr })
            }
        }
    }
}