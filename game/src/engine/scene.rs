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

    pub fn render<R: Renderer>(&mut self, r: &mut R, camera_pos: V3) {
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

            // check if behind camera
            if !(object.triangle.0 .2 >= -1.0
                && object.triangle.1 .2 >= -1.0
                && object.triangle.2 .2 >= -1.0)
            {
                continue;
            }

            if object.triangle.normal().dot(object.triangle.0 - camera_pos) >= 0.0 {
                continue;
            }

            let triangle = object.triangle.project_2d(camera_pos);

            r.draw_triangle(triangle.clone(), object.fill_color);
            r.draw_line(triangle.0, triangle.1, object.outline_color);
            r.draw_line(triangle.0, triangle.2, object.outline_color);
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
