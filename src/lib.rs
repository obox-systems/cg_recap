pub mod utils;

pub fn clear(mask: u32) {
    unsafe {
        gl::Clear(mask);
    }
}

pub fn compile_shader(r#type: u32, src: &str) -> Option<u32> {
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

pub fn draw_elements(mode: u32, count: i32, offset: usize, r#type: u32) {
    unsafe {
        gl::DrawElements(mode, count, r#type, offset as *const usize as *const _);
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

pub fn create_buffer() -> Option<u32> {
    unsafe {
        let mut buffer = 0;
        gl::CreateBuffers(1, &mut buffer);
        if buffer == 0 {
            None
        } else {
            Some(buffer)
        }
    }
}

pub fn bind_buffer(buffer: u32, target: u32) {
    unsafe {
        gl::BindBuffer(target, buffer);
    }
}

pub fn buffer_data<T: Copy>(target: u32, data: &[T], usage: u32) {
    unsafe {
        gl::BufferData(
            target,
            size_of_val(data) as isize,
            data.as_ptr().cast(),
            usage,
        );
    }
}

pub fn create_vao() -> Option<u32> {
    unsafe {
        let mut vao = 0;
        gl::CreateVertexArrays(1, &mut vao);
        if vao == 0 {
            None
        } else {
            Some(vao)
        }
    }
}

pub fn bind_vao(vao: u32) {
    unsafe {
        gl::BindVertexArray(vao);
    }
}

pub fn create_texture(target: u32) -> Option<u32> {
    unsafe {
        let mut tex = 0;
        gl::CreateTextures(target, 1, &mut tex);

        if tex == 0 {
            None
        } else {
            Some(tex)
        }
    }
}

pub fn bind_texture(texture: u32, target: u32) {
    unsafe {
        gl::BindTexture(target, texture);
    }
}

pub fn viewport(x: i32, y: i32, width: i32, height: i32) {
    unsafe {
        gl::Viewport(x, y, width, height);
    }
}

pub fn create_framebuffer() -> Option<u32> {
    unsafe {
        let mut framebuffer = 0;
        gl::CreateFramebuffers(1, &mut framebuffer);

        if framebuffer == 0 {
            None
        } else {
            Some(framebuffer)
        }
    }
}

pub fn bind_framebuffer(framebuffer: u32, target: u32) {
    unsafe {
        gl::BindFramebuffer(target, framebuffer);
    }
}

pub fn framebuffer_renderbuffer(
    target: u32,
    attachment: u32,
    renderbuffertarget: u32,
    renderbuffer: u32,
) {
    unsafe {
        gl::FramebufferRenderbuffer(target, attachment, renderbuffertarget, renderbuffer);
    }
}

pub fn framebuffer_texture2d(
    target: u32,
    attachment: u32,
    textarget: u32,
    texture: u32,
    level: i32,
) {
    unsafe {
        gl::FramebufferTexture2D(target, attachment, textarget, texture, level);
    }
}

pub fn draw_buffer(buffer: u32) {
    unsafe {
        gl::DrawBuffer(buffer);
    }
}

pub fn draw_buffers(buffers: &[u32]) {
    unsafe {
        gl::DrawBuffers(buffers.len() as i32, buffers.as_ptr());
    }
}

pub fn texture_storage2d(target: u32, levels: i32, format: u32, width: i32, height: i32) {
    unsafe {
        gl::TexStorage2D(target, levels, format, width, height);
    }
}

pub fn texture_image2d<T: Copy>(
    target: u32,
    level: i32,
    internalformat: u32,
    width: i32,
    height: i32,
    format: u32,
    r#type: u32,
    pixels: &[T],
) {
    unsafe {
        gl::TexImage2D(
            target,
            level,
            internalformat as i32,
            width,
            height,
            0,
            format,
            r#type,
            pixels.as_ptr().cast(),
        );
    }
}

pub fn texture_parameter(target: u32, name: u32, param: u32) {
    unsafe {
        gl::TexParameteri(target, name, param as i32);
    }
}

pub fn create_renderbuffer() -> Option<u32> {
    unsafe {
        let mut renderbuffer = 0;
        gl::CreateRenderbuffers(1, &mut renderbuffer);

        if renderbuffer == 0 {
            None
        } else {
            Some(renderbuffer)
        }
    }
}

pub fn bind_renderbuffer(renderbuffer: u32, target: u32) {
    unsafe {
        gl::BindRenderbuffer(target, renderbuffer);
    }
}

pub fn renderbuffer_storage(target: u32, internalformat: u32, width: i32, height: i32) {
    unsafe {
        gl::RenderbufferStorage(target, internalformat, width, height);
    }
}

pub fn generate_mipmaps(target: u32) {
    unsafe {
        gl::GenerateMipmap(target);
    }
}
