use crate::engine::{game::Renderer, Color, Shape, V2, V3};

#[derive(Clone, PartialEq, Debug)]
pub struct Triangle2(pub V2, pub V2, pub V2);

#[derive(Clone, PartialEq, Debug)]
pub struct Triangle3(pub V3, pub V3, pub V3);

impl Triangle3 {
    pub fn normal_vector(&self) -> V3 {
        (self.1 - self.0).cross(self.2 - self.1)
    }

    pub fn translate(&self, offset: V3) -> Self {
        Self(self.0 + offset, self.1 + offset, self.2 + offset)
    }

    pub fn project_2d(&self) -> Triangle2 {
        Triangle2(
            self.0.project_2d(),
            self.1.project_2d(),
            self.2.project_2d(),
        )
    }
}

pub struct R3d<'r, R: Renderer> {
    r: &'r mut R,
}

static CAMERA_POS: V3 = V3(0.0, 0.0, -1.0);

impl<'r, R: Renderer> R3d<'r, R> {
    pub fn new(r: &'r mut R) -> Self {
        Self { r }
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

    pub fn draw_cube(&mut self, pos: V3, size: V3, outline_color: Color, fill_color: Color) {
        let shape = Shape::new_cube(size);

        for face in shape.faces() {
            let normal_vector = face.normal_vector();
            let pxyz = face.0 + normal_vector;
            self.draw_line(face.0 + pos, pxyz + pos, Color::WED);

            let bingbongvector = face.0 - CAMERA_POS + pos;
            self.draw_line(face.0 + pos, V3(0.0, 0.0, -0.99), Color::WED);
            if normal_vector.dot(bingbongvector) < 0.0 {
                self.draw_triangle(face.translate(pos - CAMERA_POS), outline_color);
            }
        }

        for vertex in shape.vertices() {
            self.r
                .draw_point((vertex + pos).project_2d(), outline_color);
        }
    }
}
