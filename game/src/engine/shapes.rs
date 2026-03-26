use crate::engine::{math::V3, Triangle3, V2};

static CUBE_VERTICES: [(i8, i8, i8); 8] = [
    (0, 1, 0), //  0 front top     left
    (1, 1, 0), //  1 front top     right
    (0, 0, 0), //  2 front bottom  left
    (1, 0, 0), //  3 front bottom  right
    (0, 1, 1), //  4 back  top     left
    (1, 1, 1), //  5 back  top     right
    (0, 0, 1), //  6 back  bottom  left
    (1, 0, 1), //  7 back  bottom  right
];

static CUBE_EDGES: [(usize, usize); 12] = [
    (0, 1), // ftl -> ftr
    (2, 3), // fbl -> fbr
    (4, 5), // btl -> btr
    (6, 7), // bbl -> bbr
    (0, 2), // ftl -> fbl
    (1, 3), // ftr -> fbr
    (4, 6), // btl -> bbl
    (5, 7), // btr -> bbr
    (0, 4), // ftl -> btl
    (1, 5), // ftr -> btr
    (2, 6), // fbl -> bbl
    (3, 7), // fbr -> bbr
];

static CUBE_FACES: [(usize, usize, usize); 12] = [
    // front
    (0, 1, 2),
    (3, 2, 1),
    // top
    (0, 4, 1),
    (5, 1, 4),
    // right
    (7, 3, 5),
    (1, 5, 3),
    // back
    (5, 4, 7),
    (6, 7, 4),
    // bottom
    (3, 7, 2),
    (6, 2, 7),
    // left
    (4, 0, 6),
    (2, 6, 0),
];

static PLANE_VERTICES: [(i8, i8, i8); 4] = [
    //
    (0, 0, 0),
    (0, 0, 1),
    (1, 0, 0),
    (1, 0, 1),
];
static PLANE_EDGES: [(usize, usize); 5] = [
    //
    (0, 1),
    (1, 2),
    (2, 3),
    (0, 2),
    (1, 3),
];
static PLANE_FACES: [(usize, usize, usize); 2] = [
    //
    (0, 1, 2),
    (3, 2, 1),
];

pub struct Shape {
    vertices: Vec<V3>,
    edges: Vec<(usize, usize)>,
    faces: Vec<(usize, usize, usize)>,
}

impl Shape {
    pub fn new_plane(dim: V3) -> Self {
        Self {
            vertices: PLANE_VERTICES
                .iter()
                .map(|p| scale_i8_vertex(*p, &dim))
                .collect(),
            edges: Vec::from_iter(PLANE_EDGES),
            faces: Vec::from_iter(PLANE_FACES),
        }
    }

    pub fn new_cube(dim: V3) -> Self {
        Self {
            vertices: CUBE_VERTICES
                .iter()
                .map(|p| scale_i8_vertex(*p, &dim))
                .collect(),
            edges: Vec::from_iter(CUBE_EDGES),
            faces: Vec::from_iter(CUBE_FACES),
        }
    }

    pub fn vertices<'a>(&'a self) -> impl Iterator<Item = V3> + 'a {
        self.vertices.iter().cloned()
    }

    pub fn faces<'a>(&'a self) -> impl Iterator<Item = Triangle3> + 'a {
        let verts: &[V3] = &self.vertices;
        self.faces
            .iter()
            .map(|(a, b, c)| Triangle3(verts[*a], verts[*b], verts[*c]))
    }
}

fn scale_i8_vertex(p: (i8, i8, i8), s: &V3) -> V3 {
    V3(p.0 as f64 * s.0, p.1 as f64 * s.1, p.2 as f64 * s.2)
}
