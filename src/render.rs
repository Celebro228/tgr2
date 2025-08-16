use miniquad::*;
use crate::cross::*;
use crate::draw::*;
use crate::shader;


pub(crate) struct Render {
    ctx: Ctx,
    pipeline: Pipeline,
}

impl Render {
    pub(crate) fn new() -> Self {
        let mut ctx = Ctx::new();
        let shader = ctx.shader_new();
        let pipeline = ctx.pipeline_new(shader);
        Self {
            ctx,
            pipeline
        }
    }

    pub(crate) fn pre_update(&mut self) {
        self.ctx.frame(&self.pipeline);
        self.ctx.mvp_2d();
    }

    pub(crate) fn post_update(&mut self) {
        self.ctx.commit();
    }
    /*pub(crate) fn draw() {

        for draw in app.draw() {
            
            let vertex_buffer = self.ctx.new_buffer(
                BufferType::VertexBuffer,
                BufferUsage::Immutable,
                BufferSource::slice(draw.0),
            );
            let index_buffer = self.ctx.new_buffer(
                BufferType::IndexBuffer,
                BufferUsage::Immutable,
                BufferSource::slice(draw.1),
            );
            let bindings = Bindings {
                vertex_buffers: vec![vertex_buffer],
                index_buffer: index_buffer,
                images: vec![],
            };
            self.ctx.apply_bindings(&bindings);
            self.ctx.draw(0, draw.1.len() as i32, 1);
        }
    }*/
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

    fn mvp_2d(&mut self) {
        let canvas = vec2(50., 50.);
        let window = window::screen_size();
        let window = vec2(window.0, window.1) / 2.;
        let aspect_window = window.x / window.y;
        let aspect_canvas = canvas.x / canvas.y;
        let scale = canvas.x / (aspect_canvas / aspect_window);
        let mvp = Mat4::orthographic_rh_gl(
            -scale,
            scale,
            -canvas.y,
            canvas.y,
            -1.,
            1.
        );
        self.render_backend
            .apply_uniforms(UniformsSource::table(&shader::Uniforms {
                mvp,
            }));
    }

    fn mvp_3d(&mut self) {
        let (width, height) = window::screen_size();
        let mvp = Mat4::perspective_rh_gl(
            60.0f32.to_radians(),
            width / height,
            0.01, 10.0
        );
        self.render_backend
            .apply_uniforms(UniformsSource::table(&shader::Uniforms {
                mvp,
            }));
    }

    fn frame(&mut self, pipeline: &Pipeline) {
        self.render_backend.begin_default_pass(Default::default());
        self.render_backend.apply_pipeline(pipeline);
    }

    fn commit(&mut self) {
        self.render_backend.end_render_pass();
        self.render_backend.commit_frame();
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
                VertexAttribute::new("in_color", VertexFormat::Float4),
            ],
            shader,
            PipelineParams {
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

    pub(crate) fn bindings_new(&mut self, verts: &Vec<Vertex>, indis: &Vec<u16>) {
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
        let bindings = Bindings {
            vertex_buffers: vec![vertex_buffer],
            index_buffer: index_buffer,
            images: vec![],
        };
    }
}