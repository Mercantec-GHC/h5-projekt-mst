use std::f64::consts::PI;

use crate::engine::{Color, Renderer, Shape, Triangle3, V3};

pub struct Scene {
    tris: Vec<(Triangle3, V3, Color, Color)>,
}

impl Scene {
    pub fn new() -> Self {
        Self { tris: Vec::new() }
    }

    pub fn render<R: Renderer>(&mut self, r: &mut R, camera_pos: V3) {
        let mut indices_with_scores = self
            .tris
            .iter()
            .enumerate()
            .map(|(i, (tri, ..))| {
                let mut p_scores = [tri.0, tri.1, tri.2].map(|p| (camera_pos - p).len());

                p_scores.sort_by(|a, b| a.total_cmp(b));

                let score = p_scores[0] * p_scores[1];
                (i, score)
            })
            .rev()
            .collect::<Vec<_>>();

        indices_with_scores.sort_by(|a, b| b.1.total_cmp(&a.1));

        'render_tri_loop: for (i, _) in indices_with_scores {
            let (tri3, normal, outline_color, fill_color) = &self.tris[i];

            for v in [tri3.0, tri3.1, tri3.2] {
                if v.2 < camera_pos.2 {
                    continue 'render_tri_loop;
                }
            }

            if normal.dot(camera_pos - tri3.0) < 0.0 {
                continue;
            }

            let tri2 = tri3.project_2d(camera_pos);

            r.draw_triangle(tri2.clone(), *fill_color);
            r.draw_line(tri2.0, tri2.1, *outline_color);
            r.draw_line(tri2.0, tri2.2, *outline_color);
        }
    }

    pub fn draw_shape(&mut self, pos: V3, shape: &Shape, outline_color: Color, fill_color: Color) {
        for triangle in shape.faces() {
            let tri = triangle.translate(pos);
            let normal = tri.normal();
            self.tris.push((tri, normal, outline_color, fill_color))
        }
    }
}
