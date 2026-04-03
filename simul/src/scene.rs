use crate::{
    tri2::Tri2,
    tri3::Tri3,
    v3::V3,
    window::{Color, Renderer},
};

pub struct Scene {
    tris: Vec<(Tri3, Color, Color)>,
}

impl Scene {
    pub fn new() -> Self {
        Self { tris: Vec::new() }
    }

    pub fn render(&self, r: &mut impl Renderer, camera_pos: V3) {
        let mut indices_with_scores = self
            .tris
            .iter()
            .enumerate()
            .map(|(i, (tri, ..))| {
                let mut point_scores = tri.points().map(|p| (camera_pos - p).len());
                point_scores.sort_by(|a, b| a.total_cmp(b));

                (i, point_scores[0])
            })
            .rev()
            .collect::<Vec<_>>();

        indices_with_scores.sort_by(|a, b| b.1.total_cmp(&a.1));

        for (i, _) in indices_with_scores {
            let (tri3, outline_color, fill_color) = &self.tris[i];

            // if !(tri3.0.2 >= -1.0 && tri3.1.2 >= -1.0 && tri3.2.2 >= -1.0) {
            //     continue;
            // }

            // if tri3.normal().dot(tri3.0 - camera_pos) >= 0.0 {
            //     continue;
            // }
            //
            println!("{tri3:?}");

            let tri2 = tri3.project_2d(camera_pos);

            println!("{tri2:?}");

            r.draw_triangles(&[tri2], *fill_color);
            r.draw_line(tri2.0, tri2.1, *outline_color);
            r.draw_line(tri2.0, tri2.2, *outline_color);
        }
    }

    pub fn draw_triangle(&mut self, tri: Tri3, outline_color: Color, fill_color: Color) {
        self.tris.push((tri, outline_color, fill_color));
    }
}
