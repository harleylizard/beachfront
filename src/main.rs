mod window;
mod wgpu;

fn main() {
    let mut window = window::Window::new();
    
    while !window.should_close() {
        window.poll();
        
    }
}
