use miniquad::{ShaderMeta, UniformBlockLayout, UniformDesc, UniformType};
use glam::Mat4;


pub(crate) const VERTEX: &str = r#"#version 100
attribute vec2 in_pos;
attribute vec4 in_color;

varying lowp vec4 color;

uniform mat4 mvp;

void main() {
    gl_Position = mvp * vec4(in_pos, 0, 1);
    color = in_color;
}"#;

pub(crate) const FRAGMENT: &str = r#"#version 100
varying lowp vec4 color;

void main() {
    gl_FragColor = color;
}"#;


pub(crate) fn meta() -> ShaderMeta {
    ShaderMeta {
        images: vec![],
        uniforms: UniformBlockLayout {
            uniforms: vec![UniformDesc::new("mvp", UniformType::Mat4)],
        },
    }
}

 #[repr(C)]
pub(crate) struct Uniforms {
    pub mvp: Mat4,
}