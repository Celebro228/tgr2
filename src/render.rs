use miniquad::*;
use crate::cross::*;
use crate::draw::*;
use crate::shader;
pub(crate) use miniquad::Bindings;


pub(crate) struct Render {
    pub(crate) ctx: Ctx,
    pipeline: Pipeline,
}

impl Render {
    pub(crate) fn new() -> Self {
        let mut ctx = Ctx::new();
        let shader = ctx.shader_new();
        let pipeline = ctx.pipeline_new(shader);
        Self {
            ctx,
            pipeline,
        }
    }

    pub(crate) fn pre_update(&mut self) {
        self.ctx.frame(&self.pipeline);
    }

    pub(crate) fn post_update(&mut self) {
        self.ctx.commit();
    }
}


pub struct Ctx {
    render_backend: Box<dyn RenderingBackend>
}
impl Ctx {
    fn new() -> Self {
        Self {
            render_backend: window::new_rendering_backend(),
        }
    }

    fn frame(&mut self, pipeline: &Pipeline) {
        self.render_backend.begin_default_pass(Default::default());
        self.render_backend.apply_pipeline(pipeline);
    }

    fn commit(&mut self) {
        self.render_backend.end_render_pass();
        self.render_backend.commit_frame();
    }

    pub(crate) fn draw(&mut self, bindings: &Bindings, indis_len: i32) {
        self.render_backend.apply_bindings(bindings);
        self.render_backend.draw(0, indis_len, 1);
    }

    pub(crate) fn apply_mvp(&mut self, mvp: Mat4, color: &Color) {
        self.render_backend
            .apply_uniforms(UniformsSource::table(&shader::Uniforms {
                mvp,
                color: *color,
            }));
    }

    pub(crate) fn shader_new(&mut self) -> ShaderId {
        self.render_backend.new_shader(
            ShaderSource::Glsl {
                vertex: shader::VERTEX,
                fragment: shader::FRAGMENT,
            },
            shader::meta(),
        )
        .expect("Error to load shaders")
    }

    pub(crate) fn pipeline_new(&mut self, shader: ShaderId) -> Pipeline {
        self.render_backend.new_pipeline(
            &[BufferLayout::default()],
            &[
                VertexAttribute::new("in_pos", VertexFormat::Float3),
            ],
            shader,
            PipelineParams {
                cull_face: CullFace::Back,
                depth_write: true,
                depth_test: Comparison::LessOrEqual,
                color_blend: Some(BlendState::new(
                    Equation::Add,
                    BlendFactor::Value(BlendValue::SourceAlpha),
                    BlendFactor::OneMinusValue(BlendValue::SourceAlpha),
                )),
                alpha_blend: Some(BlendState::new(
                    Equation::Add,
                    BlendFactor::Zero,
                    BlendFactor::One,
                )),
                ..Default::default()
            },
        )
    }

    pub(crate) fn bindings_new(&mut self, verts: &Vec<Vec3>, indis: &Vec<u16>) -> Bindings {
        let vertex_buffer = self.render_backend.new_buffer(
            BufferType::VertexBuffer,
            BufferUsage::Immutable,
            BufferSource::slice(verts),
        );
        let index_buffer = self.render_backend.new_buffer(
            BufferType::IndexBuffer,
            BufferUsage::Immutable,
            BufferSource::slice(indis),
        );
        Bindings {
            vertex_buffers: vec![vertex_buffer],
            index_buffer: index_buffer,
            images: vec![],
        }
    }
}