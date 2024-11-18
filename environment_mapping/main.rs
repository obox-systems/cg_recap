use glfw::Context as _;
use nalgebra_glm as glm;

fn main() {
    let width = 1280;
    let height = 720;

    let mut glfw = glfw::init_no_callbacks().unwrap();

    let (mut window, _) = glfw
        .create_window(width, height, "", glfw::WindowMode::Windowed)
        .unwrap();

    gl::load_with(|symbol| window.get_proc_address(symbol));
    glfw.set_swap_interval(glfw::SwapInterval::Sync(1));

    let vertex_shader_source = include_str!("shaders/main.vert");
    let fragment_shader_source = include_str!("shaders/main.frag");
    let vertex = cg_recap::create_shader(gl::VERTEX_SHADER, vertex_shader_source).unwrap();
    let fragment = cg_recap::create_shader(gl::FRAGMENT_SHADER, fragment_shader_source).unwrap();

    let aspect = width as f32 / height as f32;
    let projection = glm::perspective_lh(aspect, 60.0_f32.to_radians(), 0.1, 1000.0);
    // let projection = glm::perspective(aspect, 60.0_f32.to_radians(), 0.1, 1000.0);

    let clip2world_loc = unsafe {
        let program = gl::CreateProgram();
        gl::AttachShader(program, vertex);
        gl::AttachShader(program, fragment);
        gl::LinkProgram(program);
        gl::UseProgram(program);

        let clip2world_name = std::ffi::CString::new("clip2world").unwrap();
        let clip2world_loc = gl::GetUniformLocation(program, clip2world_name.as_ptr());
        clip2world_loc
    };

    let images = [
        (
            image::open(r"environment_mapping\skybox\front.jpg").unwrap(),
            gl::TEXTURE_CUBE_MAP_POSITIVE_Z,
        ),
        (
            image::open(r"environment_mapping\skybox\back.jpg").unwrap(),
            gl::TEXTURE_CUBE_MAP_NEGATIVE_Z,
        ),
        (
            image::open(r"environment_mapping\skybox\left.jpg").unwrap(),
            gl::TEXTURE_CUBE_MAP_NEGATIVE_X,
        ),
        (
            image::open(r"environment_mapping\skybox\right.jpg").unwrap(),
            gl::TEXTURE_CUBE_MAP_POSITIVE_X,
        ),
        (
            image::open(r"environment_mapping\skybox\top.jpg").unwrap(),
            gl::TEXTURE_CUBE_MAP_POSITIVE_Y,
        ),
        (
            image::open(r"environment_mapping\skybox\bottom.jpg").unwrap(),
            gl::TEXTURE_CUBE_MAP_NEGATIVE_Y,
        ),
    ];

    unsafe {
        let mut cube_map = 0;
        gl::CreateTextures(gl::TEXTURE_CUBE_MAP, 1, &mut cube_map);
        gl::BindTexture(gl::TEXTURE_CUBE_MAP, cube_map);
        for (img, face) in images {
            gl::TexImage2D(
                face,
                0,
                gl::RGB as i32,
                img.width() as i32,
                img.height() as i32,
                0,
                gl::RGB,
                gl::UNSIGNED_BYTE,
                img.as_bytes().as_ptr().cast(),
            );
        }
        gl::GenerateMipmap(gl::TEXTURE_CUBE_MAP);
        gl::TexParameteri(
            gl::TEXTURE_CUBE_MAP,
            gl::TEXTURE_MIN_FILTER,
            gl::LINEAR_MIPMAP_LINEAR as i32,
        );
        gl::TexParameteri(
            gl::TEXTURE_CUBE_MAP,
            gl::TEXTURE_WRAP_S,
            gl::CLAMP_TO_EDGE as i32,
        );
        gl::TexParameteri(
            gl::TEXTURE_CUBE_MAP,
            gl::TEXTURE_WRAP_T,
            gl::CLAMP_TO_EDGE as i32,
        );
        gl::TexParameteri(
            gl::TEXTURE_CUBE_MAP,
            gl::TEXTURE_WRAP_R,
            gl::CLAMP_TO_EDGE as i32,
        );
    }

    cg_recap::enable(gl::TEXTURE_CUBE_MAP_SEAMLESS);
    cg_recap::enable(gl::DEPTH_TEST);
    
    while !window.should_close() {
        glfw.poll_events();

        let angle = glfw.get_time() as f32 * std::f32::consts::FRAC_PI_6;
        let rotation = glm::rotation(angle, &glm::Vec3::y_axis());
        let mvp = projection * rotation;
        let clip2world = mvp.try_inverse().unwrap();
        unsafe {
            gl::UniformMatrix4fv(clip2world_loc, 1, gl::FALSE, clip2world.as_ptr());
        }

        cg_recap::clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        cg_recap::draw_arrays(gl::TRIANGLES, 0, 3);
        window.swap_buffers();
    }
}
