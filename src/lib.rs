pub fn clear(mask: u32) {
    unsafe {
        gl::Clear(mask);
    }
}

pub fn create_shader(r#type: u32, src: &str) -> Option<u32> {
    unsafe {
        let shader = gl::CreateShader(r#type);

        if shader == 0 {
            return None;
        }

        gl::ShaderSource(
            shader,
            1,
            &src.as_bytes().as_ptr().cast(),
            &(src.as_bytes().len() as i32),
        );
        gl::CompileShader(shader);

        let mut is_compiled = 0;
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut is_compiled);

        if is_compiled == 0 {
            None
        } else {
            Some(shader)
        }
    }
}

pub fn draw_arrays(mode: u32, first: i32, count: i32) {
    unsafe {
        gl::DrawArrays(mode, first, count);
    }
}

pub fn enable(cap: u32) {
    unsafe {
        gl::Enable(cap);
    }
}

pub fn create_vert_frag_prog(vert: u32, frag: u32) -> Option<u32> {
    unsafe {
        let program = gl::CreateProgram();
        gl::AttachShader(program, vert);
        gl::AttachShader(program, frag);
        gl::LinkProgram(program);

        let mut is_linked = 0;
        gl::GetProgramiv(program, gl::LINK_STATUS, &mut is_linked);

        if is_linked == 0 {
            None
        } else {
            Some(program)
        }
    }
}

pub fn use_program(program: u32) {
    unsafe {
        gl::UseProgram(program);
    }
}

pub fn get_location(program: u32, uniform: &str) -> Option<i32> {
    let uniform = std::ffi::CString::new(uniform).ok()?;
    unsafe {
        let location = gl::GetUniformLocation(program, uniform.as_ptr());

        if location == -1 {
            None
        } else {
            Some(location)
        }
    }
}
