use std::fs;
use gl::types::{GLenum, GLuint};

pub struct ProgramPipeline {
    pipeline: GLuint
}

impl ProgramPipeline {
    pub fn new(fragment: GLuint, vertex: GLuint) -> ProgramPipeline {
        unsafe {
            let mut pipeline: GLuint = 0;
            gl::CreateProgramPipelines(1, &mut pipeline);
            
            gl::UseProgramStages(pipeline, gl::FRAGMENT_SHADER_BIT, fragment);
            gl::UseProgramStages(pipeline, gl::VERTEX_SHADER_BIT, vertex);
            
            gl::ValidateProgramPipeline(pipeline);
            ProgramPipeline {
                pipeline
            }
        }
    }
    
}

pub fn programs_of(tuple: (String, String)) -> (GLuint, GLuint) {
    let (fragment, vertex) = tuple;
    (
        program_of(gl::FRAGMENT_SHADER, fragment),
        program_of(gl::VERTEX_SHADER, vertex)
    )
}

pub fn program_of(shader: GLenum, path: String) -> GLuint {
    unsafe {
        let source = match fs::read_to_string(&path) {
            Ok(src) => src,
            Err(e) => return Err("Failed to read shader.").unwrap(),
        };
        gl::CreateShaderProgramv(shader, 1, source.as_ptr() as *const _)
    }
}