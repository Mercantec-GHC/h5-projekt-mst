use crate::engine::{game::Renderer, Color, Shape, Triangle2, Triangle3, V2, V3};

pub struct R3d<'r, R: Renderer> {
    r: &'r mut R,
    triangle_buffer: Vec<Triangle2>,
}

static CAMERA_POS: V3 = V3(0.0, 0.0, -1.0);

impl<'r, R: Renderer> R3d<'r, R> {
    pub fn new(r: &'r mut R) -> Self {
        Self {
            r,
            triangle_buffer: Vec::new(),
        }
    }

    pub fn draw_line(&mut self, from: V3, to: V3, color: Color) {
        self.r.draw_line(from.project_2d(), to.project_2d(), color);
    }

    pub fn draw_triangle(&mut self, triangle: Triangle3, color: Color) {
        let triangle = triangle.translate(CAMERA_POS).project_2d();

        self.r.draw_line(triangle.0, triangle.1, color);
        self.r.draw_line(triangle.1, triangle.2, color);
        self.r.draw_line(triangle.2, triangle.0, color);
    }

    pub fn draw_shape(&mut self, pos: V3, shape: &Shape, outline_color: Color, fill_color: Color) {
        self.triangle_buffer.clear();
        self.triangle_buffer
            .extend(shape.faces().map(|tri| tri.translate(pos).project_2d()));

        self.r.draw_triangles(&self.triangle_buffer, fill_color);

        for face in shape.faces() {
            if face.normal().dot(face.0 - CAMERA_POS + pos) >= 0.0 {
                continue;
            }
            // let pxyz = face.middle() + face.normal();
            // self.draw_line(
            //     face.middle() + pos,
            //     face.middle() + (face.1 - face.0).unit() * 0.02 + pos,
            //     Color::HEX(0xffaa00),
            // );
            // self.draw_line(
            //     face.middle() + pos,
            //     face.middle() + (face.2 - face.1).unit() * 0.02 + pos,
            //     Color::HEX(0x00ffaa),
            // );
            // self.draw_line(
            //     face.middle() + pos,
            //     face.middle() + (face.0 - face.2).unit() * 0.02 + pos,
            //     Color::HEX(0xaa00ff),
            // );
            // self.draw_line(face.middle() + pos, pxyz + pos, Color::HEX(0xffffff));

            // self.draw_triangle(face.translate(pos - CAMERA_POS), outline_color);
            self.r.draw_line(
                (face.0 + pos).project_2d(),
                (face.1 + pos).project_2d(),
                outline_color,
            );
            self.r.draw_line(
                (face.2 + pos).project_2d(),
                (face.0 + pos).project_2d(),
                outline_color,
            );
        }
    }
}
