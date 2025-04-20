extern crate glfw;

use glfw::{Context, Glfw, PWindow};

pub struct Window {
    glfw: Glfw,
    window: PWindow
}

impl Window {
    pub(crate) fn new() -> Window {
        let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();

        let mut window = glfw
            .with_primary_monitor(|glfw, m| {
                let monitor = m.expect("Failed to get primary monitor");
                
                let (_x, _y, monitor_width, monitor_height) = monitor.get_workarea();
                
                let (width, height) = (800, 600);
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
            glfw, window
        }
    }

    pub(crate) fn should_close(&self) -> bool {
        self.window.should_close()
    }

    pub(crate) fn poll(&mut self) {
        self.glfw.poll_events();
        self.window.swap_buffers();
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        self.window.set_should_close(true);
    }
}
