use gl;
use render_gl::{self, data, buffer};
use failure;
use resources::Resources;

#[derive(VertexAttribPointers)]
#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
struct Vertex {
    #[location = "0"]
    pos: data::f32_f32_f32, // position
    #[location = "1"]
    clr: data::u2_u10_u10_u10_rev_float, // color
}

pub struct Triangle {
    program: render_gl::Program,
    _vbo: buffer::ArrayBuffer,
    vao: buffer::VertexArray,
}

impl Triangle {
    pub fn new(res: &Resources, gl: &gl::Gl) -> Result<Triangle, failure::Error> {

        let program = render_gl::Program::from_res(&gl, &res, "shaders/triangle")?;

        let vertices: Vec<Vertex> = vec![
            Vertex { pos: (-0.5, -0.5, 0.0).into(), clr: (1.0, 0.0, 0.0, 1.0).into() },
            Vertex { pos: (0.5, -0.5, 0.0).into(), clr: (0.0, 1.0, 0.0, 1.0).into() },
            Vertex { pos: (0.0, 0.5, 0.0).into(), clr: (0.0, 0.0, 1.0, 1.0).into() }
        ];

        let vbo = buffer::ArrayBuffer::new(&gl);
        vbo.bind();
        vbo.static_draw_data(&vertices);
        vbo.unbind();

        let vao = buffer::VertexArray::new(&gl);
        vao.bind();
        vbo.bind();
        Vertex::vertex_attrib_pointers(&gl);
        vbo.unbind();
        vao.unbind();

        Ok(Triangle {
            program,
            _vbo: vbo,
            vao
        })
    }

    pub fn render(&self, gl: &gl::Gl) {
        self.program.set_used();
        self.vao.bind();

        unsafe {
            gl.DrawArrays(
                gl::TRIANGLES,
                0,
                3
            );
        }
    }
}
