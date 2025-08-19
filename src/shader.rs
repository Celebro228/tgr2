use miniquad::{ShaderMeta, UniformBlockLayout, UniformDesc, UniformType};
use crate::cross::*;
use crate::draw::Color;


pub(crate) const VERTEX: &str = r#"#version 100
attribute vec3 in_pos;

uniform mat4 mvp;

void main() {
    gl_Position = mvp * vec4(in_pos, 1);
}"#;

pub(crate) const FRAGMENT: &str = r#"#version 100
uniform lowp vec4 color;

void main() {
    gl_FragColor = color;
}"#;

pub(crate) fn meta() -> ShaderMeta {
    ShaderMeta {
        images: vec![],
        uniforms: UniformBlockLayout {
            uniforms: vec![
                UniformDesc::new("mvp", UniformType::Mat4),
                UniformDesc::new("color", UniformType::Float4),
            ],
        },
    }
}

#[repr(C)]
pub(crate) struct Uniforms {
    pub(crate) mvp: Mat4,
    pub(crate) color: Color,
}
