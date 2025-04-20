extern crate glfw;

use glfw::{Context, Glfw, PWindow};
use wgpu::rwh::{DisplayHandle, HandleError, HasDisplayHandle, HasWindowHandle, WindowHandle};

pub struct Window {
    glfw: Glfw,
    window: PWindow,
    size: (i32, i32),
}

impl Window {
    pub fn new() -> Window {
        let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();

        let size = (800, 600);
        let mut window = glfw
            .with_primary_monitor(|glfw, m| {
                let monitor = m.expect("Failed to get primary monitor.");

                let (_x, _y, monitor_width, monitor_height) = monitor.get_workarea();

                let (width, height) = size;
                let (x, y) = (
                    (monitor_width - width) / 2,
                    (monitor_height - height) / 2
                );

                let (mut window, _events) = glfw.create_window(
                    width as u32,
                    height as u32, "Game", glfw::WindowMode::Windowed, )
                    .expect("Failed to create a new GLFW window.");
                window.set_pos(x, y);
                window
            });

        window.make_current();
        Window {
            glfw, window, size
        }
    }

    pub fn should_close(&self) -> bool {
        self.window.should_close()
    }

    pub fn poll(&mut self) {
        self.glfw.poll_events();
        self.window.swap_buffers();
    }

    pub fn get_size(&self) -> (i32, i32) {
        self.size
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        self.window.set_should_close(true);
    }
}

impl HasWindowHandle for Window {
    fn window_handle(&self) -> Result<WindowHandle<'_>, HandleError> {
        Ok(self.window.window_handle()?)
    }
}

impl HasDisplayHandle for Window {
    fn display_handle(&self) -> Result<DisplayHandle<'_>, HandleError> {
        Ok(self.window.display_handle()?)
    }
}
