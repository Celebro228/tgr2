use miniquad::{ShaderMeta, UniformBlockLayout, UniformDesc, UniformType};


pub(crate) const VERTEX: &str = r#"#version 100
attribute vec2 in_pos;
attribute vec4 in_color;

varying lowp vec4 color;

void main() {
    gl_Position = vec4(in_pos, 0, 1);
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
            uniforms: vec![],
        },
    }
}