use std::ptr::NonNull;
use sdl3_sys::events::{SDL_Event, SDL_EventType};

use crate::enums::SDL3RendererType;

// Modules
pub mod enums;

pub struct SDL3Wrapper { }

pub struct SDL3WindowWrapper {
    pub(crate) window_ptr: NonNull<sdl3_sys::video::SDL_Window>
}

unsafe impl Sync for SDL3WindowWrapper {}
unsafe impl Send for SDL3WindowWrapper {}

pub struct SDL3RendererWrapper {
    pub(crate) renderer_ptr: NonNull<sdl3_sys::everything::SDL_Renderer>
}

unsafe impl Sync for SDL3RendererWrapper {}
unsafe impl Send for SDL3RendererWrapper {}

/// SDL3Wrapper Implementation
impl SDL3Wrapper {
    /// SDL_Init
    pub fn sdl_init(&self, init_flags: sdl3_sys::init::SDL_InitFlags) -> bool {
        unsafe {
            sdl3_sys::init::SDL_Init(init_flags)
        }
    }

    /// SDL_CreateWindow
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
                self.sdl_log();
                None
            } else {
                let window_ptr = NonNull::new_unchecked(window);
                Some(SDL3WindowWrapper { window_ptr })
            }
        }
    }

    /// SDL_CreateRenderer
    pub fn sdl_create_renderer(&self, window: &mut SDL3WindowWrapper, renderer_type: SDL3RendererType) -> Option<SDL3RendererWrapper> {
        let title_cstr = std::ffi::CString::new(renderer_type.get_renderer_name()).unwrap();
        unsafe {
            let renderer = sdl3_sys::render::SDL_CreateRenderer(
                window.window_ptr.as_ptr(),
                title_cstr.as_ptr(),
            );

            if renderer.is_null() {
                self.sdl_log();
                None
            } else {
                let renderer_ptr = NonNull::new_unchecked(renderer);
                Some(SDL3RendererWrapper { renderer_ptr })
            }
        }
    }

    /// SDL_SetAppMetadata
    pub fn sdl_set_app_metadata(&self, app_name: &str, app_version: &str, app_identifier: &str) {
        let appname = std::ffi::CString::new(app_name).unwrap();
        let appversion = std::ffi::CString::new(app_version).unwrap();
        let appidentifier = std::ffi::CString::new(app_identifier).unwrap();

        unsafe {
            sdl3_sys::everything::SDL_SetAppMetadata(appname.as_ptr(), appversion.as_ptr(), appidentifier.as_ptr());
        }
    }

    /// Gets an SDL_Event from a u32
    pub fn sdl_get_sdl_event(&self, event: &SDL_Event) -> SDL_EventType {
        unsafe { 
            sdl3_sys::events::SDL_EventType(event.r#type)
        }
    }

    /// SDL_Log
    /// This function logs the SDL error message to the console.
    pub(crate) fn sdl_log(&self) {
        unsafe {
            let sdl_error_msg = std::ffi::CStr::from_ptr(sdl3_sys::everything::SDL_GetError());
            let formatted_msg = format!("SDL ERROR: {:?}", sdl_error_msg.to_str());
            let log_cstr = std::ffi::CString::new(formatted_msg).unwrap();
            sdl3_sys::log::SDL_Log(log_cstr.as_ptr());
        }
    }
}