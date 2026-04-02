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

    pub fn render<R: Renderer>(&mut self, r: &mut R, camera_pos: V3, camera_rot: V3) {
        let mut indices_with_scores = self
            .objects
            .iter()
            .enumerate()
            .map(|(i, object)| {
                let mut point_scores = object.triangle.points().map(|p| p.distance(camera_pos));
                point_scores.sort_by(|a, b| a.total_cmp(b));

                (i, point_scores[0])
            })
            .rev()
            .collect::<Vec<_>>();

        indices_with_scores.sort_by(|a, b| b.1.total_cmp(&a.1));

        for (i, _) in indices_with_scores {
            let object = &self.objects[i];
            let tri3 = object
                .triangle
                .translate(camera_pos * -1.0)
                .rotate(camera_rot);

            // check if behind camera
            // TODO: doesn't take camera_pos or camera_rot into accout
            if !(tri3.0 .2 >= 0.0 && tri3.1 .2 >= 0.0 && tri3.2 .2 >= 0.0) {
                continue;
            }

            if tri3.normal().dot(tri3.0 - camera_pos) >= 0.0 {
                continue;
            }

            let tri2 = tri3
                .translate(V3(0.0, 0.0, -1.0))
                .project_2d(V3(0.0, 0.0, -1.0));

            r.draw_triangle(tri2.clone(), object.fill_color);
            r.draw_line(tri2.0, tri2.1, object.outline_color);
            r.draw_line(tri2.0, tri2.2, object.outline_color);
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
