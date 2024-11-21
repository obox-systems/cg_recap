use russimp::scene::{PostProcess, Scene};

pub fn load_mesh(path: &str) -> Vec<(u32, i32)> {
    let postprocess = [
        PostProcess::Triangulate,
        PostProcess::OptimizeMeshes,
        PostProcess::OptimizeGraph,
    ];

    let scene = Scene::from_file(path, postprocess.into()).unwrap();
    let mut meshes = vec![];
    for mesh in scene.meshes {
        let vao = crate::create_vao().unwrap();
        crate::bind_vao(vao);

        let position_buffer = crate::create_buffer().unwrap();
        crate::bind_buffer(position_buffer, gl::ARRAY_BUFFER);
        crate::buffer_data(gl::ARRAY_BUFFER, &mesh.vertices, gl::STATIC_DRAW);

        unsafe {
            gl::VertexAttribPointer(0, 3, gl::FLOAT, 0, 0, std::ptr::null());
            gl::EnableVertexAttribArray(0);
        }

        let normal_buffer = crate::create_buffer().unwrap();
        crate::bind_buffer(normal_buffer, gl::ARRAY_BUFFER);
        crate::buffer_data(gl::ARRAY_BUFFER, &mesh.normals, gl::STATIC_DRAW);

        unsafe {
            gl::VertexAttribPointer(1, 3, gl::FLOAT, 0, 0, std::ptr::null());
            gl::EnableVertexAttribArray(1);
        }

        if let Some(texcoords) = mesh.texture_coords.first() {
            if let Some(texcoords) = texcoords {
                let texcoords: Box<[_]> = texcoords
                    .into_iter()
                    .map(|item| russimp::Vector2D {
                        x: item.x,
                        y: item.y,
                    })
                    .collect();

                let texcoord_buffer = crate::create_buffer().unwrap();
                crate::bind_buffer(texcoord_buffer, gl::ARRAY_BUFFER);
                crate::buffer_data(gl::ARRAY_BUFFER, &texcoords, gl::STATIC_DRAW);

                unsafe {
                    gl::VertexAttribPointer(2, 2, gl::FLOAT, 0, 0, std::ptr::null());
                    gl::EnableVertexAttribArray(2);
                }
            }
        }

        let mut indices = vec![];
        for mut face in mesh.faces {
            indices.append(&mut face.0);
        }

        let index_buffer = crate::create_buffer().unwrap();
        crate::bind_buffer(index_buffer, gl::ELEMENT_ARRAY_BUFFER);
        crate::buffer_data(gl::ELEMENT_ARRAY_BUFFER, &indices, gl::STATIC_DRAW);

        let count = indices.len() as i32;

        meshes.push((vao, count));
    }
    meshes
}

pub fn draw_mesh(mesh: &[(u32, i32)]) {
    for (vao, count) in mesh {
        crate::bind_vao(*vao);
        crate::draw_elements(gl::TRIANGLES, *count, 0, gl::UNSIGNED_INT);
    }
}
