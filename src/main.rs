use std::ptr::NonNull;

use sdl3_sys::{events::SDL_Event, render};
use sdl3_main::{app_impl, AppResult};

struct ApplicationState {
    current_state: AppResult,
    window: SDL3WindowWrapper,
    renderer: SDL3RendererWrapper,
}

#[app_impl]
impl ApplicationState {
    fn app_init() -> Option<Box<std::sync::Mutex<ApplicationState>>> {
        let sdl3_wrapper = SDL3Wrapper { };

        sdl3_wrapper.sdl_set_app_metadata("SDL3 Application", "1.0.0", "com.example.sdl3app");

        if !sdl3_wrapper.sdl_init(sdl3_sys::init::SDL_INIT_VIDEO) {
            println!("Failed to initialize SDL3");
            return None;
        }

        let window_flags = sdl3_sys::everything::SDL_WINDOW_RESIZABLE; //| sdl3_sys::everything::SDL_WINDOW_HIGH_PIXEL_DENSITY;
        let window = sdl3_wrapper.sdl_create_window("SDL3 Application", 800, 600, window_flags);

        if window.is_none() {
            println!("Failed to create SDL3 window");
            return None;
        }

        let mut window = window.unwrap();

        let renderer = sdl3_wrapper.sdl_create_renderer(&mut window, SDL3RendererType::OpenGL);
        if renderer.is_none() {
            println!("Failed to create SDL3 renderer");
            return None;
        }

        let renderer = renderer.unwrap();
        


        Some(Box::new(std::sync::Mutex::new(ApplicationState {
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
        let event_type = unsafe { sdl3_sys::events::SDL_EventType(event.r#type) };

        // Todo: Figure out how to handle events properly
        if event_type == sdl3_sys::events::SDL_EVENT_QUIT {
            println!("Received quit event");
            return AppResult::Success;
        }     

        AppResult::Continue
    }
}


struct SDL3Wrapper { }

struct SDL3WindowWrapper {
    pub(crate) window_ptr: NonNull<sdl3_sys::video::SDL_Window>
}

unsafe impl Sync for SDL3WindowWrapper {}
unsafe impl Send for SDL3WindowWrapper {}

struct SDL3RendererWrapper {
    pub(crate) renderer_ptr: NonNull<sdl3_sys::everything::SDL_Renderer>
}

unsafe impl Sync for SDL3RendererWrapper {}
unsafe impl Send for SDL3RendererWrapper {}

enum SDL3RendererType {
    OpenGL,
    Vulkan,
    Metal,
    Direct3D,
}

impl SDL3RendererType {
    pub fn get_renderer_name(&self) -> &str {
        match self {
            SDL3RendererType::OpenGL => "opengl",
            SDL3RendererType::Vulkan => "vulkan",
            SDL3RendererType::Metal => "metal",
            SDL3RendererType::Direct3D => "direct3d",
        }
    }
}

impl SDL3Wrapper {
    pub fn sdl_init(&self, init_flags: sdl3_sys::init::SDL_InitFlags) -> bool {
        unsafe {
            sdl3_sys::init::SDL_Init(init_flags)
        }
    }

    pub fn sdl_create_window(&self, title: &str, width: i32, height: i32, window_flags: sdl3_sys::video::SDL_WindowFlags) -> Option<SDL3WindowWrapper> {
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
                Some(SDL3WindowWrapper { window_ptr })
            }
        }
    }

    pub fn sdl_create_renderer(&self, window: &mut SDL3WindowWrapper, renderer_type: SDL3RendererType) -> Option<SDL3RendererWrapper> {
        let title_cstr = std::ffi::CString::new(renderer_type.get_renderer_name()).unwrap();
        unsafe {
            let renderer = sdl3_sys::render::SDL_CreateRenderer(
                window.window_ptr.as_ptr(),
                title_cstr.as_ptr(),
            );

            if renderer.is_null() {
                let sdl_error_msg = std::ffi::CStr::from_ptr(sdl3_sys::everything::SDL_GetError());

                let msg = format!("SDL ERROR: {:?}", sdl_error_msg.to_str());
                let c_msg = std::ffi::CString::new(msg).unwrap();
                sdl3_sys::log::SDL_Log(c_msg.as_ptr());
                None
            } else {
                let renderer_ptr = NonNull::new_unchecked(renderer);
                Some(SDL3RendererWrapper { renderer_ptr })
            }
        }
    }

    pub fn sdl_set_app_metadata(&self, app_name: &str, app_version: &str, app_identifier: &str) {
        let appname = std::ffi::CString::new(app_name).unwrap();
        let appversion = std::ffi::CString::new(app_version).unwrap();
        let appidentifier = std::ffi::CString::new(app_identifier).unwrap();

        unsafe {
            sdl3_sys::everything::SDL_SetAppMetadata(appname.as_ptr(), appversion.as_ptr(), appidentifier.as_ptr());
        }
    }

    pub fn destroy_window(&self, window: SDL3WindowWrapper) {
        unsafe {
            sdl3_sys::video::SDL_DestroyWindow(window.window_ptr.as_ptr());
        }
    }
}