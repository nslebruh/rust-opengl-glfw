use std::{mem::size_of, ffi::c_void};

use glm::{Vec3, Vec2};


#[derive(Debug)]
pub struct Vertex {
    pub position: Vec3,
    pub normal: Vec3,
    pub tex_coords: Vec2
}

#[derive(Debug)]
pub struct Texture {

}

#[derive(Debug, Default)]
pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
    pub textures: Vec<Texture>,
    vao: u32,
    ebo: u32,
    vbo: u32
}

impl Mesh {
    pub fn new(vertices: Vec<Vertex>, indices: Vec<u32>, textures: Vec<Texture>) -> Self {
        let vao: u32;
        let vbo: u32;
        let ebo: u32;
        unsafe {
            (vao, vbo, ebo) = Mesh::setup_mesh(&vertices, &indices, &textures);
        }
        Self {
            vertices,
            indices,
            textures,
            vao,
            vbo,
            ebo
        }
    }

    pub fn vert_to_f32_vec(&self) -> Vec<f32> {
        let mut output: Vec<f32> = Vec::with_capacity(self.vertices.len() * 8);
        for vertex in self.vertices.iter() {
            output.push(vertex.position.x);
            output.push(vertex.position.y);
            output.push(vertex.position.z);
            output.push(vertex.normal.x);
            output.push(vertex.normal.y);
            output.push(vertex.normal.z);
            output.push(vertex.tex_coords.x);
            output.push(vertex.tex_coords.y);
        };
        output
    }

    unsafe fn setup_mesh(vertices: &Vec<Vertex>, indices: &Vec<u32>, textures: &Vec<Texture>) -> (u32, u32, u32) {
        let mut vao: u32 = 0;
        let mut vbo: u32 = 0;
        let mut ebo: u32 = 0;
        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1, &mut vbo);
        gl::GenBuffers(1, &mut ebo);

        gl::BindVertexArray(vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * size_of::<f32>()) as isize,
            vertices.as_ptr().cast(),
            gl::STATIC_DRAW
        );

        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
        gl::BufferData(
            gl::ELEMENT_ARRAY_BUFFER,
            (indices.len() * size_of::<u32>()) as isize,
            indices.as_ptr().cast(),
            gl::STATIC_DRAW
        );

        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(
            0,
            3, 
            gl::FLOAT,
            gl::FALSE,
            (size_of::<f32>() * 8) as i32,
            std::ptr::null()
        );

        gl::EnableVertexAttribArray(1);
        gl::VertexAttribPointer(
            1,
            3,
            gl::FLOAT,
            gl::FALSE,
            (size_of::<f32>() * 8) as i32,
            (3 * size_of::<f32>()) as *const c_void
        );

        gl::EnableVertexAttribArray(2);
        gl::VertexAttribPointer(
            2,
            2,
            gl::FLOAT,
            gl::FALSE,
            (size_of::<f32>() * 8) as i32,
            (2 * size_of::<f32>()) as *const c_void
        );

        gl::BindVertexArray(0);


        (vao, vbo, ebo)
    }
}