use glfw::Context as _;
use nalgebra_glm as glm;

fn main() {
    let width = 1280;
    let height = 720;

    let mut glfw = glfw::init_no_callbacks().unwrap();
    glfw.window_hint(glfw::WindowHint::Resizable(false));

    let (mut window, _) = glfw
        .create_window(width, height, "", glfw::WindowMode::Windowed)
        .unwrap();

    gl::load_with(|symbol| window.get_proc_address(symbol));
    glfw.set_swap_interval(glfw::SwapInterval::Sync(1));

    let cube_mesh = cg_recap::utils::load_mesh(r"cube-map_reflections\assets\cube.glb");
    let icosphere_mesh = cg_recap::utils::load_mesh(r"cube-map_reflections\assets\icosphere.glb");

    let reflection_shader = ReflectionShader::new();
    let background_shader = BackgroundShader::new();
    let object_shader = ObjectShader::new();

    let aspect = width as f32 / height as f32;
    let projection = glm::perspective(aspect, 45.0_f32.to_radians(), 0.1, 1000.0);

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

    let env_map = cg_recap::create_texture(gl::TEXTURE_CUBE_MAP).unwrap();
    cg_recap::bind_texture(env_map, gl::TEXTURE_CUBE_MAP);
    for (img, face) in images {
        cg_recap::texture_image2d(
            face,
            0,
            gl::RGB,
            img.width() as i32,
            img.height() as i32,
            gl::RGB,
            gl::UNSIGNED_BYTE,
            img.as_bytes(),
        );
    }
    cg_recap::generate_mipmaps(gl::TEXTURE_CUBE_MAP);
    cg_recap::texture_parameter(
        gl::TEXTURE_CUBE_MAP,
        gl::TEXTURE_MIN_FILTER,
        gl::LINEAR_MIPMAP_LINEAR,
    );

    let res = 1024;

    let reflection_map = cg_recap::create_texture(gl::TEXTURE_CUBE_MAP).unwrap();
    cg_recap::bind_texture(reflection_map, gl::TEXTURE_CUBE_MAP);
    cg_recap::texture_parameter(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_MIN_FILTER, gl::LINEAR);
    for target in gl::TEXTURE_CUBE_MAP_POSITIVE_X..=gl::TEXTURE_CUBE_MAP_NEGATIVE_Z {
        cg_recap::texture_storage2d(target, 1, gl::RGBA8, res, res);
    }

    let renderbuffer = cg_recap::create_renderbuffer().unwrap();
    cg_recap::bind_renderbuffer(renderbuffer, gl::RENDERBUFFER);
    cg_recap::renderbuffer_storage(gl::RENDERBUFFER, gl::DEPTH_COMPONENT24, res, res);

    let framebuffer = cg_recap::create_framebuffer().unwrap();
    println!("{framebuffer}");
    cg_recap::bind_framebuffer(framebuffer, gl::FRAMEBUFFER);
    for target in gl::TEXTURE_CUBE_MAP_POSITIVE_X..=gl::TEXTURE_CUBE_MAP_NEGATIVE_Z {
        let attachment = gl::COLOR_ATTACHMENT0 + (target - gl::TEXTURE_CUBE_MAP_POSITIVE_X);
        cg_recap::framebuffer_texture2d(gl::FRAMEBUFFER, attachment, target, reflection_map, 0);
    }
    cg_recap::framebuffer_renderbuffer(
        gl::FRAMEBUFFER,
        gl::DEPTH_ATTACHMENT,
        gl::RENDERBUFFER,
        renderbuffer,
    );

    cg_recap::enable(gl::TEXTURE_CUBE_MAP_SEAMLESS);
    cg_recap::enable(gl::DEPTH_TEST);

    while !window.should_close() {
        glfw.poll_events();
        let time = glfw.get_time() as f32;
        let view = glm::rotation(time, &glm::Vec3::y_axis());
        let clip2world = glm::inverse(&(projection * view));

        // draw reflection map
        cg_recap::bind_texture(env_map, gl::TEXTURE_CUBE_MAP);
        cg_recap::bind_framebuffer(framebuffer, gl::FRAMEBUFFER);
        cg_recap::draw_buffers(&[
            gl::COLOR_ATTACHMENT0,
            gl::COLOR_ATTACHMENT1,
            gl::COLOR_ATTACHMENT2,
            gl::COLOR_ATTACHMENT3,
            gl::COLOR_ATTACHMENT4,
            gl::COLOR_ATTACHMENT5,
        ]);
        cg_recap::clear(gl::COLOR_BUFFER_BIT);
        let cubemap_projection = glm::perspective(1.0, 90_f32.to_radians(), 0.1, 100.0);
        // let rotations = [
        //     glm::rotation(angle, v)
        // ];
        for target in gl::TEXTURE_CUBE_MAP_POSITIVE_X..=gl::TEXTURE_CUBE_MAP_NEGATIVE_Z {
            let attachment = gl::COLOR_ATTACHMENT0 + (target - gl::TEXTURE_CUBE_MAP_POSITIVE_X);
            cg_recap::clear(gl::DEPTH_BUFFER_BIT);
            cg_recap::draw_buffer(attachment);

            // gl::FramebufferTexture2D(gl::FRAMEBUFFER, attachment, target, reflection_map, 0);
        }

        cg_recap::bind_framebuffer(0, gl::FRAMEBUFFER);
        cg_recap::clear(gl::DEPTH_BUFFER_BIT);

        // draw reflective object
        cg_recap::bind_texture(reflection_map, gl::TEXTURE_CUBE_MAP);
        cg_recap::use_program(reflection_shader.program);
        let angle = time * std::f32::consts::PI / 15.0;
        let angle = 0.0;
        let model = glm::translation(&glm::vec3(0.0, -0.5, -10.0))
            * glm::rotation(angle, &glm::Vec3::y_axis());
        let mvp = projection * view * model;
        unsafe {
            gl::UniformMatrix4fv(reflection_shader.mvp_location, 1, 0, mvp.as_ptr());
            gl::UniformMatrix4fv(reflection_shader.model_location, 1, 0, model.as_ptr());
            gl::Uniform3f(reflection_shader.eye_location, 0.0, 0.0, 0.0);
        }
        cg_recap::utils::draw_mesh(&cube_mesh);

        // draw common objects
        cg_recap::use_program(object_shader.program);
        let transforms = transforms(time);
        for t in &transforms[..1] {
            let mvp = projection * view * t;
            unsafe {
                gl::UniformMatrix4fv(reflection_shader.mvp_location, 1, 0, mvp.as_ptr());
                gl::UniformMatrix4fv(reflection_shader.model_location, 1, 0, t.as_ptr());
            }
            cg_recap::utils::draw_mesh(&icosphere_mesh);
        }

        // draw backgroung
        cg_recap::bind_texture(env_map, gl::TEXTURE_CUBE_MAP);
        cg_recap::use_program(background_shader.program);
        unsafe {
            gl::UniformMatrix4fv(
                background_shader.clip2world_location,
                1,
                0,
                clip2world.as_ptr(),
            );
        }
        cg_recap::draw_arrays(gl::TRIANGLES, 0, 3);

        window.swap_buffers();
    }
}

fn transforms(time: f32) -> Vec<glm::Mat4> {
    let scale = glm::scaling(&glm::Vec3::from_element(0.5));
    let num = 5;
    let mut ret = vec![];
    let center = glm::vec3(0.0, -1.0, -10.0);
    for i in 0..num {
        let angle =
            time * std::f32::consts::FRAC_PI_3 + std::f32::consts::PI * 2.0 / num as f32 * i as f32;
        let x = angle.sin() * 3.0;
        let z = angle.cos() * 3.0;

        let position = center + glm::vec3(x, 0.0, z);
        ret.push(glm::translation(&position) * scale);
    }
    ret
}

pub struct ReflectionShader {
    pub program: u32,
    pub mvp_location: i32,
    pub model_location: i32,
    pub eye_location: i32,
}

impl ReflectionShader {
    pub fn new() -> Self {
        let vertex_shader_source = include_str!(r"shaders\main.vert");
        let fragment_shader_source = include_str!(r"shaders\reflection.frag");
        let vert = cg_recap::compile_shader(gl::VERTEX_SHADER, vertex_shader_source).unwrap();
        let frag = cg_recap::compile_shader(gl::FRAGMENT_SHADER, fragment_shader_source).unwrap();
        let program = cg_recap::create_vert_frag_prog(vert, frag).unwrap();
        let mvp_location = cg_recap::get_location(program, "mvp").unwrap();
        let model_location = cg_recap::get_location(program, "model").unwrap();
        let eye_location = cg_recap::get_location(program, "eye").unwrap();

        Self {
            program,
            mvp_location,
            model_location,
            eye_location,
        }
    }
}

pub struct BackgroundShader {
    pub program: u32,
    pub clip2world_location: i32,
}

impl BackgroundShader {
    pub fn new() -> Self {
        let vertex_shader_source = include_str!(r"..\environment_mapping\shaders\main.vert");
        let fragment_shader_source = include_str!(r"..\environment_mapping\shaders\main.frag");
        let vert = cg_recap::compile_shader(gl::VERTEX_SHADER, vertex_shader_source).unwrap();
        let frag = cg_recap::compile_shader(gl::FRAGMENT_SHADER, fragment_shader_source).unwrap();
        let program = cg_recap::create_vert_frag_prog(vert, frag).unwrap();
        let clip2world_location = cg_recap::get_location(program, "clip2world").unwrap();

        Self {
            program,
            clip2world_location,
        }
    }
}

pub struct ObjectShader {
    pub program: u32,
    pub mvp_location: i32,
    pub model_location: i32,
}

impl ObjectShader {
    pub fn new() -> Self {
        let vertex_shader_source = include_str!(r"shaders\main.vert");
        let fragment_shader_source = include_str!(r"shaders\main.frag");
        let vert = cg_recap::compile_shader(gl::VERTEX_SHADER, vertex_shader_source).unwrap();
        let frag = cg_recap::compile_shader(gl::FRAGMENT_SHADER, fragment_shader_source).unwrap();
        let program = cg_recap::create_vert_frag_prog(vert, frag).unwrap();
        let mvp_location = cg_recap::get_location(program, "mvp").unwrap();
        let model_location = cg_recap::get_location(program, "model").unwrap();

        Self {
            program,
            mvp_location,
            model_location,
        }
    }
}
