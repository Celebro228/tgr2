use glam::vec3;

use crate::object2d::Object2d;
use crate::draw::{Vertex, Color};


pub fn rect(w: f32, h: f32) -> Object2d {
    let verts = vec![
        Vertex::new(vec3(-w, -h, 0.), Color::new(1., 0., 0., 1.)),
        Vertex::new(vec3(w, -h, 0.), Color::new(0., 1., 0., 1.)),
        Vertex::new(vec3(w, h, 0.), Color::new(0., 1., 1., 1.)),
        Vertex::new(vec3(-w, h, 0.), Color::new(0., 0., 1., 1.)),
    ];

    let indis = vec![0, 1, 2, 2, 3, 0];

    Object2d::new(verts, indis)
}