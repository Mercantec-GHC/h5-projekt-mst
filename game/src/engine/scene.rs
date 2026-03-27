use crate::engine::{Color, Renderer, Shape, Triangle3, V3};

pub struct DrawnTriangle {
    triangle: Triangle3,
    outline_color: Color,
    fill_color: Color,
}

pub struct Scene {
    objects: Vec<DrawnTriangle>,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn render<R: Renderer>(&mut self, r: &mut R, camera: V3) {
        self.objects.sort_by(|a, b| {
            let a = a
                .triangle
                .points()
                .map(|p| p.distance(camera))
                .into_iter()
                .max_by(|a, b| a.total_cmp(b))
                .unwrap();
            let b = b
                .triangle
                .points()
                .map(|p| p.distance(camera))
                .into_iter()
                .max_by(|a, b| a.total_cmp(b))
                .unwrap();
            b.total_cmp(&a)
        });

        let drawn_triangles = self
            .objects
            .iter()
            .map(|v| (v.triangle.project_2d(), v.outline_color, v.fill_color));
        for (triangle, outline_color, fill_color) in drawn_triangles {
            r.draw_triangle(triangle.clone(), fill_color);
            r.draw_line(triangle.0, triangle.1, outline_color);
            r.draw_line(triangle.0, triangle.2, outline_color);
        }
    }

    pub fn draw_shape(&mut self, pos: V3, shape: &Shape, outline_color: Color, fill_color: Color) {
        for triangle in shape.faces() {
            self.objects.push(DrawnTriangle {
                triangle: triangle.translate(pos),
                outline_color,
                fill_color,
            })
        }
    }
}
