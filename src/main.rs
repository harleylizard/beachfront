mod window;
mod wgpu;
mod opengl;
mod pipeline;
mod mesh;

fn main() {
    let mut window = window::Window::new();

    let pipeline = pipeline::Pipeline::new(&window);
    
    while !window.should_close() {
        window.poll();
        
        pipeline.draw();
        
    }
}
