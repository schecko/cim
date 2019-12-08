
struct Shader(u32);

unsafe fn compile_shader(sources: Vec<&str>, shader_type: u32) -> Result<Shader, String> {
    let vertex_shader = gl::CreateShader(shader_type);
    let counts: Vec<_> = sources.iter().map(|source| source.len() as i32).collect();
    gl::ShaderSource(vertex_shader, 1, sources.as_ptr() as *const *const i8, counts.as_ptr() as *const _);
    gl::CompileShader(vertex_shader);

    let mut success = 0;
    let mut info_log = [0u8; 512];
    gl::GetShaderiv(vertex_shader, gl::COMPILE_STATUS, &mut success as *mut _);
    if success == 0 {
        gl::GetShaderInfoLog(vertex_shader, info_log.len() as _, std::ptr::null_mut(), info_log.as_mut_ptr() as *mut i8);
        return Err(String::from_utf8_lossy(&info_log[..]).to_string())
    }

    Ok(Shader(vertex_shader))
}

pub struct Pipeline(pub u32);

impl Pipeline {
    pub fn new(vert: &str, frag: &str) -> Result<Pipeline, String> {
        let pipeline = unsafe {
            let vertex_shader = compile_shader(vec![vert], gl::VERTEX_SHADER)?;
            let fragment_shader = compile_shader(vec![frag], gl::FRAGMENT_SHADER)?;

            let pipeline = gl::CreateProgram();
            gl::AttachShader(pipeline, vertex_shader.0);
            gl::AttachShader(pipeline, fragment_shader.0);
            gl::LinkProgram(pipeline);

            let mut success = 0;
            let mut info_log = [0u8; 512];
            gl::GetProgramiv(pipeline, gl::LINK_STATUS, &mut success as *mut _);
            if success == 0 {
                gl::GetProgramInfoLog(pipeline, info_log.len() as _, std::ptr::null_mut(), info_log.as_mut_ptr() as *mut i8);
                return Err(String::from_utf8_lossy(&info_log[..]).to_string())
            }

            gl::DeleteShader(vertex_shader.0);
            gl::DeleteShader(fragment_shader.0);

            pipeline
        };

        Ok(Pipeline(pipeline))
    }

    pub fn set_use(&self) {
        unsafe {
            gl::UseProgram(self.0);
        }
    }
}

