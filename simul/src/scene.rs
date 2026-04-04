use obj::Obj;

use crate::{
    m3x3::M3x3,
    tri2::Tri2,
    tri3::Tri3,
    v3::V3,
    window::{Color, Renderer},
};

pub struct Scene {
    tris: Vec<(Tri3, V3, Color)>,
}

impl Scene {
    pub fn new() -> Self {
        Self { tris: Vec::new() }
    }

    pub fn render(
        &self,
        r: &mut impl Renderer,
        camera_pos: V3,
        camera_rot: M3x3,
        screen_rel_pos: V3,
    ) {
        let mut indices_with_scores = self
            .tris
            .iter()
            .enumerate()
            .map(|(i, (tri, ..))| {
                let mut score = (camera_pos - tri.middle()).len();
                (i, score)
            })
            .rev()
            .collect::<Vec<_>>();

        indices_with_scores.sort_by(|a, b| b.1.total_cmp(&a.1));

        for (i, _) in indices_with_scores {
            let (tri3, normal, color) = &self.tris[i];

            // if !(tri3.0.2 >= -1.0 && tri3.1.2 >= -1.0 && tri3.2.2 >= -1.0) {
            //     continue;
            // }

            if normal.dot(camera_pos - tri3.0) < 0.0 {
                continue;
            }

            let tri2 = tri3.project_2d(camera_pos, camera_rot, screen_rel_pos);

            r.draw_triangles(&[tri2], *color);
            r.draw_line(tri2.0, tri2.1, Color::BLACK);
            r.draw_line(tri2.1, tri2.2, Color::BLACK);
            r.draw_line(tri2.2, tri2.0, Color::BLACK);
        }
    }

    pub fn draw_model(&mut self, model: Model) {
        self.tris.extend(model.tris());
    }

    pub fn draw_triangle(&mut self, tri: Tri3, color: Color) {
        self.tris.push((tri, tri.normal(), color));
    }
}

pub struct Model {
    tris: Vec<(Tri3, V3, Color)>,
}

impl Model {
    pub fn new() -> Self {
        Self { tris: Vec::new() }
    }

    pub fn tris(&self) -> impl Iterator<Item = (Tri3, V3, Color)> {
        self.tris.iter().copied()
    }

    pub fn translate(&mut self, offset: V3) -> &mut Self {
        for tri in &mut self.tris {
            tri.0 = tri.0.translate(offset);
        }
        self
    }

    pub fn rotate_by_m3x3(&mut self, rot: M3x3) -> &mut Self {
        for tri in &mut self.tris {
            tri.0 = tri.0.rotate_by_m3x3(rot);
            tri.1 = tri.1.rotate_by_m3x3(rot);
        }
        self
    }

    pub fn scale(&mut self, scale: V3) -> &mut Self {
        for tri in &mut self.tris {
            tri.0 = tri.0.scale(scale);
        }
        self
    }

    pub fn add_tri(&mut self, tri: Tri3, color: Color) -> &mut Self {
        self.tris.push((tri, tri.normal(), color));
        self
    }

    pub fn add_obj(&mut self, obj: &Obj, color: Color) -> &mut Self {
        // if our triangle definition convention is [0, 1, 2]
        // obj's is [2, 1, 0].
        //
        // likewise, if our coordinate system is [x, y, z],
        // obj's is [x, z, y].

        let count = obj.indices.len() / 3;
        for i in 0..count {
            let v2 = obj.vertices[obj.indices[i * 3] as usize];
            let v1 = obj.vertices[obj.indices[i * 3 + 1] as usize];
            let v0 = obj.vertices[obj.indices[i * 3 + 2] as usize];

            let tri = Tri3(
                V3(
                    v0.position[0] as _,
                    v0.position[2] as _,
                    v0.position[1] as _,
                ),
                V3(
                    v1.position[0] as _,
                    v1.position[2] as _,
                    v1.position[1] as _,
                ),
                V3(
                    v2.position[0] as _,
                    v2.position[2] as _,
                    v2.position[1] as _,
                ),
            );

            let normal = V3(v0.normal[0] as _, v0.normal[2] as _, v0.normal[1] as _)
                + V3(v1.normal[0] as _, v1.normal[2] as _, v1.normal[1] as _)
                + V3(v2.normal[0] as _, v2.normal[2] as _, v2.normal[1] as _);

            self.tris.push((tri, normal, color));
        }
        self
    }
}
