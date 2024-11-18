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
