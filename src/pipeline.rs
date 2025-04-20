use crate::window::Window;

pub struct Pipeline {
}

impl Pipeline {

    // OpenGL 4.6, WGPU doesn't provide as a back layer.
    #[cfg(target_os = "windows")]
    pub fn new(window: &Window) -> Pipeline {
        Pipeline {}
    }

    #[cfg(not(target_os = "windows"))]
    pub fn new(window: &Window) -> Pipeline {
        Pipeline {}
    }

    #[cfg(target_os = "windows")]
    pub fn draw(&self) {
        
    }

    #[cfg(not(target_os = "windows"))]
    pub fn draw(&self) {
        
    }
}
