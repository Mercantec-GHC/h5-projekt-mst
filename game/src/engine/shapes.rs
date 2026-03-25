use crate::engine::math::{Vertex, V3};

pub struct Shape {
    vertices: Vec<V3>,
    fragments: Vec<(usize, usize, usize)>,
}

impl Shape {
    pub fn new_cube(V3(w, h, d): V3) -> Self {
        Self {
            vertices: vec![
                V3(0.0, 0.0, 0.0),             //  0 front top     left
                V3(0.0 + w, 0.0, 0.0),         //  1 front top     right
                V3(0.0, 0.0 + h, 0.0),         //  2 front bottom  left
                V3(0.0 + w, 0.0 + h, 0.0),     //  3 front bottom  right
                V3(0.0, 0.0, 0.0 + d),         //  4 back  top     left
                V3(0.0 + w, 0.0, 0.0 + d),     //  5 back  top     right
                V3(0.0, 0.0 + h, 0.0 + d),     //  6 back  bottom  left
                V3(0.0 + w, 0.0 + h, 0.0 + d), //  7 back  bottom  right
            ],
            fragments: vec![
                // back
                (0, 1, 2),
                (3, 2, 1),
                // bottom
                (4, 5, 0),
                (1, 0, 5),
                // left
                (4, 0, 6),
                (2, 6, 0),
                // front
                (5, 4, 7),
                (6, 7, 4),
                // top
                (7, 6, 3),
                (2, 3, 6),
                // right
                (1, 5, 3),
                (7, 3, 5),
            ],
        }
    }

    pub fn points<'a>(&'a self) -> impl Iterator<Item = V3> + 'a {
        self.vertices.iter().cloned()
    }

    pub fn vertices<'a>(&'a self) -> impl Iterator<Item = Vertex> + 'a {
        let verts: &[V3] = &self.vertices;
        self.fragments
            .iter()
            .map(|(a, b, c)| Vertex(verts[*a], verts[*b], verts[*c]))
    }
}
