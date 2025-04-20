const WINDING_ORDER: [i32; 6] = [
    0, 1, 2,
    2, 3, 0
];

fn size_of<T>(vec: &Vec<T>) -> i32 {
    vec.len() as i32
}

struct Mesh {
    vertices: Vec<f32>,
    elements: Vec<i32>
}

impl Mesh {
    pub fn new() -> Mesh {
        Mesh {
            vertices: vec![],
            elements: vec![]
        }
    }

    pub fn vertex(&mut self, x: f32, y: f32, z: f32) {
        self.vertices.push(x);
        self.vertices.push(y);
        self.vertices.push(z);
        self.vertices.push(1.0);
    }

    pub fn triangulate(&mut self, format: i32) {
        let quads = self.size_of(format);

        let mut previous = 0;
        for i in 0 .. quads {
            self.elements.push(WINDING_ORDER[0] + previous);
            self.elements.push(WINDING_ORDER[1] + previous);
            self.elements.push(WINDING_ORDER[2] + previous);
            self.elements.push(WINDING_ORDER[3] + previous);
            self.elements.push(WINDING_ORDER[4] + previous);
            self.elements.push(WINDING_ORDER[5] + previous);
            
            previous += 4;
        }
    }

    pub fn get_count(&self, format: i32) -> i32 {
        6 * self.size_of(format)
    }
    
    fn size_of(&self, format: i32) -> i32 {
        size_of(&self.vertices) / format
    }
}