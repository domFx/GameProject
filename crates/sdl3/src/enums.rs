/// SDL3 Renderer Types
/// This enum represents the different types of renderers that are supported by SDL3.
pub enum SDL3RendererType {
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