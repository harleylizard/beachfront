mod window;
mod wgpu;

fn main() {
    let mut window = window::Window::new();

    // let wgpu = pollster::block_on(unsafe { wgpu::Wgpu::new(&window) });
    
    while !window.should_close() {
        window.poll();
        
    }
}
