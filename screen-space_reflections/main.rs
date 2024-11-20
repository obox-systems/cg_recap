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

    let icosphere_mesh = cg_recap::utils::load_mesh(r"screen-space_reflections\assets\cube.glb");

    let vertex_shader_source = include_str!(r"shaders\main.vert");
    let fragment_shader_source = include_str!(r"shaders\main.frag");
    let vert = cg_recap::create_shader(gl::VERTEX_SHADER, vertex_shader_source).unwrap();
    let frag = cg_recap::create_shader(gl::FRAGMENT_SHADER, fragment_shader_source).unwrap();
    let object_shader = cg_recap::create_vert_frag_prog(vert, frag).unwrap();
    let mvp_location = cg_recap::get_location(object_shader, "mvp").unwrap();
    let model_location = cg_recap::get_location(object_shader, "model").unwrap();
    let eye_location = cg_recap::get_location(object_shader, "eye").unwrap();

    let vertex_shader_source = include_str!(r"..\environment_mapping\shaders\main.vert");
    let fragment_shader_source = include_str!(r"..\environment_mapping\shaders\main.frag");
    let vert = cg_recap::create_shader(gl::VERTEX_SHADER, vertex_shader_source).unwrap();
    let frag = cg_recap::create_shader(gl::FRAGMENT_SHADER, fragment_shader_source).unwrap();
    let background_shader = cg_recap::create_vert_frag_prog(vert, frag).unwrap();
    let clip2world_location = cg_recap::get_location(background_shader, "clip2world").unwrap();

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
    }

    cg_recap::enable(gl::TEXTURE_CUBE_MAP_SEAMLESS);
    cg_recap::enable(gl::DEPTH_TEST);

    while !window.should_close() {
        glfw.poll_events();

        let angle = glfw.get_time() as f32 * std::f32::consts::FRAC_PI_8;
        let model = glm::translation(&glm::vec3(0.0, -5.0, 0.0))
            * glm::rotation(angle, &glm::Vec3::x_axis());
        let view = glm::rotation(std::f32::consts::FRAC_PI_2, &glm::Vec3::x_axis());
        let mvp = projection * view * model;
        let clip2world = (projection * view).try_inverse().unwrap();

        cg_recap::clear(gl::DEPTH_BUFFER_BIT);

        cg_recap::use_program(object_shader);
        unsafe {
            gl::UniformMatrix4fv(mvp_location, 1, 0, mvp.as_ptr());
            gl::UniformMatrix4fv(model_location, 1, 0, model.as_ptr());
            gl::Uniform3f(eye_location, 0.0, 0.0, 0.0);
        }
        cg_recap::utils::draw_mesh(&icosphere_mesh);

        cg_recap::use_program(background_shader);
        unsafe {
            gl::UniformMatrix4fv(clip2world_location, 1, 0, clip2world.as_ptr());
        }
        cg_recap::draw_arrays(gl::TRIANGLES, 0, 3);

        window.swap_buffers();
    }
}
