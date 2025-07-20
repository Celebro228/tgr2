use miniquad::{ShaderMeta, UniformBlockLayout, UniformDesc, UniformType};


pub(crate) const VERTEX: &str = r#"#version 100
attribute vec3 in_pos;

void main() {
    gl_Position = vec4(in_pos, 1);
}"#;

pub(crate) const FRAGMENT: &str = r#"#version 100
uniform float iTime;
uniform vec2 iResolution;

void main() {
    vec2 fragCoord = gl_FragCoord.xy;

    vec3 c = vec3(0.0);
    float l;
    float z = iTime;

    for(int i = 0; i < 3; i++) {
        vec2 uv;
        vec2 p = fragCoord / iResolution.xy;
        uv = p;
        p -= 0.5;
        p.x *= iResolution.x / iResolution.y;

        z += 0.07;
        l = length(p);
        uv += p / l * (sin(z) + 1.0) * abs(sin(l * 9.0 - z - z));
        c[i] = 0.01 / length(mod(uv, 1.0) - 0.5);
    }

    float len = length(c);
    gl_FragColor = vec4(c / len, iTime);
}"#;


pub(crate) fn meta() -> ShaderMeta {
    ShaderMeta {
        images: vec![],
        uniforms: UniformBlockLayout {
            uniforms: vec![
                UniformDesc::new("iTime", UniformType::Float1),
                UniformDesc::new("iResolution", UniformType::Float2),
            ],
        },
    }
}

#[repr(C)]
pub(crate) struct Uniforms {
    pub(crate) iTime: f32,
    pub(crate) iResolution: (f32, f32),
}