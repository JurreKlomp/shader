pub struct Compiled;
pub struct Uncompiled;

pub struct ShaderProgram<State = Uncompiled> {
    pub id: u32,
    shader_ids: Vec<u32>,
    state: std::marker::PhantomData<State>,
}

impl ShaderProgram<Uncompiled> {
    pub fn new() -> Self {
        Self {
            id: unsafe { gl::CreateProgram() },
            shader_ids: Vec::new(),
            state: std::marker::PhantomData::<Uncompiled>,
        }
    }
    pub fn add_shader<'a, T: Into<&'a str>>(
        &mut self,
        path: T,
        shader_type: gl::types::GLenum,
    ) -> u32 {
        let source = std::fs::read_to_string(path.into()).unwrap();
        let source = std::ffi::CString::new(source.as_bytes()).unwrap();

        unsafe {
            let shader_id = gl::CreateShader(shader_type);
            gl::ShaderSource(
                shader_id,
                1,
                &source.as_ptr() as *const *const i8,
                std::ptr::null(),
            );

            gl::CompileShader(shader_id);

            let mut success = gl::FALSE as gl::types::GLint;
            gl::GetShaderiv(shader_id, gl::COMPILE_STATUS, &mut success);

            if success != gl::TRUE as gl::types::GLint {
                let mut len = 0;
                gl::GetShaderiv(shader_id, gl::INFO_LOG_LENGTH, &mut len);

                let error = std::ffi::CString::from_vec_unchecked(vec![b' '; len as usize]);

                gl::GetShaderInfoLog(
                    shader_id,
                    len,
                    std::ptr::null_mut(),
                    error.as_ptr() as *mut gl::types::GLchar,
                );

                panic!("Shader compilation failed: {}", error.to_string_lossy());
            }

            self.shader_ids.push(shader_id);

            shader_id
        }
    }

    pub fn compile(self) -> ShaderProgram<Compiled> {
        unsafe {
            for &shader_id in self.shader_ids.iter() {
                gl::AttachShader(self.id, shader_id);
            }

            gl::LinkProgram(self.id);

            for &shader_id in self.shader_ids.iter() {
                gl::DeleteShader(shader_id);
            }
        }

        ShaderProgram {
            id: self.id,
            shader_ids: self.shader_ids,
            state: std::marker::PhantomData::<Compiled>,
        }
    }
}

impl ShaderProgram<Compiled> {
    pub fn bind(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }
    pub fn unbind(&self) {
        unsafe {
            gl::UseProgram(0);
        }
    }
}
